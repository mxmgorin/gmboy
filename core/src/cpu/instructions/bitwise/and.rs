use crate::cpu::flags::{Flags, FlagsData, FlagsOp};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_and_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.execute_and();
    }

    #[inline(always)]
    pub fn fetch_execute_and_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr::<R1, R2>();
        self.execute_and();
    }

    #[inline(always)]
    pub fn fetch_execute_and_r_d8<const R1: u8>(&mut self) {
        self.fetch_r_d8();
        self.execute_and();
    }

    #[inline(always)]
    pub fn execute_and(&mut self) {
        self.registers.a &= self.step_ctx.fetched_data.value as u8;
        self.registers.flags.op_and(self.registers.a);
    }
}

impl FlagsOp {
    #[inline(always)]
    pub fn and(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(data.result == 0);
        flags.set_n_raw(false);
        flags.set_h_raw(true);
        flags.set_c_raw(false);
    }
}
