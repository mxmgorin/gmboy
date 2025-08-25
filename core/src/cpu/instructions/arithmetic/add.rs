use crate::cpu::flags::{Flags, FlagsCtx, FlagsCtxData, FlagsOp};
use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_add_sp_e8(&mut self) {
        self.fetch_r_d8::<{ RegisterType::SP as u8 }>();
        let lhs = self.registers.sp;
        let rhs = self.step_ctx.fetched_data.value;

        self.clock.tick_m_cycles(2);
        self.registers.sp = self.registers.sp.wrapping_add(rhs as i8 as u16);
        self.registers.flags.set(FlagsCtx::add_sp_e8(lhs, rhs));
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
        if RegisterType::from_u8(R1).is_16bit() {
            // but not for SP
            let lhs = self.registers.get_register::<R1>();
            let rhs = self.step_ctx.fetched_data.value;
            let result = lhs.wrapping_add(rhs);
            self.registers.set_register::<R1>(result);
            self.clock.tick_m_cycles(1);
            self.registers.flags.set(FlagsCtx::add16(lhs, rhs));
        } else {
            let lhs = self.registers.get_register8::<R1>();
            let rhs = self.step_ctx.fetched_data.value as u8;
            let result = lhs.wrapping_add(rhs);
            self.registers.set_register8::<R1>(result);
            self.registers
                .flags
                .set(FlagsCtx::add8(lhs, rhs, 0, result));
        }
    }
}

impl FlagsOp {
    #[inline(always)]
    pub fn add8(data: FlagsCtxData, flags: &mut Flags) {
        flags.set_z_inner(data.result == 0);
        flags.set_n_inner(false);
        flags.set_h_inner((data.lhs as u8 & 0xF) + (data.rhs as u8 & 0xF) + data.carry_in > 0xF);
        flags.set_c_inner((data.lhs + data.rhs + data.carry_in as u16) > 0xFF);
    }

    #[inline(always)]
    pub fn add16(data: FlagsCtxData, flags: &mut Flags) {
        flags.set_n_inner(false);
        flags.set_h_inner(((data.lhs & 0x0FFF) + (data.rhs & 0x0FFF)) > 0x0FFF);
        flags.set_c_inner((data.lhs as u32 + data.rhs as u32) > 0xFFFF);
    }

    #[inline(always)]
    pub fn add_sp_e8(data: FlagsCtxData, flags: &mut Flags) {
        flags.set_z_inner(false);
        flags.set_n_inner(false);
        flags.set_h_inner((data.lhs & 0xF) + (data.rhs & 0xF) > 0xF);
        flags.set_c_inner(((data.lhs & 0xFF) + (data.rhs & 0xFF)) > 0xFF);
    }
}
