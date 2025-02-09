use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::{ConditionType, FetchedData};
use crate::cpu::stack::Stack;

#[derive(Debug, Clone, Copy)]
pub struct RetInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for RetInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        if self.condition_type.is_some() {
            cpu.m_cycles(1);
        }

        if ConditionType::check_cond(&cpu.registers, self.condition_type) {
            let lo = Stack::pop(cpu) as u16;
            let hi = Stack::pop(cpu) as u16;

            let addr = (hi << 8) | lo;

            cpu.set_pc(addr);
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
