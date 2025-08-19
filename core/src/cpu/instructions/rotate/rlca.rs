use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_rlca(&mut self, _fetched_data: FetchedData, _spec: InstructionSpec) {
        let mut u: u8 = self.registers.a;
        let c: bool = (u >> 7) & 1 != 0;
        u = (u << 1) | c as u8;
        self.registers.a = u;

        self.registers.flags.set_z(false);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(c);
    }
}
