use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, ExecutableInstruction};

#[derive(Debug, Clone, Copy)]
pub struct XorInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for XorInstruction {
    fn execute(&self, cpu: &mut Cpu) {
        cpu.registers.a ^= (cpu.fetched_data & 0xFF) as u8;
        cpu.set_flags((cpu.registers.a == 0) as i8, 0, 0, 0);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
