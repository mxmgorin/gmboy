use crate::bus::Bus;
use crate::cart::CartSaveState;
use crate::cpu::Cpu;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum EmuState {
    Running,
    Rewind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SaveStateCmd {
    Create,
    Load,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmuSaveState {
    pub cpu: Cpu,
    pub bus_without_cart: Bus,
    pub cart_save_state: CartSaveState,
}
