use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::FetchedData;
use crate::cpu::{Cpu, CpuCycleCallback};

#[derive(Debug, Clone, Copy)]
pub struct HaltInstruction;

impl ExecutableInstruction for HaltInstruction {
    fn execute(
        &self,
        cpu: &mut Cpu,
        _callback: &mut impl CpuCycleCallback,
        _fetched_data: FetchedData,
    ) {
        cpu.is_halted = true;
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
