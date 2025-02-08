use crate::core::cpu::instructions::{
    AddressMode, ConditionType, ExecutableInstruction, Instruction,
};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct JpInstruction {
    pub address_mode: AddressMode,
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for JpInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        if self.condition_type.is_none() {
            // FIXME: 0xC3
            // uses only HL and no Cycles
            cpu.registers.pc = fetched_data.value;
        } else {
            Instruction::goto_addr(cpu, self.condition_type, fetched_data.value, false);
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
