use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu};

#[derive(Debug, Clone, Copy)]
pub struct AndInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for AndInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.registers.a &= fetched_data.value as u8;

        cpu.registers.flags.set_z(cpu.registers.a == 0);
        cpu.registers.flags.set_n(false);
        cpu.registers.flags.set_h(true);
        cpu.registers.flags.set_c(false);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
