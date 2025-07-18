use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu, CpuCallback};

/// ComPLement accumulator (A = ~A); also called bitwise NOT.
/// Cycles: 1
/// Bytes: 1
/// Flags:
/// N 1
/// H 1
#[derive(Debug, Clone, Copy)]
pub struct CplInstruction;

impl ExecutableInstruction for CplInstruction {
    fn execute(&self, cpu: &mut Cpu, _callback: &mut impl CpuCallback, _fetched_data: FetchedData) {
        cpu.registers.a = !cpu.registers.a;
        cpu.registers
            .flags
            .set(None, true.into(), true.into(), None);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
