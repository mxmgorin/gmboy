use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction, Instruction};
use crate::cpu::instructions::FetchedData;
use crate::cpu::{Cpu, CpuCycleCallback};

#[derive(Debug, Clone, Copy)]
pub struct RstInstruction {
    pub address: u16,
}

impl ExecutableInstruction for RstInstruction {
    fn execute(
        &self,
        cpu: &mut Cpu,
        callback: &mut impl CpuCycleCallback,
        _fetched_data: FetchedData,
    ) {
        Instruction::goto_addr(cpu, None, self.address, true, callback);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
