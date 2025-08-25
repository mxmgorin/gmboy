use crate::cpu::flags::{Flags, FlagsCtx};
use crate::cpu::Cpu;
use serde::{Deserialize, Serialize};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_or_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.execute_or();
    }

    #[inline(always)]
    pub fn fetch_execute_or_r_d8<const R1: u8>(&mut self) {
        self.fetch_r_d8::<R1>();
        self.execute_or();
    }

    #[inline(always)]
    pub fn fetch_execute_or_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr::<R1, R2>();
        self.execute_or();
    }

    #[inline(always)]
    pub fn execute_or(&mut self) {
        let lhs = self.step_ctx.fetched_data.value;
        let result = self.registers.a | lhs as u8;
        self.registers.a = result;

        self.registers
            .flags
            .set(FlagsCtx::Or(OrFlagsCtx { result }))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct OrFlagsCtx {
    pub result: u8,
}

impl OrFlagsCtx {
    #[inline(always)]
    pub fn apply(&self, flags: &mut Flags) {
        flags.set_z_inner(self.result == 0);
        flags.set_n_inner(false);
        flags.set_h_inner(false);
        flags.set_c_inner(false);
    }
}
