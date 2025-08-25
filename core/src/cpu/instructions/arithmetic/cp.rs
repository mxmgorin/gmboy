use crate::cpu::flags::FlagsCtx;
use crate::cpu::Cpu;
use crate::cpu::instructions::arithmetic::sub::Sub8FlagsCtx;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_cp_r_d8<const R1: u8>(&mut self) {
        self.fetch_r_d8::<R1>();
        self.execute_cp();
    }

    #[inline(always)]
    pub fn fetch_execute_cp_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr::<R1, R2>();
        self.execute_cp();
    }

    #[inline(always)]
    pub fn fetch_execute_cp_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.execute_cp();
    }

    #[inline(always)]
    pub fn execute_cp(&mut self) {
        let lhs = self.registers.a;
        let rhs = self.step_ctx.fetched_data.value as u8;
        let result = lhs.wrapping_sub(rhs);
        self.registers.flags.set(FlagsCtx::Sub8(Sub8FlagsCtx {
            lhs,
            rhs,
            carry_in: 0,
            result,
        }));
    }
}
