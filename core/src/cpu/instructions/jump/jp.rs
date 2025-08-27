use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn execute_jp_no_hl(&mut self) {
        self.registers.pc = self.registers.get_register::<{ RegisterType::HL as u8 }>();
    }

    #[inline(always)]
    pub fn fetch_execute_jp_d16<const C: u8>(&mut self) {
        self.fetch_d16();
        self.execute_jp::<C>();
    }

    #[inline(always)]
    fn execute_jp<const C: u8>(&mut self) {
        let addr = self.step_ctx.fetched_data.value;
        self.goto_addr_with_cond::<C>(addr, false);
    }
}
