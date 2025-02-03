use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::{Cpu}; use crate::cpu::instructions::common::FetchedData;


#[derive(Debug, Clone, Copy)]
pub struct EiInstruction;

impl ExecutableInstruction for EiInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        cpu.enabling_ime = true;
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
