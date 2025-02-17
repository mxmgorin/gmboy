use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::FetchedData;
use crate::cpu::{Cpu, CpuCycleCallback};

/// Set Carry Flag.
/// Cycles: 1
/// Bytes: 1
/// Flags:
/// N 0
/// H 0
/// C1
#[derive(Debug, Clone, Copy)]
pub struct ScfInstruction;

impl ExecutableInstruction for ScfInstruction {
    fn execute(
        &self,
        cpu: &mut Cpu,
        _callback: &mut impl CpuCycleCallback,
        _fetched_data: FetchedData,
    ) {
        cpu.registers
            .flags
            .set(None, false.into(), false.into(), true.into());
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
