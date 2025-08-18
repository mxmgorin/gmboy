use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_nop(&mut self, _fetched_data: FetchedData, _args: InstructionSpec) {
        // does nothing
    }
}
