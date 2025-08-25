use crate::cpu::flags::{Flags, FlagsCtx, FlagsData, FlagsOp};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_rlca(&mut self) {
        let lhs = self.registers.a;
        let carry = (lhs >> 7) & 1;
        let result = (lhs << 1) | carry;
        self.registers.a = result;

        self.registers.flags.set(FlagsCtx::rlca(carry));
    }
}

impl FlagsOp {
    #[inline(always)]
    pub fn rlca(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(false);
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(data.carry_in != 0);
    }
}
