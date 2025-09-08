use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_rlca(&mut self) {
        let lhs = self.registers.a;
        let carry = (lhs >> 7) & 1;
        self.registers.a = (lhs << 1) | carry;
        self.registers.flags.op_rlca(carry);
    }
}
