
use crate::cpu::instructions::{ConditionType};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_ret_no(&mut self) {
        self.execute_ret(None);
    }

    #[inline]
    pub fn execute_ret_z(&mut self) {
        self.execute_ret(Some(ConditionType::Z));
    }

    #[inline]
    pub fn execute_ret_nc(&mut self) {
        self.execute_ret(Some(ConditionType::NC));
    }

    #[inline]
    pub fn execute_ret_c(&mut self) {
        self.execute_ret(Some(ConditionType::C));
    }

    #[inline]
    pub fn execute_ret_nz(&mut self) {
        self.execute_ret(Some(ConditionType::NZ));
    }

    #[inline(always)]
    pub fn execute_ret(&mut self, cond: Option<ConditionType>) {
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
