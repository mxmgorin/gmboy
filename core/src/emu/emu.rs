use crate::auxiliary::clock::sleep_spin;
use crate::auxiliary::io::Io;
use crate::auxiliary::joypad::Joypad;
use crate::bus::Bus;
use crate::cart::Cart;
use crate::cpu::Cpu;
use crate::debugger::Debugger;
use crate::emu::battery::BatterySave;
use crate::emu::config::EmuConfig;
use crate::emu::runtime::EmuRuntime;
use crate::emu::state::{EmuSaveState, EmuState, RunMode};
use crate::ppu::lcd::Lcd;
use crate::ppu::tile::{Pixel, PixelColor};
use std::collections::VecDeque;
use std::path::Path;
use std::time::{Duration, Instant};
use std::{fs, mem, thread};

const CYCLES_PER_SECOND: usize = 4_194_304;
const NANOS_PER_SECOND: usize = 1_000_000_000;
const CYCLE_TIME: f64 = NANOS_PER_SECOND as f64 / CYCLES_PER_SECOND as f64;

pub trait EmuCallback {
    fn update_video(&mut self, buffer: &[Pixel], fps: usize);
    fn update_audio(&mut self, output: &[f32]);
}

pub struct Emu {
    pub config: EmuConfig,
    pub state: EmuState,
    pub cpu: Cpu,
    pub runtime: EmuRuntime,
    speed_multiplier: f64,
    last_fps_timestamp: Duration,
    rewind_buffer: VecDeque<EmuSaveState>,
    last_rewind_save: Instant,
}

impl Emu {
    pub fn new(
        config: EmuConfig,
        pallet: [PixelColor; 4],
        debugger: Option<Debugger>,
    ) -> Result<Self, String> {
        let lcd = Lcd::new(pallet);
        let bus = Bus::with_bytes(vec![], Io::new(lcd));

        Ok(Self {
            cpu: Cpu::default(),
            runtime: EmuRuntime::new(debugger, bus),
            speed_multiplier: 1.0,
            state: EmuState::Paused,
            config,
            last_fps_timestamp: Default::default(),
            rewind_buffer: Default::default(),
            last_rewind_save: Instant::now(),
        })
    }

    /// Runs emulation for one frame. Return false when paused.
    pub fn run_frame(&mut self, callback: &mut impl EmuCallback) -> Result<bool, String> {
        let mode = match self.state {
            EmuState::Paused => return Ok(false),
            EmuState::Rewind => {
                if let Some(state) = self.rewind_buffer.pop_back() {
                    self.load_save_state(state);
                    thread::sleep(Duration::from_millis(100));
                }

                self.runtime.run_frame(&mut self.cpu, RunMode::Normal, true, callback)?;

                RunMode::Normal
            }
            EmuState::Running(mode) => {
                self.runtime.run_frame(&mut self.cpu, mode, self.config.is_muted, callback)?;

                mode
            },
        };

        let real_elapsed = self.runtime.clock.start_time.elapsed();
        let emulated_time = self.calc_emulated_time(mode);

        if emulated_time > real_elapsed {
            sleep_spin(emulated_time - real_elapsed);
        }

        Ok(true)
    }

    fn calc_emulated_time(&mut self, mode: RunMode) -> Duration {
        let speed_multiplier = match mode {
            RunMode::Normal => 1.0,
            RunMode::Slow => self.config.slow_speed / 100.0,
            RunMode::Turbo => self.config.turbo_speed / 100.0,
        };

        if self.speed_multiplier != speed_multiplier {
            self.runtime.clock.reset();
        }

        self.speed_multiplier = speed_multiplier;

        let emulated_time_ns =
            (self.runtime.clock.t_cycles as f64 * CYCLE_TIME / speed_multiplier).round() as u64;

        Duration::from_nanos(emulated_time_ns)
    }

    pub fn create_save_state(&self, cpu: &Cpu) -> EmuSaveState {
        EmuSaveState {
            cpu: cpu.clone(),
            bus_without_cart: self.runtime.bus.clone_empty_cart(),
            cart_save_state: self.runtime.bus.cart.create_save_state(),
        }
    }

    pub fn save_files(&self, cart_file_path: &Path) -> Result<(), String> {
        let name = cart_file_path.file_stem().unwrap().to_str();

        let Some(name) = name else {
            return Err(format!("Invalid cart_file_path: {cart_file_path:?}"));
        };

        if let Some(bytes) = self.runtime.bus.cart.dump_ram() {
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

        if self.config.rewind_size > 0
            && now.duration_since(self.last_rewind_save).as_secs_f32() >= 2.0
        {
            if self.rewind_buffer.len() > self.config.rewind_size {
                self.rewind_buffer.pop_front();
            }

            self.rewind_buffer
                .push_back(self.create_save_state(&self.cpu));
            self.last_rewind_save = now;
        }
    }

    pub fn load_cart_file(&mut self, path: &Path, save_state: bool) {
        let cart = read_cart_file(path).map_err(|e| e.to_string());

        let Ok(cart) = cart else {
            eprintln!("Failed read_cart: {}", cart.unwrap_err());
            return;
        };

        let io = Io::new(Lcd::new(self.runtime.bus.io.lcd.current_pallet));
        self.runtime.bus = Bus::new(cart, io);
        self.cpu = Cpu::default();
        self.state = EmuState::Running(RunMode::Normal);
        self.reset();
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

        self.reset();
    }

    pub fn reset(&mut self) {
        self.runtime.reset();
        self.last_fps_timestamp = Default::default();
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
