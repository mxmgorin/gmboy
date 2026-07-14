use crate::cpu::Cpu;

impl Cpu {
    // The exact behavior of this instruction depends on the state of the IME flag, and whether interrupts are pending (i.e. whether ‘[IE] & [IF]’ is non-zero):
    //
    // If the IME flag is set:
    // The CPU enters low-power mode until after an interrupt is about to be serviced. The handler is executed normally, and the CPU resumes execution after the HALT when that returns.
    // If the IME flag is not set, and no interrupts are pending:
    // As soon as an interrupt becomes pending, the CPU resumes execution. This is like the above, except that the handler is not called.
    // If the IME flag is not set, and some interrupt is pending:
    // The CPU continues execution after the HALT, but the byte after it is read twice in a row (PC is not incremented, due to a hardware bug).
    #[inline(always)]
    pub fn execute_halt(&mut self) {
        let interrupts = &self.clock.bus.io.interrupts;

        // With interrupts enabled (or being enabled via the EI delay) and one
        // already pending, HALT does not actually halt: the CPU services the
        // interrupt but leaves PC on the HALT so it re-runs after the handler
        // returns, repeating until nothing is pending. Rewind past the opcode
        // we just fetched so the pushed return address points at the HALT (only
        // an interrupt that wakes a truly-halted CPU returns to HALT+1).
        if interrupts.has_pending() && (interrupts.ime || self.enabling_ime) {
            self.registers.pc = self.registers.pc.wrapping_sub(1);
            return;
        }

        self.clock.cpu_halted = true;
    }
}
