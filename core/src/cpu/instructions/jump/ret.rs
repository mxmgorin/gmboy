use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{ConditionType, FetchedData};
use crate::cpu::{Cpu};

#[derive(Debug, Clone, Copy)]
pub struct RetInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for RetInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        if self.condition_type.is_some() {
            cpu.clock.m_cycles(1); // internal: branch decision?
        }

        if ConditionType::check_cond(&cpu.registers, self.condition_type) {
            let lo = cpu.pop() as u16;
            let hi = cpu.pop() as u16;

            let addr = (hi << 8) | lo;
            cpu.registers.pc = addr;
            cpu.clock.m_cycles(1); // internal: set PC?
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
