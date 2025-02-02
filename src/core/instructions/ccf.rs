use crate::core::cpu::{Cpu, FetchedData};
use crate::core::instructions::common::{AddressMode, ExecutableInstruction};

#[derive(Debug, Clone, Copy)]
pub struct CcfInstruction;

impl ExecutableInstruction for CcfInstruction {
    fn execute(&self, _cpu: &mut Cpu, _fetched_data: FetchedData) {
        panic!("CcfInstruction not impl")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
