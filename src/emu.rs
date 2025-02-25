use crate::auxiliary::clock::Clock;
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
use std::collections::VecDeque;
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

pub struct EmuSaveState {
    pub clock: Clock,
    pub cpu_without_bus: Cpu,
    pub bus_without_cart: Bus,
    pub cart_mbc: Option<MbcVariant>,
}

pub struct Emu {
    pub clock: Clock,
    pub debugger: Option<Debugger>,
    pub ui: Ui,

    pub ctx: EmuCtx,
}

pub struct EmuCtx {
    pub state: EmuState,
    pub config: Config,
    pub prev_frame: usize,
    pub last_fps_timestamp: Duration,
    pub rewind_buffer: VecDeque<EmuSaveState>,
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
            state: EmuState::WaitCart,
            config,
            prev_frame: 0,
            last_fps_timestamp: Default::default(),
            rewind_buffer: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        self.prev_frame = 0;
        self.last_fps_timestamp = Default::default();
    }
}

impl CpuCallback for Emu {
    fn m_cycles(&mut self, m_cycles: usize, bus: &mut Bus) {
        self.clock.m_cycles(m_cycles, bus);
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
            UiEvent::DropFile(path) => self.state = EmuState::LoadCart(path),
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
            UiEvent::Mode(mode) => self.state = EmuState::Running(mode),
        }
    }
}

impl Emu {
    pub fn new(config: Config) -> Result<Self, String> {
        let ppu = Ppu::with_fps_limit(config.graphics.fps_limit);

        Ok(Self {
            clock: Clock::with_ppu(ppu),
            debugger: Some(Debugger::new(CpuLogType::None, false)),
            ui: Ui::new(Default::default(), config.graphics.clone(), false)?,
            ctx: EmuCtx::new(config),
        })
    }

    pub fn run(&mut self, cart_path: Option<String>) -> Result<(), String> {
        if let Some(cart_path) = &self.ctx.config.last_cart_path {
            self.ctx.state = EmuState::LoadCart(cart_path.to_owned());
        }

        if let Some(cart_path) = cart_path {
            self.ctx.state = EmuState::LoadCart(cart_path);
        }

        let mut cpu = Cpu::new(Bus::with_bytes(vec![]));

        loop {
            if self.ctx.state == EmuState::Paused || self.ctx.state == EmuState::WaitCart {
                self.ui.draw_text("DROP FILE");
                self.ui.handle_events(&mut cpu.bus, &mut self.ctx);
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
                cpu = Cpu::new(bus);

                self.ctx.config.last_cart_path = Some(path.to_owned());
                self.ctx.state = EmuState::Running(RunMode::Normal);
                self.ctx.reset();
            }

            if let EmuState::Running(RunMode::Rewind) = &self.ctx.state {
                if let Some(state) = self.ctx.rewind_buffer.pop_back() {
                    self.load_state(&mut cpu, state);
                }
            }

            self.ui.handle_events(&mut cpu.bus, &mut self.ctx);
            cpu.step(self)?;

            if let Some(debugger) = self.debugger.as_mut() {
                if !debugger.get_serial_msg().is_empty() {
                    println!("Serial: {}", debugger.get_serial_msg());
                }
            }

            let ppu = self.clock.ppu.as_mut().unwrap();

            if let EmuState::Running(mode) = &self.ctx.state {
                match mode {
                    RunMode::Normal => ppu.set_fps_limit(self.ctx.config.graphics.fps_limit),
                    RunMode::Slow => ppu.set_fps_limit(
                        self.ctx.config.graphics.fps_limit * self.ctx.config.emulation.slow_speed
                            / 100.0,
                    ),
                    RunMode::Turbo => ppu.set_fps_limit(
                        self.ctx.config.graphics.fps_limit * self.ctx.config.emulation.turbo_speed
                            / 100.0,
                    ),
                    RunMode::Rewind => (),
                }
            }

            if self.ctx.prev_frame != ppu.current_frame {
                self.ui.draw(ppu, &cpu.bus);
            }

            self.ctx.prev_frame = ppu.current_frame;

            if self.ctx.config.emulation.rewind_size > 0 && self.clock.t_cycles % 5000 == 0 {
                if self.ctx.rewind_buffer.len() > self.ctx.config.emulation.rewind_size {
                    self.ctx.rewind_buffer.pop_front();
                }

                self.ctx.rewind_buffer.push_back(self.save_state(&cpu));
            }
        }

        Ok(())
    }

    pub fn save_state(&self, cpu: &Cpu) -> EmuSaveState {
        EmuSaveState {
            clock: self.clock.clone(),
            cpu_without_bus: cpu.clone_without_bus(),
            bus_without_cart: cpu.bus.clone_without_cart(),
            cart_mbc: cpu.bus.cart.mbc.clone(),
        }
    }

    pub fn load_state(&mut self, cpu: &mut Cpu, save_state: EmuSaveState) {
        let mut state_cpu = save_state.cpu_without_bus; // reconstruct cpu
        state_cpu.bus = save_state.bus_without_cart;
        state_cpu.bus.io.joypad = Joypad::default(); // reset controls
        state_cpu.bus.cart.mbc = save_state.cart_mbc; // reconstruct cart
        state_cpu.bus.cart.data = cpu.bus.cart.data.clone();

        *cpu = state_cpu;
        self.clock = save_state.clock;
        self.ctx.reset();
    }
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
