use crate::core::cpu::{Cpu, FetchedData};
use crate::core::instructions::common::{AddressMode, ExecutableInstruction};

#[derive(Debug, Clone, Copy)]
pub struct CplInstruction;

impl ExecutableInstruction for CplInstruction {
    fn execute(&self, _cpu: &mut Cpu, fetched_data: FetchedData) {
        unimplemented!("CplInstruction")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
