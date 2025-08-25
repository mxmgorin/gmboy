use crate::cpu::flags::{Flags, FlagsCtx, FlagsData, FlagsOp};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_ccf(&mut self) {
        let carry = self.registers.flags.get_c() as u8;
        self.registers.flags.set(FlagsCtx::ccf(carry));
    }
}

impl FlagsOp {
    #[inline(always)]
    pub fn ccf(data: FlagsData, flags: &mut Flags) {
        flags.set_n_inner(false);
        flags.set_h_inner(false);
        flags.set_c_inner(data.carry_in == 0);
    }
}
