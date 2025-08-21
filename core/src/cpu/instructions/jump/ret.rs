use crate::cpu::instructions::ConditionType;
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_ret<const C: u8>(&mut self) {
        let cond = ConditionType::from_u8(C);
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
