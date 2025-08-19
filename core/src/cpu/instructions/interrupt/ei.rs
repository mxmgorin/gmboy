use crate::cpu::instructions::{FetchedData};
use crate::cpu::Cpu;

impl Cpu {
    /// Enable Interrupts by setting the IME flag.
    /// The flag is only set after the instruction following EI.#[inline]
    pub fn execute_ei(&mut self, _fetched_data: FetchedData) {
        self.enabling_ime = true;
    }
}
