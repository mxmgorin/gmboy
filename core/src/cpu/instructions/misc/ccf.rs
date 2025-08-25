use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_ccf(&mut self) {
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        let c = self.registers.flags.get_c();
        self.registers.flags.set_c(!c);
    }
}
