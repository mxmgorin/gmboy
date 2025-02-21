use crate::cpu::instructions::{
    AddressMode, ConditionType, ExecutableInstruction, Instruction,
};
use crate::cpu::instructions::FetchedData;
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct CallInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for CallInstruction {
    fn execute(&self, cpu: &mut Cpu, callback: &mut impl CpuCallback, fetched_data: FetchedData) {
        Instruction::goto_addr(cpu, self.condition_type, fetched_data.value, true, callback);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D16
    }
}
