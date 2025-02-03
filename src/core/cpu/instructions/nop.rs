use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::{Cpu, FetchedData};

#[derive(Debug, Clone, Copy)]
pub struct NopInstruction;

impl ExecutableInstruction for NopInstruction {
    fn execute(&self, _cpu: &mut Cpu, _fetched_data: FetchedData) {
        // does nothing
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
