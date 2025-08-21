use crate::cpu::instructions::ConditionType;
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_ret_no(&mut self) {
        self.execute_ret(ConditionType::None);
    }

    #[inline]
    pub fn execute_ret_z(&mut self) {
        self.execute_ret(ConditionType::Z);
    }

    #[inline]
    pub fn execute_ret_nc(&mut self) {
        self.execute_ret(ConditionType::NC);
    }

    #[inline]
    pub fn execute_ret_c(&mut self) {
        self.execute_ret(ConditionType::C);
    }

    #[inline]
    pub fn execute_ret_nz(&mut self) {
        self.execute_ret(ConditionType::NZ);
    }

    #[inline(always)]
    pub fn execute_ret(&mut self, cond: ConditionType) {
        if cond != ConditionType::None {
            self.clock.m_cycles(1);
        }

        if self.check_cond(cond) {
            let lo = self.pop() as u16;
            let hi = self.pop() as u16;

            let addr = (hi << 8) | lo;
            self.registers.pc = addr;
            self.clock.m_cycles(1); // internal: set PC?
        }
    }
}
