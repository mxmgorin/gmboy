use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_and_execute_adc_r_d8(&mut self) {
        self.fetch_r_d8::<{ RegisterType::A as u8 }>();
        self.execute_adc();
    }

    #[inline(always)]
    pub fn fetch_and_execute_adc_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.execute_adc();
    }

    #[inline(always)]
    pub fn fetch_and_execute_adc_r_mr(&mut self) {
            self.fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>();

        self.execute_adc();
    }

    #[inline(always)]
    pub fn execute_adc(&mut self) {
        let u: u16 = self.step_ctx.fetched_data.value;
        let a: u16 = self.registers.a as u16;
        let c: u16 = self.registers.flags.get_c() as u16;

        self.registers.a = ((a + u + c) & 0xFF) as u8;

        self.registers.flags.set_z(self.registers.a == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h((a & 0xF) + (u & 0xF) + c > 0xF);
        self.registers.flags.set_c(a + u + c > 0xFF);
    }
}
