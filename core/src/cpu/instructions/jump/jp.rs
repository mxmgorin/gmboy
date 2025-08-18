use crate::cpu::instructions::{
    AddressMode, ConditionType, ExecutableInstruction, InstructionArgs,
};
use crate::cpu::instructions::{FetchedData, RegisterType};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_jp(&mut self, fetched_data: FetchedData, args: InstructionArgs) {
        if args.cond_type.is_none() && fetched_data.source.get_register() == Some(RegisterType::HL)
        {
            // HL uses and no Cycles
            self.registers.pc = fetched_data.value;
        } else {
            self.goto_addr(args.cond_type, fetched_data.value, false);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct JpInstruction {
    pub addr_mode: AddressMode,
    pub cond_type: Option<ConditionType>,
}

impl ExecutableInstruction for JpInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_jp(fetched_data, InstructionArgs::new(self.cond_type, 0, self.addr_mode));
    }

    fn get_address_mode(&self) -> AddressMode {
        self.addr_mode
    }
}
