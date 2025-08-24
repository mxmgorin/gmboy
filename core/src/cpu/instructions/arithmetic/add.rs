use crate::cpu::{Cpu, RegisterType};
use crate::cpu::flags::LazyFlags;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_add_sp(&mut self) {
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
        self.fetch_r_d8::<R1>();
        self.execute_add::<R1>();
    }

    #[inline(always)]
    pub fn execute_add<const R1: u8>(&mut self) {
        let lhs = self.registers.read_register::<R1>();
        let rhs = self.step_ctx.fetched_data.value;

        if RegisterType::from_u8(R1).is_16bit() {
            // but not for SP
            let result: u32 = lhs as u32 + rhs as u32;
            self.registers.set_register::<R1>((result & 0xFFFF) as u16);
            self.clock.tick_m_cycles(1);
            let h = (lhs & 0xFFF) + (self.step_ctx.fetched_data.value & 0xFFF) >= 0x1000;
            let n = (lhs as u32) + (self.step_ctx.fetched_data.value as u32);
            let c = n >= 0x10000;

            self.registers.flags.set_h(h);
            self.registers.flags.set_c(c);
            self.registers.flags.set_n(false);
        } else {
            let result = lhs + rhs;
            self.registers.set_register::<R1>(result);
            self.registers.flags.set_lazy(LazyFlags::Add8 {
                lhs: lhs as u8,
                rhs: rhs as u8,
                carry_in: 0,
                result: result as u8,
            });
        }
    }
}
