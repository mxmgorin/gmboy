use crate::cpu::instructions::JumpCondition;
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_reti(&mut self) {
        self.clock.bus.io.interrupts.ime = true;
        self.execute_ret::<{ JumpCondition::None as u8 }>();
    }
}
