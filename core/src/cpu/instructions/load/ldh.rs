use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_ldh_a8_r<const R2: u8>(&mut self) {
        self.fetch_a8_r::<R2>();
        self.write_to_memory(
            self.step_ctx.fetched_data.addr | 0xFF00,
            self.step_ctx.fetched_data.value as u8,
        );
    }

    #[inline(always)]
    pub fn fetch_execute_ldh_mr_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_mr_r::<R1, R2>();
        self.write_to_memory(
            self.step_ctx.fetched_data.addr | 0xFF00,
            self.step_ctx.fetched_data.value as u8,
        );
    }

    #[inline(always)]
    pub fn fetch_execute_ldh_r_ha8<const R1: u8>(&mut self) {
        let data = self.read_ha8();
        self.registers.set_register8::<R1>(data);
    }

    #[inline(always)]
    pub fn fetch_execute_ldh_r_hmr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_hmr::<R1, R2>();
        self.registers
            .set_register::<R1>(self.step_ctx.fetched_data.value);
    }
}
