
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_xor(&mut self) {
        self.registers.a ^= (self.step_ctx.fetched_data.value & 0xFF) as u8;

        self.registers.flags.set_z(self.registers.a == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(false);
    }
}
