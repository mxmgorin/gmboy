use crate::cpu::instructions::{FetchedData, InstructionArgs};
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_xor(&mut self, fetched_data: FetchedData, _args: InstructionArgs) {
        self.registers.a ^= (fetched_data.value & 0xFF) as u8;

        self.registers.flags.set_z(self.registers.a == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(false);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct XorInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for XorInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_xor(fetched_data, InstructionArgs::default(self.address_mode));
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
