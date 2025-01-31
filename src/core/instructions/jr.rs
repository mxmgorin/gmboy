use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, ConditionType, ExecutableInstruction, RegisterType};

#[derive(Debug, Clone, Copy)]
pub struct JrInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for JrInstruction {
    fn execute(&self, _cpu: &mut Cpu) {
        eprintln!("JrInstruction not impl")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D8
    }
}
