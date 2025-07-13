use crate::auxiliary::clock::{spin_wait, Clock};
use crate::auxiliary::joypad::Joypad;
use crate::bus::Bus;
use crate::cart::Cart;
use crate::config::Config;
use crate::cpu::{Cpu, CpuCallback, DebugCtx};
use crate::debugger::{CpuLogType, Debugger};
use crate::mbc::MbcVariant;
use crate::ppu::Ppu;
use crate::ui::events::{UiEvent, UiEventHandler};
use crate::ui::Ui;
use crate::CYCLES_PER_FRAME;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::{env, fs, thread};

const _CYCLES_PER_SECOND: usize = 4_194_304;
const CYCLE_TIME: f64 = 238.4185791; // 1 / 4_194_304 seconds ≈ 238.41858 nanoseconds

#[derive(Serialize, Deserialize)]
pub struct EmuSaveState {
    pub cpu_without_bus: Cpu,
    pub bus_without_cart: Bus,
    pub cart_mbc: Option<MbcVariant>,
}

impl EmuSaveState {
    pub fn save(&self, game_name: &str, index: usize) -> std::io::Result<()> {
        let path = Self::generate_path(game_name, index);
        let encoded: Vec<u8> = bincode::serialize(self).expect("Failed to serialize state");
        let mut file = File::create(path)?;
        file.write_all(&encoded)?;

        Ok(())
    }

    pub fn load(game_name: &str, index: usize) -> std::io::Result<Self> {
        let path = Self::generate_path(game_name, index);
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let decoded = bincode::deserialize(&buffer).expect("Failed to deserialize state");
        Ok(decoded)
    }

    pub fn generate_path(game_name: &str, index: usize) -> PathBuf {
        let exe_path = env::current_exe().expect("Failed to get executable path");
        let exe_dir = exe_path
            .parent()
            .expect("Failed to get executable directory");

        exe_dir
            .join("save_states")
            .join(format!("{game_name}_{index}.state"))
    }
}

pub struct Emu {
    pub ctx: EmuCtx,
    pub cpu: Cpu,
    pub ui: Ui,
}

