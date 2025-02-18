use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction, Instruction};
use crate::cpu::instructions::{FetchedData};
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct RstInstruction {
    pub address: u16,
}

impl ExecutableInstruction for RstInstruction {
    fn execute(&self, cpu: &mut Cpu, callback: &mut impl CpuCallback, _fetched_data: FetchedData) {
        callback.m_cycles(1, &mut cpu.bus);
        Instruction::goto_addr(cpu, self.address, true, callback);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
