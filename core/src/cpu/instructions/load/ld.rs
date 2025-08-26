use crate::cpu::flags::{Flags, FlagsCtx, FlagsData, FlagsOp};
use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_ld_lh_spi8(&mut self) {
        self.fetch_lh_spi8();
        let lhs = self.step_ctx.fetched_data.value;
        let rhs = self.registers.sp;

        let offset_e = lhs as i8; // truncate to 8 bits (+8e)
        let result = self.registers.sp.wrapping_add(offset_e as u16);
        self.registers
            .set_register::<{ RegisterType::HL as u8 }>(result);
        self.registers.flags.set(FlagsCtx::new_ld(lhs, rhs));

        self.clock.tick_m_cycles(1);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_d8<const R1: u8>(&mut self) {
        let rhs = self.read_pc();
        self.registers.set_register8::<R1>(rhs);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_d16<const R1: u8>(&mut self) {
        let rhs = self.read_pc16();
        self.registers.set_register::<R1>(rhs);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.ld_r_r::<R1, R2>();
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr::<R1, R2>();
        self.ld_r_r::<R1, R2>();
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_mrd<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mrd::<R1, R2>();
        self.ld_r_r::<R1, R2>();
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_a16<const R1: u8>(&mut self) {
        let rhs = self.read_a16();
        self.registers.set_register8::<R1>(rhs);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_mr_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_mr_r::<R1, R2>();
        self.ld_addr_r::<R2>(self.step_ctx.fetched_data.addr);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_a16_r<const R2: u8>(&mut self) {
        self.fetch_a16_r::<R2>();
        self.ld_addr_r::<R2>(self.step_ctx.fetched_data.addr);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_mri_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_mri_r::<R1, R2>();
        self.ld_addr_r::<R2>(self.step_ctx.fetched_data.addr);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_mri<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mri::<R1, R2>();
        self.ld_r_r::<R1, R2>();
    }

    #[inline(always)]
    pub fn fetch_execute_ld_mrd_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_mrd_r::<R1, R2>();
        self.ld_addr_r::<R2>(self.step_ctx.fetched_data.addr);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_mr_d8<const R1: u8>(&mut self) {
        self.fetch_mr_d8::<R1>();
        self.write_to_memory(
            self.step_ctx.fetched_data.addr,
            self.step_ctx.fetched_data.value as u8,
        );
    }

    #[inline(always)]
    fn ld_addr_r<const R: u8>(&mut self, addr: u16) {
        if RegisterType::from_u8(R).is_16bit() {
            let value_l = ((self.step_ctx.fetched_data.value >> 8) & 0xFF) as u8;
            self.write_to_memory(addr + 1, value_l);
            self.write_to_memory(addr, (self.step_ctx.fetched_data.value & 0xFF) as u8);
        } else {
            self.write_to_memory(addr, self.step_ctx.fetched_data.value as u8);
        }
    }

    #[inline(always)]
    fn ld_r_r<const R1: u8, const R2: u8>(&mut self) {
        if RegisterType::from_u8(R1).is_16bit() && RegisterType::from_u8(R2).is_16bit() {
            self.clock.tick_m_cycles(1);
        }

        self.registers
            .set_register::<R1>(self.step_ctx.fetched_data.value);
    }
}

impl FlagsOp {
    #[inline(always)]
    pub fn ld(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(false);
        flags.set_n_raw(false);
        flags.set_h_raw((data.lhs & 0xF) + (data.rhs & 0xF) >= 0x10);
        flags.set_c_raw((data.lhs & 0xFF) + (data.rhs & 0xFF) >= 0x100);
    }
}
