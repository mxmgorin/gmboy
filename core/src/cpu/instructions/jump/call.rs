use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{AddressMode, ConditionType, ExecutableInstruction};
use crate::cpu::{Cpu};

#[derive(Debug, Clone, Copy)]
pub struct CallInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for CallInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.goto_addr(self.condition_type, fetched_data.value, true);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D16
    }
}
