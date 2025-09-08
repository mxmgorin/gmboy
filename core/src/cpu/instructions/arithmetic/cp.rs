use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_cp_r_d8<const R1: u8>(&mut self) {
        let val = self.read_pc();
        self.execute_cp(val);
    }

    #[inline(always)]
    pub fn fetch_execute_cp_r_mr<const R1: u8, const R2: u8>(&mut self) {
        let val = self.read_mr::<R2>();
        self.execute_cp(val);
    }

    #[inline(always)]
    pub fn fetch_execute_cp_r_r<const R1: u8, const R2: u8>(&mut self) {
        let val = self.registers.get_register8::<R2>();
        self.execute_cp(val);
    }

    #[inline(always)]
    pub fn execute_cp(&mut self, rhs: u8) {
        let lhs = self.registers.a;
        let result = lhs.wrapping_sub(rhs);
        self.registers.flags.op_sub8(lhs, rhs, 0, result);
    }
}
