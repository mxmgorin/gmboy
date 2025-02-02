use crate::core::cpu::{Cpu, FetchedData};
use crate::core::instructions::common::{AddressMode, ConditionType, ExecutableInstruction, Instruction};

#[derive(Debug, Clone, Copy)]
pub struct JrInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for JrInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        let rel = fetched_data.value as i8;
        let addr = cpu.registers.pc.wrapping_add(rel as u16);
        Instruction::goto_addr(cpu, self.condition_type, addr, false);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D8
    }
}
