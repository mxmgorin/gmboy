use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu};

use crate::cpu::instructions::{DataDestination, FetchedData};

#[derive(Debug, Clone, Copy)]
pub struct AdcInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for AdcInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        let DataDestination::Register(_) = fetched_data.dest else {
            unreachable!();
        };

        let u: u16 = fetched_data.value;
        let a: u16 = cpu.registers.a as u16;
        let c: u16 = cpu.registers.flags.get_c() as u16;

        cpu.registers.a = ((a + u + c) & 0xFF) as u8;

        cpu.registers.flags.set_z(cpu.registers.a == 0);
        cpu.registers.flags.set_n(false);
        cpu.registers.flags.set_h((a & 0xF) + (u & 0xF) + c > 0xF);
        cpu.registers.flags.set_c(a + u + c > 0xFF);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
