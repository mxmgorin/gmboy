use crate::cpu::instructions::{FetchedData};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_scf(&mut self, _fetched_data: FetchedData) {
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(true);
    }
}
