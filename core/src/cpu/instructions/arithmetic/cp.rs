use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_cp_r_d8<const R1: u8>(&mut self) {
        self.step_ctx.fetched_data = self.fetch_r_d8::<R1>();
        self.execute_cp();
    }

    #[inline(always)]
    pub fn fetch_execute_cp_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.step_ctx.fetched_data = self.fetch_r_mr::<R1, R2>();
        self.execute_cp();
    }

    #[inline(always)]
    pub fn fetch_execute_cp_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.step_ctx.fetched_data = self.fetch_r_r::<R1, R2>();
        self.execute_cp();
    }

    #[inline(always)]
    pub fn execute_cp(&mut self) {
        let fetched_value_i32 = self.step_ctx.fetched_data.value as i32;
        let reg_i32 = self.registers.a as i32;
        let result: i32 = reg_i32.wrapping_sub(fetched_value_i32);
        let reg_value_diff = (reg_i32 & 0x0F) - (fetched_value_i32 & 0x0F);

        self.registers.flags.set_z(result == 0);
        self.registers.flags.set_n(true);
        self.registers.flags.set_h(reg_value_diff < 0);
        self.registers.flags.set_c(result < 0);
    }
}
