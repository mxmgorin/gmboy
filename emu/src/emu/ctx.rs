use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use crate::auxiliary::clock::Clock;
use crate::debugger::{CpuLogType, Debugger};
use crate::emu::config::EmuConfig;
use crate::{Cpu, CpuCallback, DebugCtx, Ppu};
use crate::bus::Bus;
use crate::emu::save_state::EmuSaveState;
use crate::save_state::SaveStateEvent;

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

#[derive(Debug, Clone, PartialEq)]
pub enum EmuState {
    WaitCart,
    Running(RunMode),
    Paused,
    LoadCart(PathBuf),
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
