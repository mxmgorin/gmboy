use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_scf(&mut self, _fetched_data: FetchedData, _spec: InstructionSpec) {
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(true);
    }
}
