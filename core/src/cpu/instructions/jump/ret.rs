use crate::cpu::instructions::InstructionSpec;
use crate::cpu::instructions::{ConditionType, FetchedData};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_ret_no(&mut self, _fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_ret_inner(None);
    }

    #[inline]
    pub fn execute_ret_z(&mut self, _fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_ret_inner(Some(ConditionType::Z));
    }

    #[inline]
    pub fn execute_ret_nc(&mut self, _fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_ret_inner(Some(ConditionType::NC));
    }

    #[inline]
    pub fn execute_ret_c(&mut self, _fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_ret_inner(Some(ConditionType::C));
    }

    #[inline]
    pub fn execute_ret_nz(&mut self, _fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_ret_inner(Some(ConditionType::NZ));
    }
    
    #[inline]
    pub fn execute_ret(&mut self, _fetched_data: FetchedData, spec: InstructionSpec) {
        self.execute_ret_inner(spec.cond_type);
    }

    #[inline(always)]
    fn execute_ret_inner(&mut self, cond: Option<ConditionType>) {
        if cond.is_some() {
            self.clock.m_cycles(1); // internal: branch decision?
        }

        if ConditionType::check_cond(&self.registers, cond) {
            let lo = self.pop() as u16;
            let hi = self.pop() as u16;

            let addr = (hi << 8) | lo;
            self.registers.pc = addr;
            self.clock.m_cycles(1); // internal: set PC?
        }
    }
}
