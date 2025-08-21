use crate::cpu::instructions::ConditionType;
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_call_no(&mut self) {
        self.goto_addr(self.step_ctx.fetched_data.value, true);
    }

    #[inline(always)]
    pub fn execute_call_z(&mut self) {
        self.execute_call(self.step_ctx.fetched_data.value, ConditionType::Z);
    }

    #[inline(always)]
    pub fn execute_call_nc(&mut self) {
        self.execute_call(self.step_ctx.fetched_data.value, ConditionType::NC);
    }

    #[inline(always)]
    pub fn execute_call_c(&mut self) {
        self.execute_call(self.step_ctx.fetched_data.value, ConditionType::C);
    }

    #[inline(always)]
    pub fn execute_call_nz(&mut self) {
        self.execute_call(self.step_ctx.fetched_data.value, ConditionType::NZ);
    }

    #[inline(always)]
    pub fn execute_call(&mut self, addr: u16, cond: ConditionType) {
        self.goto_addr_with_cond(cond, addr, true);
    }
}
