use crate::cpu::flags::FlagsCtx;
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

        // todo: for some reason fails test when lazy is used
        self.registers
            .flags
            .force_set(FlagsCtx::or(self.registers.a));
    }
}
