use crate::core::cpu::{Cpu, FetchedData};
use crate::core::instructions::common::{AddressMode, ExecutableInstruction};

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
