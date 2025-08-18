use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_call(&mut self, fetched_data: FetchedData, spec: InstructionSpec) {
        self.goto_addr(spec.cond_type, fetched_data.value, true);
    }
}
