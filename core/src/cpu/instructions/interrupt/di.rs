use crate::cpu::instructions::{FetchedData};
use crate::cpu::Cpu;

impl Cpu {
    /// Disable Interrupts by clearing the IME flag.
    #[inline]
    pub fn execute_di(&mut self, _fetched_data: FetchedData) {
        self.clock.bus.io.interrupts.ime = false;
    }
}
