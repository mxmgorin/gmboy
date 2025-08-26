use crate::cpu::flags::{Flags, FlagsCtx, FlagsData, FlagsOp};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_rla(&mut self) {
        let lhs = self.registers.a;
        let carry_in = self.registers.flags.get_c() as u8;
        self.registers.a = (lhs << 1) | carry_in;
        self.registers.flags.set(FlagsCtx::new_rla(lhs));
    }
}

impl FlagsOp {
    #[inline(always)]
    pub fn rla(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(false);
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(((data.lhs >> 7) & 1) != 0);
    }
}
