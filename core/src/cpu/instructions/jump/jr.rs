use crate::cpu::instructions::{AddressMode, ConditionType, ExecutableInstruction};
use crate::cpu::instructions::{FetchedData, InstructionArgs};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_jr(&mut self, fetched_data: FetchedData, args: InstructionArgs) {
        let rel = fetched_data.value as i8;
        let addr = (self.registers.pc as i32).wrapping_add(rel as i32);
        self.goto_addr(args.cond_type, addr as u16, false);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JrInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for JrInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_jr(fetched_data, InstructionArgs::new(self.condition_type, 0, self.get_address_mode()));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D8
    }
}
