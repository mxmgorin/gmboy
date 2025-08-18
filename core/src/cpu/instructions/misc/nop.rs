use crate::cpu::instructions::{FetchedData, InstructionArgs};
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_nop(&mut self, _fetched_data: FetchedData, _args: InstructionArgs) {
        // does nothing
    }
}

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
