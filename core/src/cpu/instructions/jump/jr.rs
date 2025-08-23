use crate::cpu::instructions::JumpCondition;
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_jr_d8<const C: u8>(&mut self) {
        self.fetch_d8();
        self.execute_jr::<C>();
    }

    #[inline(always)]
    pub fn execute_jr<const C: u8>(&mut self) {
        let cond = JumpCondition::from_u8(C);
        let rel = self.step_ctx.fetched_data.value as i8;
        let addr = (self.registers.pc as i32).wrapping_add(rel as i32);
        self.goto_addr_with_cond(cond, addr as u16, false);
    }
}
