use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, ConditionType, ExecutableInstruction};

#[derive(Debug, Clone, Copy)]
pub struct CallInstruction {
    pub condition_type: ConditionType,
}

impl ExecutableInstruction for CallInstruction {
    fn execute(&self, _cpu: &mut Cpu) {
        panic!("CallInstruction not impl")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D16
    }
}
