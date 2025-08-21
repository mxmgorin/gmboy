
use crate::cpu::Cpu;
use crate::cpu::instructions::ConditionType;

impl Cpu {
    #[inline]
    pub fn execute_reti(&mut self) {
        self.clock.bus.io.interrupts.ime = true;
        self.execute_ret(ConditionType::None);
    }
}
