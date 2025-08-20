use crate::cpu::instructions::{ConditionType};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_call_no(&mut self) {
        self.execute_call(self.step_ctx.fetched_data.value, None);
    }

    #[inline(always)]
    pub fn execute_call_z(&mut self) {
        self.execute_call(self.step_ctx.fetched_data.value, Some(ConditionType::Z));
    }

    #[inline(always)]
    pub fn execute_call_nc(&mut self) {
        self.execute_call(self.step_ctx.fetched_data.value, Some(ConditionType::NC));
    }

    #[inline(always)]
    pub fn execute_call_c(&mut self) {
        self.execute_call(self.step_ctx.fetched_data.value, Some(ConditionType::C));
    }

    #[inline(always)]
    pub fn execute_call_nz(&mut self) {
        self.execute_call(self.step_ctx.fetched_data.value, Some(ConditionType::NZ));
    }

    #[inline(always)]
    pub fn execute_call(&mut self, addr: u16, cond: Option<ConditionType>) {
        self.goto_addr(cond, addr, true);
    }
}
