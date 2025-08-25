use crate::cpu::flags::{Flags, FlagsCtx, FlagsCtxData, FlagsOp};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_rla(&mut self) {
        let lhs = self.registers.a;
        let carry_in = self.registers.flags.get_c() as u8;
        self.registers.a = (lhs << 1) | carry_in;

        self.registers.flags.set(FlagsCtx::rla(lhs));
    }
}

impl FlagsOp {
    #[inline(always)]
    pub fn rla(data: FlagsCtxData, flags: &mut Flags) {
        flags.set_z_inner(false);
        flags.set_n_inner(false);
        flags.set_h_inner(false);
        flags.set_c_inner(((data.lhs >> 7) & 1) != 0);
    }
}
