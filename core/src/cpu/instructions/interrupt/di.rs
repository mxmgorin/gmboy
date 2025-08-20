
use crate::cpu::Cpu;

impl Cpu {
    /// Disable Interrupts by clearing the IME flag.
    #[inline]
    pub fn execute_di(&mut self) {
        self.clock.bus.io.interrupts.ime = false;
    }
}
