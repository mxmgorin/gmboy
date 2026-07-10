use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_and_r_r<const R1: u8, const R2: u8>(&mut self) {
        let rhs = self.registers.get_register8::<R2>();
        self.execute_and(rhs);
    }

    #[inline(always)]
    pub fn fetch_execute_and_r_mr<const R1: u8, const R2: u8>(&mut self) {
        let rhs = self.read_mr::<R2>();
        self.execute_and(rhs);
    }

    #[inline(always)]
    pub fn fetch_execute_and_r_d8<const R1: u8>(&mut self) {
        let rhs = self.read_pc();
        self.execute_and(rhs);
    }

    #[inline(always)]
    pub fn execute_and(&mut self, rhs: u8) {
        self.registers.a &= rhs;
        self.registers.flags.op_and(self.registers.a);
    }
}
