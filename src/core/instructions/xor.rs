use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, ExecutableInstruction};

#[derive(Debug, Clone, Copy)]
pub struct XorInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for XorInstruction {
    fn execute(&self, _cpu: &mut Cpu) {
        eprintln!("LD instruction not implemented");
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
