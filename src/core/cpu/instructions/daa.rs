use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::{Cpu, FetchedData};

/// Decimal Adjust Accumulator
#[derive(Debug, Clone, Copy)]
pub struct DaaInstruction;

impl ExecutableInstruction for DaaInstruction {
    fn execute(&self, _cpu: &mut Cpu, _fetched_data: FetchedData) {
        unimplemented!("DaaInstruction")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
