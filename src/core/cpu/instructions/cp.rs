use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct CpInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for CpInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        let fetched_value_i32 = fetched_data.value as i32;
        let reg_a_i32 = cpu.registers.a as i32;
        let n: i32 = reg_a_i32 - fetched_value_i32;
        let reg_value_diff = (reg_a_i32 & 0x0F) - (fetched_value_i32 & 0x0F);

        cpu.registers.set_flags(
            ((n == 0) as i8).into(),
            1.into(),
            ((reg_value_diff < 0) as i8).into(),
            ((n < 0) as i8).into(),
        )
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
