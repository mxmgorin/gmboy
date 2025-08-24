use crate::cpu::flags::LazyFlags;
use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_adc_r_d8(&mut self) {
        self.fetch_r_d8::<{ RegisterType::A as u8 }>();
        self.execute_adc();
    }

    #[inline(always)]
    pub fn fetch_execute_adc_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.execute_adc();
    }

    #[inline(always)]
    pub fn fetch_execute_adc_r_mr(&mut self) {
        self.fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>();

        self.execute_adc();
    }

    #[inline(always)]
    pub fn execute_adc(&mut self) {
        let lhs = self.registers.a;
        let rhs = self.step_ctx.fetched_data.value as u8;
        let carry_in = self.registers.flags.get_c() as u8;

        let result = lhs.wrapping_add(rhs).wrapping_add(carry_in);
        self.registers.a = result;
        self.registers.flags.set_lazy(LazyFlags::Add8 {
            lhs,
            rhs,
            carry_in,
            result,
        });
    }
}
