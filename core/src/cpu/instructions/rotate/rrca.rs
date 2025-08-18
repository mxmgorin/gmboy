use crate::cpu::instructions::{AddressMode, ExecutableInstruction, FetchedData, InstructionArgs};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_rrca(&mut self, _fetched_data: FetchedData, _args: InstructionArgs) {
        let b: u8 = self.registers.a & 1;
        self.registers.a >>= 1;
        self.registers.a |= b << 7;

        self.registers.flags.set_z(false);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(b != 0);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RrcaInstruction;

impl ExecutableInstruction for RrcaInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        cpu.execute_rrca(_fetched_data, InstructionArgs::default(self.get_address_mode()));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
