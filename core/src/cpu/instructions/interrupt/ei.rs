use crate::cpu::instructions::{FetchedData, InstructionArgs};
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_ei(&mut self, _fetched_data: FetchedData, _args: InstructionArgs) {
        self.enabling_ime = true;
    }
}

/// Enable Interrupts by setting the IME flag.
/// The flag is only set after the instruction following EI.
#[derive(Debug, Clone, Copy)]
pub struct EiInstruction;

impl ExecutableInstruction for EiInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_ei(fetched_data, InstructionArgs::default(self.get_address_mode()));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
