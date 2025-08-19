use crate::cpu::instructions::{FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_or(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        let value = fetched_data.value & 0xFF;
        self.registers.a |= value as u8;

        self.registers.flags.set_z(self.registers.a == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(false);
    }
}
