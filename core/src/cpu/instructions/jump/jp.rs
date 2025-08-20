use crate::cpu::instructions::{ConditionType};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_jp_no_hl(&mut self) {
        self.registers.pc = self.step_ctx.fetched_data.value;
    }

    #[inline(always)]
    pub fn execute_jp_c(&mut self) {
        self.execute_jp(self.step_ctx.fetched_data.value, Some(ConditionType::C));
    }

    #[inline(always)]
    pub fn execute_jp_nz(&mut self) {
        self.execute_jp(self.step_ctx.fetched_data.value, Some(ConditionType::NZ));
    }

    #[inline(always)]
    pub fn execute_jp_z(&mut self) {
        self.execute_jp(self.step_ctx.fetched_data.value, Some(ConditionType::Z));
    }

    #[inline(always)]
    pub fn execute_jp_nc(&mut self) {
        self.execute_jp(self.step_ctx.fetched_data.value, Some(ConditionType::NC));
    }

    #[inline(always)]
    pub fn execute_jp_no(&mut self) {
        self.execute_jp(self.step_ctx.fetched_data.value, None);
    }

    #[inline(always)]
    fn execute_jp(&mut self, addr: u16, cond: Option<ConditionType>) {
        self.goto_addr(cond, addr, false);
    }
}
