use crate::auxiliary::clock::Clock;
use crate::bus::Bus;
use crate::cpu::{Cpu, CpuCallback, DebugCtx};
use crate::debugger::Debugger;
pub use crate::emu::state::{EmuSaveState, SaveStateCommand};
use crate::emu::EmuCallback;
use crate::ppu::Ppu;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum RunMode {
    Normal,
    Slow,
    Turbo,
}

/// Contains all runnable components.
pub struct EmuRuntime {
    pub mode: RunMode,
    pub bus: Bus,
    pub ppu: Ppu,
    pub clock: Clock,
    pub debugger: Option<Debugger>,
}

impl EmuRuntime {
    pub fn new(ppu: Ppu, bus: Bus, debugger: Option<Debugger>) -> EmuRuntime {
        Self {
            mode: RunMode::Normal,
            ppu,
            clock: Clock::default(),
            debugger,
            bus,
        }
    }

    pub fn set_mode(&mut self, mode: RunMode) {
        self.mode = mode;
    }

    pub fn run_frame(
        &mut self,
        cpu: &mut Cpu,
        callback: &mut impl EmuCallback,
    ) -> Result<(), String> {
        let start_frame = self.ppu.current_frame;

        while start_frame == self.ppu.current_frame {
            cpu.step(self)?;

            if let Some(debugger) = self.debugger.as_mut() {
                debugger.print_serial()
            }

            if self.bus.io.apu.buffer_ready() {
                let output = self.bus.io.apu.get_buffer();
                callback.update_audio(output, self);
                self.bus.io.apu.clear_buffer();
            }
        }

        callback.update_video(&self.ppu.pipeline.buffer, self);

        Ok(())
    }
}

impl CpuCallback for EmuRuntime {
    fn m_cycles(&mut self, m_cycles: usize) {
        self.clock.m_cycles(m_cycles, &mut self.bus, &mut self.ppu);
    }

    fn update_serial(&mut self, _cpu: &mut Cpu) {
        if let Some(debugger) = self.debugger.as_mut() {
            debugger.update_serial(&mut self.bus);
        }
    }

    fn debug(&mut self, cpu: &mut Cpu, ctx: Option<DebugCtx>) {
        if let Some(debugger) = self.debugger.as_mut() {
            debugger.print(cpu, &self.clock, ctx, &self.bus);
        }
    }

    fn get_bus_mut(&mut self) -> &mut Bus {
        &mut self.bus
    }
}
