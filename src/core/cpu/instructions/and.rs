use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct AndInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for AndInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.registers.a &= fetched_data.value as u8;
        cpu.registers.f.set(
            Some(cpu.registers.a == 0),
            Some(false),
            Some(true),
            Some(false),
        );
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
