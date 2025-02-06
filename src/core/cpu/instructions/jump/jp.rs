use crate::core::cpu::instructions::common::{
    AddressMode, ConditionType, ExecutableInstruction, Instruction,
};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct JpInstruction {
    pub address_mode: AddressMode,
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for JpInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        Instruction::goto_addr(cpu, self.condition_type, fetched_data.value, false);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
