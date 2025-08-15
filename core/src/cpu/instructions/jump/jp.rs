use crate::cpu::instructions::{AddressMode, ConditionType, ExecutableInstruction};
use crate::cpu::instructions::{FetchedData, RegisterType};
use crate::cpu::{Cpu};

#[derive(Debug, Clone, Copy)]
pub struct JpInstruction {
    pub address_mode: AddressMode,
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for JpInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        if self.condition_type.is_none()
            && fetched_data.source.get_register() == Some(RegisterType::HL)
        {
            // HL uses and no Cycles
            cpu.registers.pc = fetched_data.value;
        } else {
                cpu.goto_addr(
                self.condition_type,
                fetched_data.value,
                false,
            );
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
