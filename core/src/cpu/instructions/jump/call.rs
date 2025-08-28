use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_call_d16<const C: u8>(&mut self) {
        let addr = self.read_pc16();
        self.execute_call::<C>(addr);
    }

    #[inline(always)]
    fn execute_call<const C: u8>(&mut self, addr: u16) {
        self.goto_addr_push_pc_with_cond::<C>(addr);
    }
}
