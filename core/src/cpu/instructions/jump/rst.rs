use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{FetchedData, InstructionArgs};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_rst(&mut self, _fetched_data: FetchedData, args: InstructionArgs) {
        self.goto_addr(None, args.addr, true);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RstInstruction {
    pub address: u16,
}

impl ExecutableInstruction for RstInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        cpu.execute_rst(_fetched_data, InstructionArgs::new(None, self.address, self.get_address_mode()));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
