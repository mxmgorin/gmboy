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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmuSaveState {
    pub cpu: Cpu,
    pub cart_save_state: CartSaveState,
}
