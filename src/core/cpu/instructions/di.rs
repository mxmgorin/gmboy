use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::{Cpu}; use crate::cpu::instructions::common::FetchedData;

/// Disable Interrupts by clearing the IME flag.
#[derive(Debug, Clone, Copy)]
pub struct DiInstruction;

impl ExecutableInstruction for DiInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        cpu.bus.io.interrupts.int_master_enabled = false;
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
