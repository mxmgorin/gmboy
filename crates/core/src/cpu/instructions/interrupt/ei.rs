use crate::cpu::Cpu;

impl Cpu {
    /// Enable Interrupts by setting the IME flag.
    /// The flag is only set after the instruction following EI.
    #[inline(always)]
    pub fn execute_ei(&mut self) {
        self.enabling_ime = true;
    }
}
