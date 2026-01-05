use core::fmt;

use crate::cart::CartSaveState;
use crate::cpu::Cpu;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum EmuState {
    Running,
    Rewind,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum SaveStateCmd {
    Create,
    Load,
}

impl SaveStateCmd {
    pub const fn name(&self) -> &'static str {
        match self {
            SaveStateCmd::Create => "Save",
            SaveStateCmd::Load => "Load",
        }
    }
}

impl fmt::Display for SaveStateCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmuSaveState {
    pub cpu: Cpu,
    pub cart_save_state: CartSaveState,
}
