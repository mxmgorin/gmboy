use crate::cpu::instructions::{FetchedData, InstructionArgs};
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_halt(&mut self, _fetched_data: FetchedData, _args: InstructionArgs) {
        self.is_halted = true;
    }
}

// The exact behavior of this instruction depends on the state of the IME flag, and whether interrupts are pending (i.e. whether ‘[IE] & [IF]’ is non-zero):
//
// If the IME flag is set:
// The CPU enters low-power mode until after an interrupt is about to be serviced. The handler is executed normally, and the CPU resumes execution after the HALT when that returns.
// If the IME flag is not set, and no interrupts are pending:
// As soon as an interrupt becomes pending, the CPU resumes execution. This is like the above, except that the handler is not called.
// If the IME flag is not set, and some interrupt is pending:
// The CPU continues execution after the HALT, but the byte after it is read twice in a row (PC is not incremented, due to a hardware bug).
#[derive(Debug, Clone, Copy)]
pub struct HaltInstruction;

impl ExecutableInstruction for HaltInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_halt(fetched_data, InstructionArgs::default(self.get_address_mode()));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
