use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, ExecutableInstruction};

#[derive(Debug, Clone, Copy)]
pub struct HaltInstruction;

impl ExecutableInstruction for HaltInstruction {
    fn execute(&self, _cpu: &mut Cpu) {
        eprintln!("HaltInstruction not impl")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
