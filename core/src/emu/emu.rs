use crate::auxiliary::clock::spin_wait;
use crate::auxiliary::joypad::Joypad;
use crate::bus::Bus;
use crate::cart::Cart;
use crate::cpu::Cpu;
use crate::emu::battery::BatterySave;
use crate::emu::config::EmuConfig;
use crate::emu::ctx::{EmuCtx, EmuState, RunMode};
use crate::emu::save_state::EmuSaveState;
use crate::into_pallet;
use crate::ppu::tile::Pixel;
use crate::ppu::CYCLES_PER_FRAME;
use std::path::Path;
use std::time::{Duration, Instant};
use std::{fs, thread};

const CYCLES_PER_SECOND: usize = 4_194_304;
const NANOS_PER_SECOND: usize = 1_000_000_000;
const CYCLE_TIME: f64 = NANOS_PER_SECOND as f64 / CYCLES_PER_SECOND as f64;

pub struct Emu {
    pub ctx: EmuCtx,
    pub cpu: Cpu,
}

pub trait EmuCallback {
    fn update_video(&mut self, buffer: &[Pixel], fps: usize);
    fn update_audio(&mut self, output: &[f32]);
}

impl Emu {
    pub fn new(config: EmuConfig) -> Result<Self, String> {
        let mut bus = Bus::with_bytes(vec![]);
        bus.io.lcd.set_pallet(into_pallet(&config.pallet));

        Ok(Self {
            cpu: Cpu::new(bus),
            ctx: EmuCtx::new(config),
        })
    }

    /// Runs emulation for one frame. Return false when paused.
    pub fn run_frame(&mut self, callback: &mut impl EmuCallback) -> Result<bool, String> {
        match self.ctx.state {
            EmuState::Paused => Ok(false),
            EmuState::Rewind => {
                if let Some(state) = self.ctx.rewind_buffer.pop_back() {
                    self.load_save_state(state);
                    thread::sleep(Duration::from_millis(100));
                }

                self.emulate_frame(RunMode::Normal, callback)
            }
            EmuState::Running(mode) => self.emulate_frame(mode, callback),
        }
    }

    fn emulate_frame(
        &mut self,
        mode: RunMode,
        callback: &mut impl EmuCallback,
    ) -> Result<bool, String> {
        let prev_m_cycles = self.ctx.clock.get_m_cycles();

        while self.ctx.clock.get_m_cycles() - prev_m_cycles < CYCLES_PER_FRAME {
            self.cpu.step(&mut self.ctx)?;

            if let Some(debugger) = self.ctx.debugger.as_mut() {
                if !debugger.get_serial_msg().is_empty() {
                    println!("Serial: {}", debugger.get_serial_msg());
                }
            }

            if !self.ctx.config.is_muted
                && EmuState::Running(RunMode::Normal) == self.ctx.state
            {
                callback.update_audio(self.cpu.bus.io.apu.take_output());
            }
        }

        callback.update_video(&self.ctx.ppu.pipeline.buffer, self.ctx.ppu.fps);
        self.ctx.prev_frame = self.ctx.ppu.current_frame;
        let real_elapsed = self.ctx.clock.start_time.elapsed();
        let emulated_time = self.calc_emulated_time(mode);

        if emulated_time > real_elapsed {
            spin_wait(emulated_time - real_elapsed);
        }

        Ok(true)
    }

    fn calc_emulated_time(&mut self, mode: RunMode) -> Duration {
        let speed_multiplier = match mode {
            RunMode::Normal => 1.0,
            RunMode::Slow => self.ctx.config.slow_speed / 100.0,
            RunMode::Turbo => self.ctx.config.turbo_speed / 100.0,
        };

        if self.ctx.speed_multiplier != speed_multiplier {
            self.ctx.clock.reset();
        }

        self.ctx.speed_multiplier = speed_multiplier;

        let emulated_time_ns =
            (self.ctx.clock.t_cycles as f64 * CYCLE_TIME / speed_multiplier).round() as u64;

        Duration::from_nanos(emulated_time_ns)
    }

    pub fn create_save_state(&self, cpu: &Cpu) -> EmuSaveState {
        EmuSaveState {
            cpu_without_bus: cpu.clone_without_bus(),
            bus_without_cart: cpu.bus.clone_without_cart(),
            cart_mbc: cpu.bus.cart.mbc.clone(),
        }
    }

