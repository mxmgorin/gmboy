use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_and(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        self.registers.a &= fetched_data.value as u8;

        self.registers.flags.set_z(self.registers.a == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(true);
        self.registers.flags.set_c(false);
    }
}
