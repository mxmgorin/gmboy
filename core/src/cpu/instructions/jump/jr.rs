use crate::cpu::instructions::{ConditionType};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_jr_no(&mut self) {
        self.execute_jr_inner(None);
    }

    #[inline]
    pub fn execute_jr_nz(&mut self) {
        self.execute_jr_inner(Some(ConditionType::NZ));
    }

    #[inline]
    pub fn execute_jr_z(&mut self) {
        self.execute_jr_inner(Some(ConditionType::Z));
    }

    #[inline]
    pub fn execute_jr_nc(&mut self) {
        self.execute_jr_inner(Some(ConditionType::NC));
    }

    #[inline]
    pub fn execute_jr_c(&mut self) {
        self.execute_jr_inner(Some(ConditionType::C));
    }

    #[inline(always)]
    fn execute_jr_inner(&mut self, cond: Option<ConditionType>) {
        let rel = self.step_ctx.fetched_data.value as i8;
        let addr = (self.registers.pc as i32).wrapping_add(rel as i32);
        self.goto_addr(cond, addr as u16, false);
    }
}
