use crate::cpu::instructions::{ConditionType};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_jp_no_hl(&mut self) {
        self.registers.pc = self.step_ctx.fetched_data.value;
    }

    #[inline(always)]
    pub fn execute_jp_c(&mut self) {
        self.execute_jp(self.step_ctx.fetched_data.value, ConditionType::C);
    }

    #[inline(always)]
    pub fn execute_jp_nz(&mut self) {
        self.execute_jp(self.step_ctx.fetched_data.value, ConditionType::NZ);
    }

    #[inline(always)]
    pub fn execute_jp_z(&mut self) {
        self.execute_jp(self.step_ctx.fetched_data.value, ConditionType::Z);
    }

    #[inline(always)]
    pub fn execute_jp_nc(&mut self) {
        self.execute_jp(self.step_ctx.fetched_data.value, ConditionType::NC);
    }

    #[inline(always)]
    pub fn execute_jp_no(&mut self) {
        self.goto_addr(self.step_ctx.fetched_data.value, false);
    }

    #[inline(always)]
    fn execute_jp(&mut self, addr: u16, cond: ConditionType) {
        self.goto_addr_with_cond(cond, addr, false);
    }
}
