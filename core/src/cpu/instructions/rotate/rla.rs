use crate::cpu::flags::{Flags, FlagsCtx};
use crate::cpu::Cpu;
use serde::{Deserialize, Serialize};

impl Cpu {
    #[inline(always)]
    pub fn execute_rla(&mut self) {
        let lhs = self.registers.a;
        let carry_in = self.registers.flags.get_c() as u8;
        self.registers.a = (lhs << 1) | carry_in;

        self.registers.flags.set(FlagsCtx::Rla(RlaFlagsCtx { lhs }));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlaFlagsCtx {
    pub lhs: u8,
}

impl RlaFlagsCtx {
    #[inline(always)]
    pub fn apply(&self, flags: &mut Flags) {
        flags.set_z_inner(false);
        flags.set_n_inner(false);
        flags.set_h_inner(false);
        flags.set_c_inner(((self.lhs >> 7) & 1) != 0);
    }
}
