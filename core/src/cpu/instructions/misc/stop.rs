use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu};

#[derive(Debug, Clone, Copy)]
pub struct StopInstruction;

impl ExecutableInstruction for StopInstruction {
    fn execute(
        &self,
        _cpu: &mut Cpu,

        _fetched_data: FetchedData,
    ) {
        // todo: research
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
