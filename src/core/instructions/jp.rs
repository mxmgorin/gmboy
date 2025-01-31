use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, ConditionType, ExecutableInstruction, Instruction};

#[derive(Debug, Clone, Copy)]
pub struct JpInstruction {
    pub address_mode: AddressMode,
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for JpInstruction {
    fn execute(&self, cpu: &mut Cpu) {
        Instruction::goto_addr(cpu, self.condition_type, cpu.fetched_data, false);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}