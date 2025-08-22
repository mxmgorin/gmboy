use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_and_execute_add_sp(&mut self) {
        const R1: RegisterType = RegisterType::SP;
        self.fetch_r_d8::<{ R1 as u8 }>();
        let reg_val = self.registers.read_register::<{ R1 as u8 }>();

        self.clock.tick_m_cycles(2);
        let reg_val_u32 =
            self.registers
                .read_register::<{ R1 as u8 }>()
                .wrapping_add(self.step_ctx.fetched_data.value as i8 as u16) as u32;

        let h = (reg_val & 0xF) + (self.step_ctx.fetched_data.value & 0xF) >= 0x10;
        let c = (reg_val & 0xFF) + (self.step_ctx.fetched_data.value & 0xFF) >= 0x100;

        self.registers.flags.set_z(false);
        self.registers.flags.set_h(h);
        self.registers.flags.set_c(c);

        self.registers.flags.set_n(false);
        self.registers
            .set_register::<{ R1 as u8 }>((reg_val_u32 & 0xFFFF) as u16);
    }
    #[inline(always)]
    pub fn fetch_and_execute_add_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.execute_add::<R1>();
    }

    #[inline(always)]
    pub fn fetch_and_execute_add_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr::<R1, R2>();
        self.execute_add::<R1>();
    }

    #[inline(always)]
    pub fn fetch_and_execute_add_r_d8<const R1: u8>(&mut self) {
        self.fetch_r_d8::<R1>();
        self.execute_add::<R1>();
    }

    #[inline(always)]
    pub fn execute_add<const R1: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        let reg_val = self.registers.read_register::<R1>();
        let reg_val_u32: u32 = reg_val as u32 + self.step_ctx.fetched_data.value as u32;

        if r1.is_16bit() {
            // but not for SP
            self.clock.tick_m_cycles(1);
            let h = (reg_val & 0xFFF) + (self.step_ctx.fetched_data.value & 0xFFF) >= 0x1000;
            let n = (reg_val as u32) + (self.step_ctx.fetched_data.value as u32);
            let c = n >= 0x10000;

            self.registers.flags.set_h(h);
            self.registers.flags.set_c(c);
        } else {
            let z = (reg_val_u32 & 0xFF) == 0;
            let h = (reg_val & 0xF) + (self.step_ctx.fetched_data.value & 0xF) >= 0x10;
            let c = ((reg_val as i32) & 0xFF) + ((self.step_ctx.fetched_data.value as i32) & 0xFF)
                >= 0x100;

            self.registers.flags.set_z(z);
            self.registers.flags.set_h(h);
            self.registers.flags.set_c(c);
        }

        self.registers.flags.set_n(false);
        self.registers
            .set_register::<R1>((reg_val_u32 & 0xFFFF) as u16);
    }
}
