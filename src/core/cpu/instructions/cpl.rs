use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

/// ComPLement accumulator (A = ~A); also called bitwise NOT.
/// Cycles: 1
/// Bytes: 1
/// Flags:
/// N 1
/// H 1
#[derive(Debug, Clone, Copy)]
pub struct CplInstruction;

impl ExecutableInstruction for CplInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        cpu.registers.a = !cpu.registers.a;
        cpu.registers.f.set(None, 1.into(), 1.into(), None);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