pub struct EmuCtx {
    pub ppu: Ppu,
    pub clock: Clock,
    pub debugger: Option<Debugger>,
    pub speed_multiplier: f64,
    pub state: EmuState,
    pub config: Config,
    pub prev_frame: usize,
    pub last_fps_timestamp: Duration,
    pub rewind_buffer: VecDeque<EmuSaveState>,
    pub last_rewind_save: Instant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmuState {
    WaitCart,
    Running(RunMode),
    Paused,
    LoadCart(String),
    Quit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RunMode {
    Normal,
    Slow,
    Turbo,
    Rewind,
}

impl EmuCtx {
    pub fn new(config: Config) -> EmuCtx {
        Self {
            ppu: Ppu::default(),
            clock: Clock::default(),
            debugger: Some(Debugger::new(CpuLogType::None, false)),
            speed_multiplier: 1.0,
            state: EmuState::WaitCart,
            config,
            prev_frame: 0,
            last_fps_timestamp: Default::default(),
            rewind_buffer: Default::default(),
            last_rewind_save: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.prev_frame = 0;
        self.last_fps_timestamp = Default::default();
        self.clock = Clock::default();
    }
}

impl CpuCallback for EmuCtx {
    fn m_cycles(&mut self, m_cycles: usize, bus: &mut Bus) {
        self.clock.m_cycles(m_cycles, bus, &mut self.ppu);
    }

    fn update_serial(&mut self, cpu: &mut Cpu) {
        if let Some(debugger) = self.debugger.as_mut() {
            debugger.update_serial(cpu);
        }
    }

    fn debug(&mut self, cpu: &mut Cpu, ctx: Option<DebugCtx>) {
        if let Some(debugger) = self.debugger.as_mut() {
            debugger.print_gb_doctor_info(cpu);

            if let Some(ctx) = ctx {
                debugger.print_cpu_info(
                    &self.clock,
                    cpu,
                    ctx.pc,
                    &ctx.instruction,
                    ctx.opcode,
                    &ctx.fetched_data,
                );
            }
        }
    }
}

impl UiEventHandler for EmuCtx {
    fn on_event(&mut self, _bus: &mut Bus, event: UiEvent) {
        match event {
            UiEvent::Quit => self.state = EmuState::Quit,
            UiEvent::FileDropped(path) => self.state = EmuState::LoadCart(path),
            UiEvent::Pause => {
                if self.state == EmuState::Paused {
                    self.state = EmuState::Running(RunMode::Normal);
                } else {
                    self.state = EmuState::Paused;
                }
            }
            UiEvent::Restart => {
                if let Some(path) = &self.config.last_cart_path {
                    self.state = EmuState::LoadCart(path.to_owned());
                }
            }
            UiEvent::ConfigChanged(config) => self.config.graphics = config,
            UiEvent::ModeChanged(mode) => self.state = EmuState::Running(mode),
            UiEvent::Mute => self.config.emulation.is_muted = !self.config.emulation.is_muted,
        }
    }
}

impl Emu {
    pub fn new(config: Config) -> Result<Self, String> {
        Ok(Self {
            cpu: Cpu::new(Bus::with_bytes(vec![])),
            ui: Ui::new(config.graphics.clone(), false)?,
            ctx: EmuCtx::new(config),
        })
    }

    fn calc_emulated_time(&mut self) -> Duration {
        let speed_multiplier = if let EmuState::Running(mode) = &self.ctx.state {
            match mode {
                RunMode::Rewind | RunMode::Normal => 1.0,
                RunMode::Slow => self.ctx.config.emulation.slow_speed / 100.0,
                RunMode::Turbo => self.ctx.config.emulation.turbo_speed / 100.0,
            }
        } else {
            1.0
        };

        if self.ctx.speed_multiplier != speed_multiplier {
            self.ctx.clock.reset();
        }

        self.ctx.speed_multiplier = speed_multiplier;

        let emulated_time_ns =
            (self.ctx.clock.t_cycles as f64 * CYCLE_TIME / speed_multiplier).round() as u64;

        Duration::from_nanos(emulated_time_ns)
    }

    fn tick(&mut self) -> Result<(), String> {
        let prev_m_cycles = self.ctx.clock.get_m_cycles();

        while self.ctx.clock.get_m_cycles() - prev_m_cycles < CYCLES_PER_FRAME {
            self.cpu.step(&mut self.ctx)?;

            if let Some(debugger) = self.ctx.debugger.as_mut() {
                if !debugger.get_serial_msg().is_empty() {
                    println!("Serial: {}", debugger.get_serial_msg());
                }
            }

            if !self.ctx.config.emulation.is_muted
                && EmuState::Running(RunMode::Normal) == self.ctx.state
            {
                self.ui.audio.update(&mut self.cpu.bus.io.apu)?;
            }
        }

        let real_elapsed = self.ctx.clock.start_time.elapsed();
        let emulated_time = self.calc_emulated_time();

        if emulated_time > real_elapsed {
            spin_wait(emulated_time - real_elapsed);
        }

        if self.ctx.prev_frame != self.ctx.ppu.current_frame {
            self.ui.draw(&mut self.ctx.ppu, &self.cpu.bus);
        }

        self.ctx.prev_frame = self.ctx.ppu.current_frame;

        Ok(())
    }

    pub fn run(&mut self, cart_path: Option<String>) -> Result<(), String> {
        if let Some(cart_path) = &self.ctx.config.last_cart_path {
            if Path::new(cart_path).exists() {
                self.ctx.state = EmuState::LoadCart(cart_path.to_owned());
            }
        }

        if let Some(cart_path) = cart_path {
            self.ctx.state = EmuState::LoadCart(cart_path);
        }

        loop {
            if self.ctx.state == EmuState::Paused || self.ctx.state == EmuState::WaitCart {
                let text = if self.ctx.state == EmuState::Paused {
                    "PAUSED"
                } else {
                    "DROP FILE"
                };
                self.ui.draw_text(text);
                self.ui.handle_events(&mut self.cpu.bus, &mut self.ctx);
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            if self.ctx.state == EmuState::Quit {
                self.ctx.config.save().map_err(|e| e.to_string())?;
                break;
            }

            if let EmuState::LoadCart(path) = &self.ctx.state {
                let cart = read_cart(path).map_err(|e| e.to_string())?;

                let mut bus = Bus::new(cart);
                bus.io.lcd.set_pallet(self.ui.curr_palette);
                self.cpu = Cpu::new(bus);

                self.ctx.config.last_cart_path = Some(path.to_owned());
                self.ctx.state = EmuState::Running(RunMode::Normal);
                self.ctx.reset();
            }

            if let EmuState::Running(RunMode::Rewind) = &self.ctx.state {
                if let Some(state) = self.ctx.rewind_buffer.pop_back() {
                    load_save_state(self, state);
                    thread::sleep(Duration::from_millis(100));
                }
            }

            self.ui.handle_events(&mut self.cpu.bus, &mut self.ctx);
            self.tick()?;

            let now = Instant::now();
            if self.ctx.config.emulation.rewind_size > 0
                && now.duration_since(self.ctx.last_rewind_save).as_secs_f32() >= 2.0
            {
                if self.ctx.rewind_buffer.len() > self.ctx.config.emulation.rewind_size {
                    self.ctx.rewind_buffer.pop_front();
                }

                self.ctx
                    .rewind_buffer
                    .push_back(self.create_save_state(&self.cpu));
                self.ctx.last_rewind_save = now;
            }
        }

        Ok(())
    }

    pub fn create_save_state(&self, cpu: &Cpu) -> EmuSaveState {
        EmuSaveState {
            cpu_without_bus: cpu.clone_without_bus(),
            bus_without_cart: cpu.bus.clone_without_cart(),
            cart_mbc: cpu.bus.cart.mbc.clone(),
        }
    }
}

fn load_save_state(emu: &mut Emu, save_state: EmuSaveState) {
    let mut state_cpu = save_state.cpu_without_bus; // reconstruct cpu
    state_cpu.bus = save_state.bus_without_cart;
    state_cpu.bus.io.joypad = Joypad::default(); // reset controls
    state_cpu.bus.cart.mbc = save_state.cart_mbc; // reconstruct cart
    state_cpu.bus.cart.data = emu.cpu.bus.cart.data.clone();

    emu.cpu = state_cpu;
    emu.ctx.reset();
}

pub fn read_cart(file: &str) -> Result<Cart, String> {
    let bytes = read_bytes(file).map_err(|e| e.to_string())?;
    let cart = Cart::new(bytes).map_err(|e| e.to_string())?;
    _ = print_cart(&cart).map_err(|e| println!("Failed to print cart: {}", e));

    Ok(cart)
}

fn print_cart(cart: &Cart) -> Result<(), String> {
    println!("Cart Loaded:");
    println!("\t Title          : {}", cart.data.get_title()?);
    println!("\t Type           : {:?}", cart.data.get_cart_type()?);
    println!("\t ROM Size       : {:?}", cart.data.get_rom_size()?);
    println!("\t RAM Size       : {:?}", cart.data.get_ram_size()?);
    println!("\t ROM Version    : {:02X}", cart.data.get_rom_version());
    println!("\t Checksum Valid : {}", cart.data.checksum_valid());

    Ok(())
}

pub fn read_bytes(file_path: &str) -> Result<Vec<u8>, String> {
    if !Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }

    fs::read(file_path).map_err(|e| format!("Failed to read file: {}", e))
}
