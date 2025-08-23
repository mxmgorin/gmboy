use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_cpl(&mut self) {
        self.registers.a = !self.registers.a;

        self.registers.flags.set_n(true);
        self.registers.flags.set_h(true);
    }
}
