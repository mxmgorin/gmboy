use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    /// ComPLement accumulator (A = ~A); also called bitwise NOT.
    /// Cycles: 1
    /// Bytes: 1
    /// Flags:
    /// N 1
    /// H 1
    #[inline]
    pub fn execute_cpl(&mut self, _fetched_data: FetchedData, _spec: InstructionSpec) {
        self.registers.a = !self.registers.a;

        self.registers.flags.set_n(true);
        self.registers.flags.set_h(true);
    }
}
