use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_reti(&mut self, fetched_data: FetchedData, spec: InstructionSpec) {
        self.clock.bus.io.interrupts.ime = true;
        self.execute_ret(fetched_data, spec);
    }
}
