use crate::cpu::instructions::{AddressMode, ConditionType, ExecutableInstruction};
use crate::cpu::instructions::{FetchedData, InstructionArgs};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_call(&mut self, fetched_data: FetchedData, args: InstructionArgs) {
        self.goto_addr(args.cond_type, fetched_data.value, true);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CallInstruction {
    pub condition_type: Option<ConditionType>,
}

impl ExecutableInstruction for CallInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_call(
            fetched_data,
            InstructionArgs::new(self.condition_type, 0, self.get_address_mode()),
        );
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D16
    }
}
