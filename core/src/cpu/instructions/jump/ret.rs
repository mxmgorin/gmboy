use crate::cpu::instructions::{AddressMode, ExecutableInstruction, InstructionArgs};
use crate::cpu::instructions::{ConditionType, FetchedData};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_ret(&mut self, _fetched_data: FetchedData, args: InstructionArgs) {
        if args.cond_type.is_some() {
            self.clock.m_cycles(1); // internal: branch decision?
        }

        if ConditionType::check_cond(&self.registers, args.cond_type) {
            let lo = self.pop() as u16;
            let hi = self.pop() as u16;

            let addr = (hi << 8) | lo;
            self.registers.pc = addr;
            self.clock.m_cycles(1); // internal: set PC?
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RetInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for RetInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        cpu.execute_ret(_fetched_data, InstructionArgs::new(self.condition_type, 0, self.get_address_mode()));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
