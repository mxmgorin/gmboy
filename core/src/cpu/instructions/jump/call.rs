use crate::cpu::instructions::{ConditionType, FetchedData, InstructionSpec};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_call_no(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_call(fetched_data.value, None);
    }

    #[inline(always)]
    pub fn execute_call_z(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_call(fetched_data.value, Some(ConditionType::Z));
    }

    #[inline(always)]
    pub fn execute_call_nc(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_call(fetched_data.value, Some(ConditionType::NC));
    }

    #[inline(always)]
    pub fn execute_call_c(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_call(fetched_data.value, Some(ConditionType::C));
    }

    #[inline(always)]
    pub fn execute_call_nz(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        self.execute_call(fetched_data.value, Some(ConditionType::NZ));
    }

    #[inline(always)]
    pub fn execute_call(&mut self, addr: u16, cond: Option<ConditionType>) {
        self.goto_addr(cond, addr, true);
    }
}
