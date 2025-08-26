use crate::cpu::flags::{Flags, FlagsData, FlagsOp};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.flags.op_cpl()
    }
}

impl FlagsOp {
    pub fn cpl(_data: FlagsData, flags: &mut Flags) {
        flags.set_n_raw(true);
        flags.set_h_raw(true);
    }
}
