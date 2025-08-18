use crate::cpu::instructions::{FetchedData, InstructionArgs};
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_cpl(&mut self, _fetched_data: FetchedData, _args: InstructionArgs) {
        self.registers.a = !self.registers.a;

        self.registers.flags.set_n(true);
        self.registers.flags.set_h(true);
    }
}

/// ComPLement accumulator (A = ~A); also called bitwise NOT.
/// Cycles: 1
/// Bytes: 1
/// Flags:
/// N 1
/// H 1
#[derive(Debug, Clone, Copy)]
pub struct CplInstruction;

impl ExecutableInstruction for CplInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_cpl(fetched_data, InstructionArgs::default(self.get_address_mode()))
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
