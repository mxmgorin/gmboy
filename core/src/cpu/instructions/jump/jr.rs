use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_jr(&mut self, fetched_data: FetchedData, spec: InstructionSpec) {
        let rel = fetched_data.value as i8;
        let addr = (self.registers.pc as i32).wrapping_add(rel as i32);
        self.goto_addr(spec.cond_type, addr as u16, false);
    }
}