    pub fn save_files(self, cart_file_path: &Path) -> Result<(), String> {
        let name = cart_file_path.file_stem().unwrap().to_str();

        let Some(name) = name else {
            return Err(format!("Invalid cart_file_path: {cart_file_path:?}"));
        };

        if let Some(bytes) = self.cpu.bus.cart.dump_ram() {
            if let Err(err) = BatterySave::from_bytes(bytes)
                .save_file(name)
                .map_err(|e| e.to_string())
            {
                eprint!("Failed BatterySave: {err}");
            };
        }

        if let Err(err) = self.create_save_state(&self.cpu).save_file(name, 0) {
            eprintln!("Failed save_state: {err}");
        }

        Ok(())
    }

    pub fn push_rewind(&mut self) {
        let now = Instant::now();

        if self.ctx.config.rewind_size > 0
            && now.duration_since(self.ctx.last_rewind_save).as_secs_f32() >= 2.0
        {
            if self.ctx.rewind_buffer.len() > self.ctx.config.rewind_size {
                self.ctx.rewind_buffer.pop_front();
            }

            self.ctx
                .rewind_buffer
                .push_back(self.create_save_state(&self.cpu));
            self.ctx.last_rewind_save = now;
        }
    }

    pub fn load_cart_file(&mut self, path: &Path, save_state: bool) {
        let cart = read_cart_file(path).map_err(|e| e.to_string());

        let Ok(cart) = cart else {
            eprintln!("Failed read_cart: {}", cart.unwrap_err());
            return;
        };

        let current_pallet = self.cpu.bus.io.lcd.current_pallet;
        let mut bus = Bus::new(cart);
        bus.io.lcd.set_pallet(current_pallet);
        self.cpu = Cpu::new(bus);
        self.ctx.state = EmuState::Running(RunMode::Normal);
        self.ctx.reset();

        if save_state {
            let name = path.file_stem().unwrap().to_str().expect("cart is valid");
            let save_state = EmuSaveState::load_file(name, 0);

            if let Ok(save_state) = save_state {
                self.load_save_state(save_state);
            } else {
                eprintln!("Failed load save_state: {}", save_state.unwrap_err());
            };
        }
    }

    pub fn load_save_state(&mut self, save_state: EmuSaveState) {
        let mut state_cpu = save_state.cpu_without_bus; // reconstruct cpu
        state_cpu.bus = save_state.bus_without_cart;
        state_cpu.bus.io.joypad = Joypad::default(); // reset controls
        state_cpu.bus.cart.mbc = save_state.cart_mbc; // reconstruct cart
        state_cpu.bus.cart.data = self.cpu.bus.cart.data.clone();

        self.cpu = state_cpu;
        self.ctx.reset();
    }
}

pub fn read_cart_file(path: &Path) -> Result<Cart, String> {
    let bytes = read_bytes(path).map_err(|e| e.to_string())?;
    let mut cart = Cart::new(bytes).map_err(|e| e.to_string())?;
    _ = print_cart(&cart).map_err(|e| eprintln!("Failed print_cart: {e}"));
    let file_name = path.file_stem().expect("we read file").to_str().unwrap();

    let Ok(save) = BatterySave::load_file(file_name) else {
        return Ok(cart);
    };

    cart.load_ram(save.ram_bytes);

    Ok(cart)
}

fn print_cart(cart: &Cart) -> Result<(), String> {
    println!("Cart Loaded:");
    println!("\t Title          : {}", cart.data.get_title());
    println!("\t Type           : {:?}", cart.data.get_cart_type()?);
    println!("\t ROM Size       : {:?}", cart.data.get_rom_size()?);
    println!("\t ROM bytes      : {:?}", cart.data.bytes.len());
    println!("\t RAM Size       : {:?}", cart.data.get_ram_size()?);
    println!("\t ROM Version    : {:02X}", cart.data.get_rom_version());
    println!("\t Checksum Valid : {}", cart.data.checksum_valid());

    Ok(())
}

pub fn read_bytes(file_path: &Path) -> Result<Box<[u8]>, String> {
    if !file_path.exists() {
        return Err(format!("File not found: {file_path:?}"));
    }

    fs::read(file_path)
        .map(|x| x.into_boxed_slice())
        .map_err(|e| format!("Failed to read file: {e}"))
}
