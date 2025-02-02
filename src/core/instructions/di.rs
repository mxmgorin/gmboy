use crate::core::cpu::{Cpu, FetchedData};
use crate::core::instructions::common::{AddressMode, ExecutableInstruction};

#[derive(Debug, Clone, Copy)]
pub struct DiInstruction;

impl ExecutableInstruction for DiInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.bus.io.interrupts.int_master_enabled = false;
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
