use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_sub_r_r<const R1: u8, const R2: u8>(&mut self) {
        let val = self.registers.get_register8::<R2>();
        self.execute_sub::<R1>(val);
    }

    #[inline(always)]
    pub fn fetch_execute_sub_r_mr<const R1: u8, const R2: u8>(&mut self) {
        let val = self.read_mr::<R2>();
        self.execute_sub::<R1>(val);
    }

    #[inline(always)]
    pub fn fetch_execute_sub_r_d8<const R1: u8>(&mut self) {
        let val = self.read_pc();
        self.execute_sub::<R1>(val);
    }

    pub fn execute_sub<const R1: u8>(&mut self, rhs: u8) {
        let lhs = self.registers.get_register8::<R1>();
        let result = lhs.wrapping_sub(rhs);
        self.registers.set_register8::<R1>(result);
        self.registers.flags.op_sub8(lhs, rhs, 0, result);
    }
}
