use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::{Cpu, FetchedData};

#[derive(Debug, Clone, Copy)]
pub struct HaltInstruction;

impl ExecutableInstruction for HaltInstruction {
    fn execute(&self, _cpu: &mut Cpu, _fetched_data: FetchedData) {
        unimplemented!("Execute HaltInstruction")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
