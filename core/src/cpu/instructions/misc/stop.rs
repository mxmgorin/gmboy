use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{FetchedData, InstructionArgs};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_stop(&mut self, _fetched_data: FetchedData, _args: InstructionArgs) {}
}

#[derive(Debug, Clone, Copy)]
pub struct StopInstruction;

impl ExecutableInstruction for StopInstruction {
    fn execute(&self, _cpu: &mut Cpu, _fetched_data: FetchedData) {
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
