use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_rrca(&mut self) {
        let lhs = self.registers.a & 1;
        self.registers.a >>= 1;
        self.registers.a |= lhs << 7;
        self.registers.flags.op_rra(lhs);
    }
}
