use crate::cpu::instructions::ConditionType;
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_call_d16<const C: u8>(&mut self) {
        self.fetch_d16();
        self.execute_call::<C>();
    }

    #[inline(always)]
    fn execute_call<const C: u8>(&mut self) {
        let cond = ConditionType::from_u8(C);
        let addr = self.step_ctx.fetched_data.value;
        self.goto_addr_with_cond(cond, addr, true);
    }
}
