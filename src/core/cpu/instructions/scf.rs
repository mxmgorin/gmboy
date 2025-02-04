use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct ScfInstruction;

impl ExecutableInstruction for ScfInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        cpu.registers.f.set(None, false.into(), false.into(), true.into());
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
