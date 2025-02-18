use crate::core::cpu::instructions::{
    AddressMode, ConditionType, ExecutableInstruction, Instruction,
};
use crate::cpu::instructions::FetchedData;
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct JrInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for JrInstruction {
    fn execute(&self, cpu: &mut Cpu, callback: &mut impl CpuCallback, fetched_data: FetchedData) {
        let rel = fetched_data.value as i8;
        let addr = (cpu.registers.pc as i32).wrapping_add(rel as i32);
        Instruction::goto_addr(cpu, self.condition_type, addr as u16, false, callback);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D8
    }
}
