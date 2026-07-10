use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_scf(&mut self) {
        self.registers.flags.op_scf();
    }
}
