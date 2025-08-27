use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_adc_r_d8(&mut self) {
        let value = self.read_pc();
        self.execute_adc(value);
    }

    #[inline(always)]
    pub fn fetch_execute_adc_r_r<const R1: u8, const R2: u8>(&mut self) {
        let value = self.registers.get_register8::<R2>();
        self.execute_adc(value);
    }

    #[inline(always)]
    pub fn fetch_execute_adc_r_mr(&mut self) {
        let value = self.read_mr::<{ RegisterType::HL as u8 }>();
        self.execute_adc(value);
    }

    #[inline(always)]
    pub fn execute_adc(&mut self, value: u8) {
        let lhs = self.registers.a;
        let carry_in = self.registers.flags.get_c() as u8;

        let result = lhs.wrapping_add(value).wrapping_add(carry_in);
        self.registers.a = result;
        self.registers.flags.op_add8(lhs, value, carry_in, result);
    }
}
