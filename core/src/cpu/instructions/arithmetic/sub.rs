use crate::cpu::flags::{Flags, FlagsCtx, FlagsData, FlagsOp};
use crate::cpu::Cpu;

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
        self.fetch_r_d8();
        self.execute_sub::<R1>();
    }

    pub fn execute_sub<const R1: u8>(&mut self) {
        let lhs = self.registers.get_register8::<R1>();
        let rhs = self.step_ctx.fetched_data.value as u8;
        let result = lhs.wrapping_sub(rhs);
        self.registers.set_register8::<R1>(result);
        self.registers
            .flags
            .set(FlagsCtx::sub8(lhs, rhs, 0, result));
    }
}

impl FlagsOp {
    pub fn sub8(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(data.result == 0);
        flags.set_n_raw(true);
        flags.set_h_raw((data.lhs as u8 & 0xF) < ((data.rhs as u8 & 0xF) + data.carry_in));
        flags.set_c_raw((data.lhs) < (data.rhs + data.carry_in as u16));
    }
}
