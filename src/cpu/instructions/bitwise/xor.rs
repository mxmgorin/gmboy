use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::FetchedData;
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct XorInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for XorInstruction {
    fn execute(&self, cpu: &mut Cpu, _callback: &mut impl CpuCallback, fetched_data: FetchedData) {
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
