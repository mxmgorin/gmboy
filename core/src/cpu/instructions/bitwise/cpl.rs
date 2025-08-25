use crate::cpu::flags::{Flags, FlagsCtx};
use crate::cpu::Cpu;
use serde::{Deserialize, Serialize};

impl Cpu {
    #[inline(always)]
    pub fn execute_cpl(&mut self) {
        self.registers.a = !self.registers.a;

        self.registers.flags.set(FlagsCtx::Cpl(CplFlagsCtx))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct CplFlagsCtx;

impl CplFlagsCtx {
    #[inline(always)]
    pub fn apply(&self, flags: &mut Flags) {
        flags.set_n_inner(true);
        flags.set_h_inner(true);
    }
}
