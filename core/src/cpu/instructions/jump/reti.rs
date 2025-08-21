use crate::cpu::instructions::ConditionType;
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_reti(&mut self) {
        self.clock.bus.io.interrupts.ime = true;
        self.execute_ret::<{ ConditionType::None as u8 }>();
    }
}
