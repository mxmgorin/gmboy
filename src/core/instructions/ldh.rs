use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, ExecutableInstruction};

#[derive(Debug, Clone, Copy)]
pub struct LdhInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for LdhInstruction {
    fn execute(&self, _cpu: &mut Cpu) {
        panic!("Executing LdhInstruction");
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
