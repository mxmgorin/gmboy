use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_sbc_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.execute_sbc::<R1>();
    }

    #[inline(always)]
    pub fn fetch_execute_sbc_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr::<R1, R2>();
        self.execute_sbc::<R1>();
    }

    #[inline(always)]
    pub fn fetch_execute_sbc_r_d8<const R1: u8>(&mut self) {
        self.fetch_r_d8::<R1>();
        self.execute_sbc::<R1>();
    }

    #[inline(always)]
    pub fn execute_sbc<const R1: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        let c_val = self.registers.flags.get_c();
        let val_plus_c = self.step_ctx.fetched_data.value.wrapping_add(c_val as u16) as u8;
        let r_val = self.registers.read_register(r1);

        let c_val_i32 = c_val as i32;
        let r_val_i32 = r_val as i32;
        let fetched_val_i32 = self.step_ctx.fetched_data.value as i32;

        let h = (r_val_i32 & 0xF)
            .wrapping_sub(fetched_val_i32 & 0xF)
            .wrapping_sub(c_val_i32)
            < 0;
        let c = r_val_i32
            .wrapping_sub(fetched_val_i32)
            .wrapping_sub(c_val_i32)
            < 0;

        let result = r_val.wrapping_sub(val_plus_c as u16);

        self.registers.set_register(r1, result);

        self.registers.flags.set_z(result == 0);
        self.registers.flags.set_n(true);
        self.registers.flags.set_h(h);
        self.registers.flags.set_c(c);
    }
}
