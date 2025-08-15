use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu};

#[derive(Debug, Clone, Copy)]
pub struct RstInstruction {
    pub address: u16,
}

impl ExecutableInstruction for RstInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        cpu.goto_addr(None, self.address, true);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
