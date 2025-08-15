use crate::cpu::instructions::{AddressMode, ExecutableInstruction, FetchedData};
use crate::cpu::{Cpu};

#[derive(Debug, Clone, Copy)]
pub struct OrInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for OrInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        let value = fetched_data.value & 0xFF;
        cpu.registers.a |= value as u8;
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
