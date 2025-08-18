use crate::cpu::instructions::{FetchedData, InstructionArgs};
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_rla(&mut self, _fetched_data: FetchedData, _args: InstructionArgs) {
        let u: u8 = self.registers.a;
        let cf: u8 = self.registers.flags.get_c() as u8;
        let c: u8 = (u >> 7) & 1;

        self.registers.a = (u << 1) | cf;

        self.registers.flags.set_z(false);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(c != 0);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RlaInstruction;

impl ExecutableInstruction for RlaInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        cpu.execute_rla(_fetched_data, InstructionArgs::default(self.get_address_mode()));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
