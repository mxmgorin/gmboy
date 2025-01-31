use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, ExecutableInstruction, RegisterType};

#[derive(Debug, Clone, Copy)]
pub struct DecInstruction {
    pub register_type: RegisterType,
}

impl ExecutableInstruction for DecInstruction {
    fn execute(&self, _cpu: &mut Cpu) {
        eprintln!("DecInstruction not impl")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::R(self.register_type)
    }
}
