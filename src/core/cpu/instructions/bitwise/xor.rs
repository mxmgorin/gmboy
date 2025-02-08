use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct XorInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for XorInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.registers.a ^= (fetched_data.value & 0xFF) as u8;
        cpu.registers.flags.set(
            (cpu.registers.a == 0).into(),
            false.into(),
            false.into(),
            false.into(),
        );
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
