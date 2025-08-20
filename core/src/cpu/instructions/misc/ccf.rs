
use crate::cpu::Cpu;

impl Cpu {
    /// Complement Carry Flag.
    /// Cycles: 1
    /// Bytes: 1
    /// Flags:
    /// N 0
    /// H 0
    /// C Inverted#[inline]
    pub fn execute_ccf(&mut self) {
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(!self.registers.flags.get_c());
    }
}
