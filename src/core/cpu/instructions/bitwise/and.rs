use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::FetchedData;
use crate::cpu::{Cpu, CpuCycleCallback};

#[derive(Debug, Clone, Copy)]
pub struct AndInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for AndInstruction {
    fn execute(
        &self,
        cpu: &mut Cpu,
        _callback: &mut impl CpuCycleCallback,
        fetched_data: FetchedData,
    ) {
        cpu.registers.a &= fetched_data.value as u8;
        cpu.registers.flags.set(
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
