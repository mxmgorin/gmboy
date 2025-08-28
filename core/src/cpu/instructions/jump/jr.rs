use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_jr_d8<const C: u8>(&mut self) {
        let val = self.read_d8();
        self.execute_jr::<C>(val);
    }

    #[inline(always)]
    pub fn execute_jr<const C: u8>(&mut self, val: u8) {
        let rel = val as i8;
        let addr = (self.registers.pc as i32).wrapping_add(rel as i32);
        self.goto_addr_with_cond::<C>(addr as u16);
    }
}
