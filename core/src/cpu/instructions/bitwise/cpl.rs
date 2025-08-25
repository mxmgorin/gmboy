use crate::cpu::flags::{Flags, FlagsCtx, FlagsCtxData, FlagsOp};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_cpl(&mut self) {
        self.registers.a = !self.registers.a;

        self.registers.flags.set(FlagsCtx::cpl())
    }
}

impl FlagsOp {
    pub fn cpl(_data: FlagsCtxData, flags: &mut Flags) {
        flags.set_n_inner(true);
        flags.set_h_inner(true);
    }
}
