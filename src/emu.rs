use crate::auxiliary::clock::Clock;
use crate::bus::Bus;
use crate::cart::Cart;
use crate::config::Config;
use crate::cpu::{Cpu, CpuCallback, DebugCtx};
use crate::debugger::{CpuLogType, Debugger};
use crate::ppu::Ppu;
use crate::ui::events::{UiEvent, UiEventHandler};
use crate::ui::Ui;
use crate::TARGET_FPS_F;
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

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
}

impl EmuCtx {
    pub fn new(config: Config) -> EmuCtx {
        Self {
            state: EmuState::WaitCart,
            config,
            prev_frame: 0,
            last_fps_timestamp: Default::default(),
        }
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
            ui: Ui::new(config.graphics.clone(), false)?,
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

                let path = path.to_owned();
                self.ctx = EmuCtx::new(self.ctx.config.clone());
                self.ctx.state = EmuState::Running(RunMode::Normal);
                self.ctx.config.last_cart_path = Some(path.to_owned());
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
                    RunMode::Normal => ppu.set_fps_limit(TARGET_FPS_F),
                    RunMode::Slow => ppu.set_fps_limit(TARGET_FPS_F / 2.0),
                    RunMode::Turbo => ppu.set_fps_limit(TARGET_FPS_F * 2.0),
                }
            }

            if self.ctx.prev_frame != ppu.current_frame {
                self.ui.draw(ppu, &cpu.bus);
            }

            self.ctx.prev_frame = ppu.current_frame;
        }

        Ok(())
    }
}

pub fn read_cart(file: &str) -> Result<Cart, String> {
    let bytes = read_bytes(file).map_err(|e| e.to_string())?;
    let cart = Cart::new(bytes).map_err(|e| e.to_string())?;
    print_cart(&cart);

    Ok(cart)
}

fn print_cart(cart: &Cart) {
    println!("Cart Loaded:");
    println!("\t Title    : {}", cart.header.title);
    println!("\t Type     : {:?}", cart.header.cart_type);
    println!("\t ROM Size : {:?}", cart.header.rom_size);
    println!("\t RAM Size : {:?}", cart.header.ram_size);
    println!("\t LIC Code : {:?} ", cart.header.new_licensee_code);
    println!("\t ROM Version : {:02X}", cart.header.mask_rom_version);
}

pub fn read_bytes(file_path: &str) -> Result<Vec<u8>, String> {
    if !Path::new(file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }

    fs::read(file_path).map_err(|e| format!("Failed to read file: {}", e))
}
