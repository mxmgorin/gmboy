use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
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

        cpu.registers.flags.set(
            (cpu.registers.a == 0).into(),
            false.into(),
            ((a & 0xF) + (u & 0xF) + c > 0xF).into(),
            (a + u + c > 0xFF).into(),
        );
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
