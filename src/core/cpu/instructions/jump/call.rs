use crate::core::cpu::instructions::{
    AddressMode, ConditionType, ExecutableInstruction, Instruction,
};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct CallInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for CallInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        Instruction::goto_addr(cpu, self.condition_type, fetched_data.value, true);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D16
    }
}
