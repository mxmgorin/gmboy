use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_rrca(&mut self, _fetched_data: FetchedData, _args: InstructionSpec) {
        let b: u8 = self.registers.a & 1;
        self.registers.a >>= 1;
        self.registers.a |= b << 7;

        self.registers.flags.set_z(false);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(b != 0);
    }
}
