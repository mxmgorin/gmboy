use crate::auxiliary::clock::Clock;
use crate::bus::Bus;
use crate::cart::Cart;
use crate::config::Config;
use crate::cpu::{Cpu, CpuCallback, DebugCtx};
use crate::debugger::{CpuLogType, Debugger};
use crate::ppu::{Ppu, TARGET_FPS_F};
use crate::ui::events::{UiEvent, UiEventHandler};
use crate::ui::{into_pallet, Ui};
use sdl2::keyboard::Keycode;
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};

#[derive(Debug)]
pub struct Emu {
    running: bool,
    paused: bool,
    ctx: EmuCtx,
    config: Config,
}

#[derive(Debug, Clone, Default)]
pub struct EmuCtx {
    pub clock: Clock,
    pub debugger: Option<Debugger>,
    pub cart: Option<Cart>,
}

impl EmuCtx {
    pub fn with_fps_limit(fps: f64) -> EmuCtx {
        let ppu = Ppu::with_fps_limit(fps);

        Self {
            clock: Clock::with_ppu(ppu),
            debugger: Some(Debugger::new(CpuLogType::None, false)),
            cart: None,
        }
    }
}

impl CpuCallback for EmuCtx {
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

impl UiEventHandler for Emu {
    fn on_event(&mut self, bus: &mut Bus, event: UiEvent) {
        match event {
            UiEvent::Quit => self.running = false,
            UiEvent::Key(keycode, is_down) => {
                match keycode {
                    Keycode::UP => bus.io.joypad.up = is_down,
                    Keycode::DOWN => bus.io.joypad.down = is_down,
                    Keycode::LEFT => bus.io.joypad.left = is_down,
                    Keycode::RIGHT => bus.io.joypad.right = is_down,
                    Keycode::Z => bus.io.joypad.b = is_down,
                    Keycode::X => bus.io.joypad.a = is_down,
                    Keycode::Return => bus.io.joypad.start = is_down,
                    Keycode::BACKSPACE => bus.io.joypad.select = is_down,
                    Keycode::SPACE => {
                        if is_down {
                            self.paused = !self.paused
                        }
                    }
                    Keycode::R => self.ctx.cart = Some(bus.cart.clone()),
                    _ => (), // Ignore other keycodes
                }
            }
            UiEvent::DropFile(filename) => {
                let bytes = read_bytes(&filename);

                let Ok(bytes) = bytes else {
                    eprintln!("Failed to read bytes: {}", bytes.unwrap_err());
                    return;
                };

                let cart = Cart::new(bytes);

                let Ok(cart) = cart else {
                    eprintln!("Failed to load cart: {}", cart.unwrap_err());
                    return;
                };

                self.ctx.cart = Some(cart);
            }
        }
    }
}

impl Emu {
    pub fn new(config: Config) -> Result<Self, String> {
        Ok(Self {
            running: false,
            paused: false,
            ctx: EmuCtx::with_fps_limit(TARGET_FPS_F),
            config,
        })
    }

    pub fn run(&mut self, cart_path: Option<String>) -> Result<(), String> {
        self.running = true;
        let mut prev_frame = 0;
        let mut last_fps_timestamp = Duration::new(0, 0);

        if let Some(cart_path) = cart_path {
            self.ctx.cart = Some(read_cart(&cart_path)?);
        }

        let mut cpu = Cpu::new(Bus::with_bytes(vec![]));
        let mut ui = Ui::new(self.config.graphics.clone(), false)?;

        while self.ctx.cart.is_none() && self.running {
            ui.handle_events(&mut cpu.bus, self);
            thread::sleep(Duration::from_millis(100));
            continue;
        }

        while self.running {
            if let Some(cart) = self.ctx.cart.take() {
                cpu = Cpu::new(Bus::new(cart));
                self.ctx = EmuCtx::with_fps_limit(TARGET_FPS_F);
                last_fps_timestamp = Duration::new(0, 0);
                cpu.bus.io.lcd.set_pallet(ui.curr_palette);
            }

            if self.paused {
                ui.handle_events(&mut cpu.bus, self);
                thread::sleep(Duration::from_millis(100));
                continue;
            }

            ui.handle_events(&mut cpu.bus, self);
            cpu.step(&mut self.ctx)?;

            if let Some(debugger) = self.ctx.debugger.as_mut() {
                if !debugger.get_serial_msg().is_empty() {
                    println!("Serial: {}", debugger.get_serial_msg());
                }
            }

            let ppu = self.ctx.clock.ppu.as_mut().unwrap();
            if prev_frame != ppu.current_frame {
                ui.draw(ppu, &cpu.bus);
            }

            if (ppu.instant.elapsed() - last_fps_timestamp).as_millis() >= 1000 {
                println!("FPS: {}", ppu.fps);
                last_fps_timestamp = ppu.instant.elapsed();
            }

            prev_frame = ppu.current_frame;
        }

        Ok(())
    }
}

pub fn read_cart(file: &str) -> Result<Cart, String> {
    let bytes = read_bytes(file);

    let Ok(bytes) = bytes else {
        return Err(format!("Failed to read bytes: {}", bytes.unwrap_err()));
    };

    let cart = Cart::new(bytes);

    let Ok(cart) = cart else {
        return Err(format!("Failed to load cart: {}", cart.unwrap_err()));
    };

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
