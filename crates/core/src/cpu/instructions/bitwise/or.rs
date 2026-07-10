use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_or_r_r<const R1: u8, const R2: u8>(&mut self) {
        let rhs = self.registers.get_register8::<R2>();
        self.execute_or(rhs);
    }

    #[inline(always)]
    pub fn fetch_execute_or_r_d8<const R1: u8>(&mut self) {
        let rhs = self.read_pc();
        self.execute_or(rhs);
    }

    #[inline(always)]
    pub fn fetch_execute_or_r_mr<const R1: u8, const R2: u8>(&mut self) {
        let rhs = self.read_mr::<R2>();
        self.execute_or(rhs);
    }

    #[inline(always)]
    pub fn execute_or(&mut self, rhs: u8) {
        self.registers.a |= rhs;
        self.registers.flags.op_or(self.registers.a)
    }
}
