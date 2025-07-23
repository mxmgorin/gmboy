use crate::auxiliary::io::Io;
use crate::auxiliary::joypad::Joypad;
use crate::bus::Bus;
use crate::cart::Cart;
use crate::cpu::Cpu;
use crate::debugger::Debugger;
use crate::emu::battery::BatterySave;
use crate::emu::config::{EmuConfig};
use crate::emu::runtime::{EmuRuntime, RunMode};
use crate::emu::state::{EmuSaveState, EmuState};
use crate::ppu::lcd::Lcd;
use crate::ppu::tile::PixelColor;
use std::collections::VecDeque;
use std::path::Path;
use std::time::{Duration, Instant};
use std::{fs, mem, thread};
use crate::apu::{Apu, ApuConfig};

const CYCLES_PER_SECOND: usize = 4_194_304;
const NANOS_PER_SECOND: usize = 1_000_000_000;
const CYCLE_DURATION_NS: f64 = NANOS_PER_SECOND as f64 / CYCLES_PER_SECOND as f64;

pub trait EmuCallback {
    fn update_video(&mut self, buffer: &[u32], runtime: &EmuRuntime);
    fn update_audio(&mut self, output: &[f32], runtime: &EmuRuntime);
    fn paused(&mut self);
}

pub struct Emu {
    pub config: EmuConfig,
    pub state: EmuState,
    pub runtime: EmuRuntime,
    cpu: Cpu,
    prev_speed_multiplier: f64,
    rewind_buffer: VecDeque<EmuSaveState>,
    last_rewind_save_time: Instant,
}

impl Emu {
    pub fn new(
        config: EmuConfig,
        pallet: [PixelColor; 4],
        apu_config: ApuConfig,
        debugger: Option<Debugger>,
    ) -> Result<Self, String> {
        let lcd = Lcd::new(pallet);
        let io = Io::new(lcd, Apu::new(apu_config));
        let bus = Bus::with_bytes(vec![], io);

        Ok(Self {
            cpu: Cpu::default(),
            runtime: EmuRuntime::new(debugger, bus),
            prev_speed_multiplier: config.normal_speed,
            state: EmuState::Paused,
            rewind_buffer: VecDeque::with_capacity(config.rewind_size),
            last_rewind_save_time: Instant::now(),
            config,
        })
    }

    /// Runs emulation for one frame. Return false when paused.
    pub fn run_frame(&mut self, callback: &mut impl EmuCallback) -> Result<(), String> {
        match self.state {
            EmuState::Paused => {
                thread::sleep(Duration::from_millis(100));
                self.runtime.clock.reset();
                callback.paused();

                return Ok(());
            }
            EmuState::Rewind => {
                if let Some(state) = self.rewind_buffer.pop_back() {
                    self.load_save_state(state);
                    thread::sleep(Duration::from_millis(333));
                }

                self.runtime.run_frame(&mut self.cpu, callback)?;
            }
            EmuState::Running => {
                self.runtime.run_frame(&mut self.cpu, callback)?;
                self.push_rewind();
            }
        };

        let real_elapsed = self.runtime.clock.time.elapsed();
        let emulated_time = self.calc_emulated_time();

        if emulated_time > real_elapsed {
            self.sleep_spin(emulated_time - real_elapsed);
        }

        Ok(())
    }

    fn sleep_spin(&self, duration: Duration) {
        let start = Instant::now();

        // Sleep to avoid overshooting
        if duration > self.config.spin_duration {
            thread::sleep(duration - self.config.spin_duration);
        }

        // Spin the rest to get close to the target duration
        while start.elapsed() < duration {
            std::hint::spin_loop();
            //thread::yield_now();
        }
    }

    fn calc_emulated_time(&mut self) -> Duration {
        let speed_multiplier = match self.runtime.mode {
            RunMode::Normal => self.config.normal_speed,
            RunMode::Slow => self.config.slow_speed,
            RunMode::Turbo => self.config.turbo_speed,
        };

        if self.prev_speed_multiplier != speed_multiplier {
            self.runtime.clock.reset();
        }

        self.prev_speed_multiplier = speed_multiplier;

        let emulated_duration_ns = (self.runtime.clock.t_cycles as f64 * CYCLE_DURATION_NS
            / speed_multiplier)
            .round() as u64;

        Duration::from_nanos(emulated_duration_ns)
    }

    pub fn create_save_state(&self) -> EmuSaveState {
        EmuSaveState {
            cpu: self.cpu.clone(),
            bus_without_cart: self.runtime.bus.clone_empty_cart(),
            cart_save_state: self.runtime.bus.cart.create_save_state(),
        }
    }

    fn push_rewind(&mut self) {
        if self.config.rewind_size > 0 {
            let now = Instant::now();
            let duration = now.duration_since(self.last_rewind_save_time);

            if duration >= self.config.rewind_interval {
                if self.rewind_buffer.len() > self.config.rewind_size {
                    self.rewind_buffer.pop_front();
                }

                self.rewind_buffer.push_back(self.create_save_state());
                self.last_rewind_save_time = now;
            }
        }
    }

    pub fn load_cart_file(&mut self, path: &Path, save_state: bool) {
        let cart = read_cart_file(path).map_err(|e| e.to_string());

        let Ok(cart) = cart else {
            eprintln!("Failed read_cart: {}", cart.unwrap_err());
            return;
        };

        self.runtime.bus.load_cart(cart);
        self.cpu = Cpu::default();
        self.state = EmuState::Running;
        self.runtime.clock.reset();
        self.rewind_buffer.clear();

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
        // reconstruct cart
        let mut bus = save_state.bus_without_cart;
        mem::swap(&mut bus.cart.data, &mut self.runtime.bus.cart.data);
        let cart = save_state.cart_save_state.into_cart(bus.cart.data);
        bus.cart = cart;

        self.runtime.bus = bus;
        self.runtime.bus.io.joypad = Joypad::default(); // reset controls

        self.cpu = save_state.cpu;
        self.runtime.clock.reset();
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
