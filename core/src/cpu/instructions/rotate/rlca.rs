use crate::cpu::instructions::{AddressMode, ExecutableInstruction, FetchedData, InstructionArgs};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_rlca(&mut self, _fetched_data: FetchedData, _args: InstructionArgs) {
        let mut u: u8 = self.registers.a;
        let c: bool = (u >> 7) & 1 != 0;
        u = (u << 1) | c as u8;
        self.registers.a = u;

        self.registers.flags.set_z(false);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(c);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RlcaInstruction;

impl ExecutableInstruction for RlcaInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        cpu.execute_rlca(_fetched_data, InstructionArgs::default(self.get_address_mode()));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
