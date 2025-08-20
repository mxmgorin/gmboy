
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_xor_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.execute_xor();
    }

    #[inline(always)]
    pub fn fetch_execute_xor_r_d8<const R1: u8>(&mut self) {
        self.fetch_r_d8::<R1>();
        self.execute_xor();
    }

    #[inline(always)]
    pub fn fetch_execute_xor_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr::<R1, R2>();
        self.execute_xor();
    }

    #[inline(always)]
    pub fn execute_xor(&mut self) {
        self.registers.a ^= (self.step_ctx.fetched_data.value & 0xFF) as u8;

        self.registers.flags.set_z(self.registers.a == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(false);
    }
}
