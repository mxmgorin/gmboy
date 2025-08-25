use crate::cpu::Cpu;
use crate::cpu::flags::{Flags, FlagsCtx, FlagsData, FlagsOp};

impl Cpu {
    #[inline(always)]
    pub fn execute_rra(&mut self) {
        let carry = self.registers.flags.get_c() as u8;
        let lhs = self.registers.a;
        self.registers.a >>= 1;
        self.registers.a |= carry << 7;
        self.registers.flags.force_set(FlagsCtx::rra(lhs));
    }
}

impl FlagsOp {
    #[inline(always)]
    pub fn rra(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(false);
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw((data.lhs & 1) != 0);
    }
}
