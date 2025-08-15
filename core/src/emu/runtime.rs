use crate::auxiliary::clock::{Clock};
use crate::bus::Bus;
use crate::cpu::{Cpu};
use crate::debugger::Debugger;
pub use crate::emu::state::{EmuSaveState, SaveStateCmd};
use crate::emu::EmuAudioCallback;
use crate::ppu::Ppu;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum RunMode {
    Normal,
    Slow,
    Turbo,
}

/// Contains all runnable components.
pub struct EmuRuntime
{
    pub mode: RunMode,
    pub cpu: Cpu,
    debugger: Option<Debugger>,
}

impl EmuRuntime
{
    pub fn new(ppu: Ppu, bus: Bus, debugger: Option<Debugger>) -> Self {
        Self {
            mode: RunMode::Normal,
            cpu: Cpu::new(Clock::new(ppu, bus)),
            debugger,
        }
    }

    pub fn set_mode(&mut self, mode: RunMode) {
        self.mode = mode;
    }

    pub fn run_frame(&mut self, callback: &mut impl EmuAudioCallback) -> Result<(), String> {
        let start_frame = self.cpu.clock.ppu.current_frame;

        while start_frame == self.cpu.clock.ppu.current_frame {
            self.cpu.step(self.debugger.as_mut())?;

            if let Some(debugger) = self.debugger.as_mut() {
                debugger.print_serial()
            }

            if self.cpu.clock.bus.io.apu.buffer_ready() {
                let output = self.cpu.clock.bus.io.apu.get_buffer();
                callback.update(output, self);
                self.cpu.clock.bus.io.apu.clear_buffer();
            }
        }

        Ok(())
    }
}
