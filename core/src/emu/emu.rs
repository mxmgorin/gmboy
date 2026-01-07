use crate::apu::Apu;
use crate::auxiliary::clock::Clock;
use crate::auxiliary::io::Io;
use crate::auxiliary::joypad::Joypad;
use crate::bus::Bus;
use crate::cart::Cart;
use crate::cpu::Cpu;
use crate::emu::config::EmuConfig;
use crate::emu::runtime::{EmuRuntime, RunMode};
use crate::emu::state::{EmuSaveState, EmuState};
use crate::ppu::framebuffer::FrameBuffer;
use crate::ppu::lcd::Lcd;
use crate::ppu::Ppu;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use std::{mem, thread};

const CYCLES_PER_SECOND: usize = 4_194_304;
const NANOS_PER_SECOND: usize = 1_000_000_000;
const T_CYCLE_DURATION_NS: f64 = NANOS_PER_SECOND as f64 / CYCLES_PER_SECOND as f64;

pub trait EmuAudioCallback {
    fn update(&mut self, output: &[f32], runtime: &EmuRuntime);
}

pub struct Emu {
    pub config: EmuConfig,
    pub state: EmuState,
    pub runtime: EmuRuntime,
    prev_speed_multiplier: f64,
    rewind_buffer: VecDeque<EmuSaveState>,
    last_rewind_frame: usize,
}

impl Emu {
    pub fn new(config: EmuConfig, runtime: EmuRuntime) -> Result<Self, String> {
        Ok(Self {
            runtime,
            prev_speed_multiplier: config.normal_speed,
            state: EmuState::Running,
            rewind_buffer: VecDeque::with_capacity(config.rewind_size),
            last_rewind_frame: 0,
            config,
        })
    }

    #[inline(always)]
    pub fn get_framebuffer(&mut self) -> &mut FrameBuffer {
        &mut self.runtime.cpu.clock.bus.io.ppu.buffer
    }

    /// Runs emulation for one frame. Return whether the emulation is on time.
    #[inline(always)]
    pub fn run_frame(&mut self, callback: &mut impl EmuAudioCallback) -> bool {
        match self.state {
            EmuState::Rewind => {
                if let Some(state) = self.rewind_buffer.pop_back() {
                    self.load_save_state(state);
                    thread::sleep(Duration::from_millis(250));
                }

                self.runtime.run_frame(callback);
            }
            EmuState::Running => {
                self.runtime.run_frame(callback);
                self.push_rewind();
            }
        };

        let real_elapsed = self.runtime.cpu.clock.time.elapsed();
        let emulated_time = self.calc_emulated_time();
        let on_time = emulated_time >= real_elapsed;

        if on_time {
            self.sleep_spin(emulated_time - real_elapsed);
        }

        on_time
    }

    #[inline(always)]
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

    #[inline(always)]
    fn calc_emulated_time(&mut self) -> Duration {
        let speed_multiplier = match self.runtime.mode {
            RunMode::Normal => self.config.normal_speed,
            RunMode::Slow => self.config.slow_speed,
            RunMode::Turbo => self.config.turbo_speed,
        };

        if self.prev_speed_multiplier != speed_multiplier {
            self.runtime.cpu.clock.reset();
        }

        self.prev_speed_multiplier = speed_multiplier;

        let emulated_duration_ns =
            (self.runtime.cpu.clock.get_t_cycles() as f64 * T_CYCLE_DURATION_NS / speed_multiplier)
                .round() as u64;

        Duration::from_nanos(emulated_duration_ns)
    }

    #[inline(always)]
    pub fn create_save_state(&self) -> EmuSaveState {
        EmuSaveState {
            cpu: self.runtime.cpu.clone(),
            cart_save_state: self.runtime.cpu.clock.bus.cart.create_save_state(),
        }
    }

    #[inline(always)]
    fn push_rewind(&mut self) {
        if self.config.rewind_size > 0 {
            let curr_frame = self.runtime.cpu.clock.bus.io.ppu.current_frame;
            let diff =  curr_frame.saturating_sub(self.last_rewind_frame);

            if diff >= self.config.rewind_frames {
                if self.rewind_buffer.len() > self.config.rewind_size {
                    self.rewind_buffer.pop_front();
                }

                self.rewind_buffer.push_back(self.create_save_state());
                self.last_rewind_frame = curr_frame;
            }
        }
    }

    pub fn load_cart(&mut self, cart: Cart) {
        let lcd = Lcd::new(self.runtime.cpu.clock.bus.io.ppu.lcd.current_colors);
        let ppu = Ppu::new(lcd);
        let apu = Apu::new(self.runtime.cpu.clock.bus.io.apu.config.clone());
        let io = Io::new(ppu, apu);
        let bus = Bus::new(cart, io);
        let clock = Clock::new(bus);
        self.runtime.cpu = Cpu::new(clock);

        self.state = EmuState::Running;
        self.runtime.cpu.clock.reset();
        self.rewind_buffer.clear();
    }

    pub fn load_save_state(&mut self, mut save_state: EmuSaveState) {
        // reconstruct cart
        mem::swap(
            &mut save_state.cpu.clock.bus.cart.data,
            &mut self.runtime.cpu.clock.bus.cart.data,
        );
        let cart = save_state
            .cart_save_state
            .into_cart(save_state.cpu.clock.bus.cart.data);
        save_state.cpu.clock.bus.cart = cart;
        self.runtime.cpu = save_state.cpu;
        self.runtime.cpu.clock.bus.io.joypad = Joypad::default(); // reset controls
        self.runtime.cpu.clock.reset();
    }
}
