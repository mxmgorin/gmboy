use std::fmt;

use crate::auxiliary::clock::Clock;
use crate::bus::Bus;
use crate::cpu::Cpu;
#[cfg(feature = "debug")]
use crate::debugger::Debugger;
pub use crate::emu::state::{EmuSaveState, SaveStateCmd};
use crate::emu::EmuAudioCallback;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum RunMode {
    Normal,
    Slow,
    Turbo,
}

impl RunMode {
    pub const fn name(&self) -> &'static str {
        match self {
            RunMode::Normal => "Normal",
            RunMode::Slow => "Slow",
            RunMode::Turbo => "Turbo",
        }
    }
}

impl fmt::Display for RunMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Contains all runnable components.
pub struct EmuRuntime {
    pub mode: RunMode,
    pub cpu: Cpu,
    #[cfg(feature = "debug")]
    debugger: Option<Debugger>,
}

impl EmuRuntime {
    #[cfg(feature = "debug")]
    pub fn new(bus: Bus, debugger: Option<Debugger>) -> Self {
        Self {
            mode: RunMode::Normal,
            cpu: Cpu::new(Clock::new(bus)),
            #[cfg(feature = "debug")]
            debugger,
        }
    }

    #[cfg(not(feature = "debug"))]
    pub fn new(bus: Bus) -> Self {
        Self {
            mode: RunMode::Normal,
            cpu: Cpu::new(Clock::new(bus)),
        }
    }

    #[inline]
    pub fn set_mode(&mut self, mode: RunMode) {
        self.mode = mode;
    }

    #[inline(always)]
    pub fn run_frame(&mut self, callback: &mut impl EmuAudioCallback) {
        let start_frame = self.cpu.clock.bus.io.ppu.current_frame;

        while start_frame == self.cpu.clock.bus.io.ppu.current_frame {
            #[cfg(feature = "debug")]
            if let Some(debugger) = self.debugger.as_mut() {
                self.cpu.step_debug(debugger);
            }

            #[cfg(not(feature = "debug"))]
            self.cpu.step();

            if self.cpu.clock.bus.io.apu.buffer_ready() {
                let output = self.cpu.clock.bus.io.apu.get_buffer();
                callback.update(output, self);
                self.cpu.clock.bus.io.apu.clear_buffer();
            }
        }
    }
}
