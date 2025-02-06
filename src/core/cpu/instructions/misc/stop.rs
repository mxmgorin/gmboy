use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct StopInstruction;

impl ExecutableInstruction for StopInstruction {
    fn execute(&self, _cpu: &mut Cpu, _fetched_data: FetchedData) {
        // todo: research
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
