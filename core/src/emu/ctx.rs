use crate::auxiliary::clock::Clock;
use crate::bus::Bus;
use crate::cpu::{Cpu, CpuCallback, DebugCtx};
use crate::debugger::{CpuLogType, Debugger};
use crate::emu::config::EmuConfig;
use crate::emu::save_state::{EmuSaveState, SaveStateEvent};
use crate::emu::{load_save_state, read_cart, Emu};
use crate::ppu::tile::PixelColor;
use crate::ppu::Ppu;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

pub struct EmuCtx {
    pub ppu: Ppu,
    pub clock: Clock,
    pub debugger: Option<Debugger>,
    pub speed_multiplier: f64,
    pub state: EmuState,
    pub config: EmuConfig,
    pub prev_frame: usize,
    pub last_fps_timestamp: Duration,
    pub rewind_buffer: VecDeque<EmuSaveState>,
    pub last_rewind_save: Instant,
    pub pending_save_state: Option<(SaveStateEvent, usize)>,
}

impl EmuCtx {
    pub fn new(config: EmuConfig) -> EmuCtx {
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
            pending_save_state: None,
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

#[derive(Debug, Clone, PartialEq)]
pub enum RunMode {
    Normal,
    Slow,
    Turbo,
    Rewind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EmuState {
    WaitCart,
    Running(RunMode),
    Paused,
    LoadCart(PathBuf),
    Quit,
}

impl EmuState {
    pub fn handle_load_cart(emu: &mut Emu, pallet: [PixelColor; 4]) {
        if let EmuState::LoadCart(path) = &emu.ctx.state {
            let cart = read_cart(path).map_err(|e| e.to_string());

            let Ok(cart) = cart else {
                eprintln!("Failed read_cart: {}", cart.unwrap_err());
                return;
            };

            let mut bus = Bus::new(cart);
            bus.io.lcd.set_pallet(pallet);
            emu.cpu = Cpu::new(bus);
            emu.ctx.config.last_cart_path = Some(path.to_string_lossy().to_string());
            emu.ctx.state = EmuState::Running(RunMode::Normal);
            emu.ctx.reset();

            if emu.ctx.config.load_save_state_at_start {
                let name = emu.ctx.config.get_last_cart_file_stem().unwrap();
                let save_state = EmuSaveState::load_file(&name, 0);

                if let Ok(save_state) = save_state {
                    load_save_state(emu, save_state);
                } else {
                    eprintln!("Failed load save_state: {:?}", save_state);
                };
            }
        }
    }

    pub fn handle_pending_save_state(emu: &mut Emu) {
        if let Some((event, index)) = emu.ctx.pending_save_state.take() {
            let name = emu.ctx.config.get_last_cart_file_stem().unwrap();

            match event {
                SaveStateEvent::Create => {
                    let save_state = emu.create_save_state(&emu.cpu);

                    if let Err(err) = save_state.save_file(&name, index) {
                        eprintln!("Failed save_state: {:?}", err);
                    }
                }
                SaveStateEvent::Load => {
                    let save_state = EmuSaveState::load_file(&name, index);

                    let Ok(save_state) = save_state else {
                        eprintln!("Failed load save_state: {:?}", save_state);
                        return;
                    };

                    load_save_state(emu, save_state);
                }
            }
        }
    }

    pub fn handle_rewind(emu: &mut Emu) {
        if let EmuState::Running(RunMode::Rewind) = &emu.ctx.state {
            if let Some(state) = emu.ctx.rewind_buffer.pop_back() {
                load_save_state(emu, state);
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
