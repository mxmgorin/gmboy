use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, ExecutableInstruction, RegisterType};

#[derive(Debug, Clone, Copy)]
pub struct IncInstruction {
    pub register_type: RegisterType,
}

impl ExecutableInstruction for IncInstruction {
    fn execute(&self, _cpu: &mut Cpu) {
        eprintln!("IncInstruction not impl")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::R(self.register_type)
    }
}
