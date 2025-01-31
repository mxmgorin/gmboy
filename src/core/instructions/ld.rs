use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, ExecutableInstruction};

#[derive(Debug, Clone, Copy)]
pub struct LdInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for LdInstruction {
    fn execute(&self, _cpu: &mut Cpu) {
        eprintln!("LD instruction not implemented");
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
