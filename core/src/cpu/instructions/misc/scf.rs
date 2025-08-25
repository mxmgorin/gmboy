use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_scf(&mut self) {
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(true);
    }
}
