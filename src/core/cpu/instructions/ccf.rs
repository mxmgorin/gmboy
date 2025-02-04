use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

/// Complement Carry Flag.
/// Cycles: 1
/// Bytes: 1
/// Flags:
/// N 0
/// H 0
/// C Inverted
#[derive(Debug, Clone, Copy)]
pub struct CcfInstruction;

impl ExecutableInstruction for CcfInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        cpu.registers.f.set(
            None,
            Some(false),
            Some(false),
            Some(!cpu.registers.f.get_c()),
        );
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
