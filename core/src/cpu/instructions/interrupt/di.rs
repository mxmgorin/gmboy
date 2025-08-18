use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{FetchedData, InstructionArgs};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_di(&mut self, _fetched_data: FetchedData, _args: InstructionArgs) {
        self.clock.bus.io.interrupts.ime = false;
    }
}

/// Disable Interrupts by clearing the IME flag.
#[derive(Debug, Clone, Copy)]
pub struct DiInstruction;

impl ExecutableInstruction for DiInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_di(fetched_data, InstructionArgs::default(self.get_address_mode()));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
