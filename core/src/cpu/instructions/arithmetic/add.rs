use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_add_sp_e8(&mut self) {
        self.fetch_r_d8();
        let lhs = self.registers.sp;
        let rhs = self.step_ctx.fetched_data.value;

        self.clock.tick_m_cycles(2);
        self.registers.sp = self.registers.sp.wrapping_add(rhs as i8 as u16);
        self.registers.flags.op_add_sp_e8(lhs, rhs);
    }

    #[inline(always)]
    pub fn fetch_execute_add_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.execute_add::<R1>();
    }

    #[inline(always)]
    pub fn fetch_execute_add_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr::<R1, R2>();
        self.execute_add::<R1>();
    }

    #[inline(always)]
    pub fn fetch_execute_add_r_d8<const R1: u8>(&mut self) {
        self.fetch_r_d8();
        self.execute_add::<R1>();
    }

    #[inline(always)]
    pub fn execute_add<const R1: u8>(&mut self) {
        if RegisterType::from_u8(R1).is_16bit() {
            // but not for SP
            let lhs = self.registers.get_register::<R1>();
            let rhs = self.step_ctx.fetched_data.value;
            let result = lhs.wrapping_add(rhs);
            self.registers.set_register::<R1>(result);
            self.clock.tick_m_cycles(1);
            self.registers.flags.op_add16(lhs, rhs);
        } else {
            let lhs = self.registers.get_register8::<R1>();
            let rhs = self.step_ctx.fetched_data.value as u8;
            let result = lhs.wrapping_add(rhs);
            self.registers.set_register8::<R1>(result);
            self.registers
                .flags
                .op_add8(lhs, rhs, 0, result);
        }
    }
}
