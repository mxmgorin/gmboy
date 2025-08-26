use crate::cpu::flags::{Flags, FlagsCtx, FlagsData, FlagsOp};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_rlca(&mut self) {
        let lhs = self.registers.a;
        let carry = (lhs >> 7) & 1;
        self.registers.a = (lhs << 1) | carry;
        self.registers.flags.set(FlagsCtx::new_rlca(carry));
    }
}

impl FlagsOp {
    #[inline(always)]
    pub fn rlca(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(false);
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(data.carry != 0);
    }
}
