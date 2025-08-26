use crate::cpu::Cpu;
use crate::cpu::flags::{Flags, FlagsCtx, FlagsData, FlagsOp};

impl Cpu {
    #[inline(always)]
    pub fn execute_scf(&mut self) {
        self.registers.flags.set(FlagsCtx::new_scf());
    }
}

impl FlagsOp {
    pub fn scf(_data: FlagsData, flags: &mut Flags) {
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(true);
    }
}
