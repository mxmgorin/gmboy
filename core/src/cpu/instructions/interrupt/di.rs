use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu, CpuCallback};

/// Disable Interrupts by clearing the IME flag.
#[derive(Debug, Clone, Copy)]
pub struct DiInstruction;

impl ExecutableInstruction for DiInstruction {
    fn execute(&self, _cpu: &mut Cpu, callback: &mut impl CpuCallback, _fetched_data: FetchedData) {
        callback.get_bus_mut().io.interrupts.ime = false;
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
