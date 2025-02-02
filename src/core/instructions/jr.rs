use crate::core::cpu::{Cpu, FetchedData};
use crate::core::instructions::common::{AddressMode, ConditionType, ExecutableInstruction};

#[derive(Debug, Clone, Copy)]
pub struct JrInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for JrInstruction {
    fn execute(&self, _cpu: &mut Cpu, fetched_data: FetchedData) {
        unimplemented!("Execute JrInstruction")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D8
    }
}
