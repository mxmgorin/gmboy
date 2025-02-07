use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction, Instruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct RstInstruction {
    pub address: u16,
}

impl ExecutableInstruction for RstInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        Instruction::goto_addr(cpu, None, self.address, true);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
