use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{ConditionType};
use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_jp_no_hl(&mut self, fetched_data: FetchedData) {
        self.registers.pc = fetched_data.value;
    }

    #[inline(always)]
    pub fn execute_jp_c(&mut self, fetched_data: FetchedData) {
        self.execute_jp(fetched_data.value, Some(ConditionType::C));
    }

    #[inline(always)]
    pub fn execute_jp_nz(&mut self, fetched_data: FetchedData) {
        self.execute_jp(fetched_data.value, Some(ConditionType::NZ));
    }

    #[inline(always)]
    pub fn execute_jp_z(&mut self, fetched_data: FetchedData) {
        self.execute_jp(fetched_data.value, Some(ConditionType::Z));
    }

    #[inline(always)]
    pub fn execute_jp_nc(&mut self, fetched_data: FetchedData) {
        self.execute_jp(fetched_data.value, Some(ConditionType::NC));
    }

    #[inline(always)]
    pub fn execute_jp_no(&mut self, fetched_data: FetchedData) {
        self.execute_jp(fetched_data.value, None);
    }

    #[inline(always)]
    fn execute_jp(&mut self, addr: u16, cond: Option<ConditionType>) {
        self.goto_addr(cond, addr, false);
    }
}
