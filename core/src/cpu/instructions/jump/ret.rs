use crate::cpu::instructions::InstructionSpec;
use crate::cpu::instructions::{ConditionType, FetchedData};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_ret(&mut self, _fetched_data: FetchedData, spec: InstructionSpec) {
        if spec.cond_type.is_some() {
            self.clock.m_cycles(1); // internal: branch decision?
        }

        if ConditionType::check_cond(&self.registers, spec.cond_type) {
            let lo = self.pop() as u16;
            let hi = self.pop() as u16;

            let addr = (hi << 8) | lo;
            self.registers.pc = addr;
            self.clock.m_cycles(1); // internal: set PC?
        }
    }
}
