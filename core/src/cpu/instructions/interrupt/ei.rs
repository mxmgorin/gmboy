use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu};

/// Enable Interrupts by setting the IME flag.
/// The flag is only set after the instruction following EI.
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
