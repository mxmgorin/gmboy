
use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_sub_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.execute_sub::<R1>();
    }

    #[inline(always)]
    pub fn fetch_execute_sub_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr::<R1, R2>();
        self.execute_sub::<R1>();
    }

    #[inline(always)]
    pub fn fetch_execute_sub_r_d8<const R1: u8>(&mut self) {
        self.fetch_r_d8::<R1>();
        self.execute_sub::<R1>();
    }

    pub fn execute_sub<const R1: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        let reg_val = self.registers.read_register(r1);
        let result = reg_val.wrapping_sub(self.step_ctx.fetched_data.value);

        let reg_val_i32 = reg_val as i32;
        let fetched_val_i32 = result as i32;

        let h = ((reg_val_i32 & 0xF).wrapping_sub(fetched_val_i32 & 0xF)) < 0;
        let c = reg_val_i32.wrapping_sub(fetched_val_i32) < 0;

        self.registers.set_register(r1, result);

        self.registers.flags.set_z(result == 0);
        self.registers.flags.set_n(true);
        self.registers.flags.set_h(h);
        self.registers.flags.set_c(c);
    }
}
