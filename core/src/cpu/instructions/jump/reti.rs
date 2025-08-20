
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_reti(&mut self) {
        self.clock.bus.io.interrupts.ime = true;
        self.execute_ret(None);
    }
}
