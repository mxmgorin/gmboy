use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_reti(&mut self, _fetched_data: FetchedData, spec: InstructionSpec) {
        self.clock.bus.io.interrupts.ime = true;
        self.execute_ret(spec.cond_type);
    }
}
