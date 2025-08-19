use crate::cpu::instructions::{ConditionType, FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_jr_no(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_jr_inner(fetched_data, None);
    }

    #[inline]
    pub fn execute_jr_nz(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_jr_inner(fetched_data, Some(ConditionType::NZ));
    }

    #[inline]
    pub fn execute_jr_z(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_jr_inner(fetched_data, Some(ConditionType::Z));
    }

    #[inline]
    pub fn execute_jr_nc(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_jr_inner(fetched_data, Some(ConditionType::NC));
    }

    #[inline]
    pub fn execute_jr_c(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_jr_inner(fetched_data, Some(ConditionType::C));
    }

    #[inline(always)]
    fn execute_jr_inner(&mut self, fetched_data: FetchedData, cond: Option<ConditionType>) {
        let rel = fetched_data.value as i8;
        let addr = (self.registers.pc as i32).wrapping_add(rel as i32);
        self.goto_addr(cond, addr as u16, false);
    }
}
