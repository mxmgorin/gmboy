use crate::cpu::flags::{Flags, FlagsCtx};
use crate::cpu::Cpu;
use serde::{Deserialize, Serialize};

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
        self.fetch_r_d8::<R1>();
        self.execute_sub::<R1>();
    }

    pub fn execute_sub<const R1: u8>(&mut self) {
        let lhs = self.registers.get_register8::<R1>();
        let rhs = self.step_ctx.fetched_data.value as u8;
        let result = lhs.wrapping_sub(rhs);
        self.registers.set_register8::<R1>(result);
        self.registers.flags.set(FlagsCtx::Sub8(Sub8FlagsCtx {
            lhs,
            rhs,
            carry_in: 0,
            result,
        }));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sub8FlagsCtx {
    pub lhs: u8,
    pub rhs: u8,
    pub carry_in: u8,
    pub result: u8,
}

impl Sub8FlagsCtx {
    #[inline(always)]
    pub fn apply(&self, flags: &mut Flags) {
        flags.set_z_inner(self.result == 0);
        flags.set_n_inner(true);
        flags.set_h_inner((self.lhs & 0xF) < ((self.rhs & 0xF) + self.carry_in));
        flags.set_c_inner((self.lhs as u16) < (self.rhs as u16 + self.carry_in as u16));
    }
}
