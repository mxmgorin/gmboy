use crate::cpu::flags::{Flags, FlagsData, FlagsOp};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_ccf(&mut self) {
        let carry = self.registers.flags.get_c() as u8;
        self.registers.flags.op_ccf(carry);
    }
}

impl FlagsOp {
    #[inline(always)]
    pub fn ccf(data: FlagsData, flags: &mut Flags) {
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(data.carry == 0);
    }
}
