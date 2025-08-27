use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_rra(&mut self) {
        let carry = self.registers.flags.get_c() as u8;
        let lhs = self.registers.a;
        self.registers.a >>= 1;
        self.registers.a |= carry << 7;
        self.registers.flags.force_op_rra(lhs);
    }
}

