use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::FetchedData;
use crate::cpu::{Cpu, CpuCallback};
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
    fn execute(&self, cpu: &mut Cpu, _callback: &mut impl CpuCallback, _fetched_data: FetchedData) {
        cpu.is_halted = true;
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
