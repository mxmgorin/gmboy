use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    /// Disable Interrupts by clearing the IME flag.
    #[inline]
    pub fn execute_di(&mut self, _fetched_data: FetchedData, _spec: InstructionSpec) {
        self.clock.bus.io.interrupts.ime = false;
    }
}
