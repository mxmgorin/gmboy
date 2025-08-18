use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_rst(&mut self, _fetched_data: FetchedData, spec: InstructionSpec) {
        self.goto_addr(None, spec.addr, true);
    }
}
