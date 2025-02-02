use crate::core::cpu::{Cpu, FetchedData};
use crate::core::instructions::common::{AddressMode, ExecutableInstruction};

/// Decimal Adjust Accumulator
#[derive(Debug, Clone, Copy)]
pub struct DaaInstruction;

impl ExecutableInstruction for DaaInstruction {
    fn execute(&self, _cpu: &mut Cpu, fetched_data: FetchedData) {
        unimplemented!("DaaInstruction")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
