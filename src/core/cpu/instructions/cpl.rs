use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::{Cpu, FetchedData};

#[derive(Debug, Clone, Copy)]
pub struct CplInstruction;

impl ExecutableInstruction for CplInstruction {
    fn execute(&self, _cpu: &mut Cpu, _fetched_data: FetchedData) {
        unimplemented!("CplInstruction")
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
