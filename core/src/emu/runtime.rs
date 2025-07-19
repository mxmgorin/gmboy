use crate::auxiliary::clock::Clock;
use crate::bus::Bus;
use crate::cpu::{Cpu, CpuCallback, DebugCtx};
use crate::debugger::Debugger;
pub use crate::emu::state::{EmuSaveState, SaveStateEvent};
use crate::ppu::Ppu;

pub struct EmuRuntime {
    pub bus: Bus,
    pub ppu: Ppu,
    pub clock: Clock,
    pub debugger: Option<Debugger>,
}

impl EmuRuntime {
    pub fn new(debugger: Option<Debugger>, bus: Bus) -> EmuRuntime {
        Self {
            ppu: Ppu::default(),
            clock: Clock::default(),
            debugger,
            bus,
        }
    }

    pub fn reset(&mut self) {
        self.clock = Clock::default();
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
