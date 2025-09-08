use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_xor_r_r<const R1: u8, const R2: u8>(&mut self) {
        let rhs = self.registers.get_register8::<R2>();
        self.execute_xor(rhs);
    }

    #[inline(always)]
    pub fn fetch_execute_xor_r_d8<const R1: u8>(&mut self) {
        let rhs = self.read_pc();
        self.execute_xor(rhs);
    }

    #[inline(always)]
    pub fn fetch_execute_xor_r_mr<const R1: u8, const R2: u8>(&mut self) {
        let rhs = self.read_mr::<R2>();
        self.execute_xor(rhs);
    }

    #[inline(always)]
    pub fn execute_xor(&mut self, rhs: u8) {
        self.registers.a ^= rhs & 0xFF;
        // todo: for some reason fails test when lazy is used
        self.registers.flags.force_op_or(self.registers.a);
    }
}
