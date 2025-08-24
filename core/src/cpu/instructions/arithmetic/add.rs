use crate::cpu::flags::LazyFlags;
use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_add_sp_e8(&mut self) {
        self.fetch_r_d8::<{ RegisterType::SP as u8 }>();
        let lhs = self.registers.sp;
        let rhs = self.step_ctx.fetched_data.value;

        self.clock.tick_m_cycles(2);
        self.registers.sp = self.registers.sp.wrapping_add(rhs as i8 as u16);
        self.registers
            .flags
            .set_lazy(LazyFlags::AddSpE8 { lhs, rhs });
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
        let lhs = self.registers.get_register::<R1>();
        let rhs = self.step_ctx.fetched_data.value;

        if RegisterType::from_u8(R1).is_16bit() {
            // but not for SP
            let result = lhs.wrapping_add(rhs);
            self.registers.set_register::<R1>(result);
            self.clock.tick_m_cycles(1);
            self.registers.flags.set_lazy(LazyFlags::Add16 { lhs, rhs });
        } else {
            let result = lhs.wrapping_add(rhs);
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
