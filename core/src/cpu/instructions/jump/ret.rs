use crate::cpu::instructions::JumpCondition;
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_ret<const C: u8>(&mut self) {
        if JumpCondition::from_u8(C) != JumpCondition::None {
            self.clock.tick_m_cycles(1);
        }

        if self.check_cond::<C>() {
            let lo = self.pop() as u16;
            let hi = self.pop() as u16;

            let addr = (hi << 8) | lo;
            self.registers.pc = addr;
            self.clock.tick_m_cycles(1); // internal: set PC?
        }
    }
}
