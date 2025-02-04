use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::{ConditionType, FetchedData};
use crate::cpu::stack::Stack;

#[derive(Debug, Clone, Copy)]
pub struct RetInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for RetInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        if self.condition_type.is_some() {
            cpu.update_cycles(1);
        }

        if ConditionType::check_cond(&cpu.registers, self.condition_type) {
            let lo = Stack::pop(&mut cpu.registers, &mut cpu.bus) as u16;
            cpu.update_cycles(1);

            let hi = Stack::pop(&mut cpu.registers, &mut cpu.bus) as u16;
            cpu.update_cycles(1);

            let n = (hi << 8) | lo;
            cpu.registers.pc = n;

            cpu.update_cycles(1);
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
