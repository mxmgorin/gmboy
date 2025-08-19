use crate::cpu::instructions::{FetchedData};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_rla(&mut self, _fetched_data: FetchedData) {
        let u: u8 = self.registers.a;
        let cf: u8 = self.registers.flags.get_c() as u8;
        let c: u8 = (u >> 7) & 1;

        self.registers.a = (u << 1) | cf;

        self.registers.flags.set_z(false);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(c != 0);
    }
}
