use crate::cpu::instructions::InstructionSpec;
use crate::cpu::instructions::{FetchedData};
use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline]
    pub fn execute_jp(&mut self, fetched_data: FetchedData, spec: InstructionSpec) {
        if spec.cond_type.is_none() && fetched_data.source.get_register() == Some(RegisterType::HL)
        {
            // HL uses and no Cycles
            self.registers.pc = fetched_data.value;
        } else {
            self.goto_addr(spec.cond_type, fetched_data.value, false);
        }
    }
}
