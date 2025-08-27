use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_ccf(&mut self) {
        let carry = self.registers.flags.get_c() as u8;
        self.registers.flags.op_ccf(carry);
    }
}

