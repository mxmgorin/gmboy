use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct NopInstruction;

impl ExecutableInstruction for NopInstruction {
    fn execute(
        &self,
        _cpu: &mut Cpu,
        _callback: &mut impl CpuCallback,
        _fetched_data: FetchedData,
    ) {
        // does nothing
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
