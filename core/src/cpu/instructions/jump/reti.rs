use crate::cpu::instructions::{FetchedData};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_reti(&mut self, _fetched_data: FetchedData) {
        self.clock.bus.io.interrupts.ime = true;
        self.execute_ret(None);
    }
}
