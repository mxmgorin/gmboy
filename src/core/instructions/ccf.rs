use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, ExecutableInstruction};

#[derive(Debug, Clone, Copy)]
pub struct CcfInstruction;

impl ExecutableInstruction for CcfInstruction {
    fn execute(&self, _cpu: &mut Cpu) {
        eprintln!("CcfInstruction not impl")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
