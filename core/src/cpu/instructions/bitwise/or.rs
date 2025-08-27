use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_or_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.execute_or();
    }

    #[inline(always)]
    pub fn fetch_execute_or_r_d8<const R1: u8>(&mut self) {
        self.fetch_r_d8();
        self.execute_or();
    }

    #[inline(always)]
    pub fn fetch_execute_or_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr::<R1, R2>();
        self.execute_or();
    }

    #[inline(always)]
    pub fn execute_or(&mut self) {
        self.registers.a |= self.step_ctx.fetched_data.value as u8;
        self.registers.flags.op_or(self.registers.a)
    }
}
