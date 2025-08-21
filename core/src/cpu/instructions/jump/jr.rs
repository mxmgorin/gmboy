use crate::cpu::instructions::ConditionType;
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_jr_no(&mut self) {
        self.execute_jr_inner(ConditionType::None);
    }

    #[inline(always)]
    pub fn execute_jr_nz(&mut self) {
        self.execute_jr_inner(ConditionType::NZ);
    }

    #[inline(always)]
    pub fn execute_jr_z(&mut self) {
        self.execute_jr_inner(ConditionType::Z);
    }

    #[inline(always)]
    pub fn execute_jr_nc(&mut self) {
        self.execute_jr_inner(ConditionType::NC);
    }

    #[inline(always)]
    pub fn execute_jr_c(&mut self) {
        self.execute_jr_inner(ConditionType::C);
    }

    #[inline(always)]
    fn execute_jr_inner(&mut self, cond: ConditionType) {
        let rel = self.step_ctx.fetched_data.value as i8;
        let addr = (self.registers.pc as i32).wrapping_add(rel as i32);
        self.goto_addr_with_cond(cond, addr as u16, false);
    }
}
