use crate::core::cpu::{Cpu, FetchedData};
use crate::core::instructions::common::{
    AddressMode, ConditionType, ExecutableInstruction, Instruction,
};

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
