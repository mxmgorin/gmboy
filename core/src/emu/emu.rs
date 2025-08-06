use crate::apu::Apu;
use crate::auxiliary::io::Io;
use crate::auxiliary::joypad::Joypad;
use crate::bus::Bus;
use crate::cart::Cart;
use crate::cpu::Cpu;
use crate::emu::config::EmuConfig;
use crate::emu::runtime::{EmuRuntime, RunMode};
use crate::emu::state::{EmuSaveState, EmuState};
use crate::ppu::lcd::Lcd;
use crate::read_bytes;
use std::collections::VecDeque;
use std::path::Path;
use std::time::{Duration, Instant};
use std::{mem, thread};

const CYCLES_PER_SECOND: usize = 4_194_304;
const NANOS_PER_SECOND: usize = 1_000_000_000;
const CYCLE_DURATION_NS: f64 = NANOS_PER_SECOND as f64 / CYCLES_PER_SECOND as f64;

pub trait EmuAudioCallback {
    fn update(&mut self, output: &[f32], runtime: &EmuRuntime);
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
    pub fn new(config: EmuConfig, runtime: EmuRuntime) -> Result<Self, String> {
        Ok(Self {
            cpu: Cpu::default(),
            runtime,
            prev_speed_multiplier: config.normal_speed,
            state: EmuState::Running,
            rewind_buffer: VecDeque::with_capacity(config.rewind_size),
            last_rewind_save_time: Instant::now(),
            config,
        })
    }

    /// Runs emulation for one frame. Return false when paused.
    pub fn run_frame(&mut self, callback: &mut impl EmuAudioCallback) -> Result<(), String> {
        match self.state {
            EmuState::Rewind => {
                if let Some(state) = self.rewind_buffer.pop_back() {
                    self.load_save_state(state);
                    thread::sleep(Duration::from_millis(250));
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

    pub fn load_cart_file(&mut self, path: &Path, ram_bytes: Option<Box<[u8]>>) -> Result<(), String> {
        let cart = read_cart_file(path, ram_bytes).map_err(|e| e.to_string());

        let Ok(cart) = cart else {
            let msg = format!("Failed read_cart: {}", cart.unwrap_err());
            return Err(msg);
        };

        let apu = Apu::new(self.runtime.bus.io.apu.config.clone());
        let lcd = Lcd::new(self.runtime.bus.io.lcd.current_colors);
        let io = Io::new(lcd, apu);
        self.runtime.bus = Bus::new(cart, io);
        self.cpu = Cpu::default();
        self.state = EmuState::Running;
        self.runtime.clock.reset();
        self.rewind_buffer.clear();

        Ok(())
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

pub fn read_cart_file(path: &Path, ram_bytes: Option<Box<[u8]>>) -> Result<Cart, String> {
    let bytes = read_bytes(path).map_err(|e| e.to_string())?;
    let mut cart = Cart::new(bytes).map_err(|e| e.to_string())?;
    _ = print_cart(&cart).map_err(|e| eprintln!("Failed print_cart: {e}"));

    if let Some(ram_bytes) = ram_bytes {
        cart.load_ram(ram_bytes);
    }

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
