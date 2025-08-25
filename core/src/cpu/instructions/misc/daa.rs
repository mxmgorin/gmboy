use crate::cpu::Cpu;

impl Cpu {
    /// Decimal Adjust Accumulator.
    /// Designed to be used after performing an arithmetic instruction (ADD, ADC, SUB, SBC) whose inputs were in Binary-Coded Decimal (BCD), adjusting the result to likewise be in BCD.
    /// The exact behavior of this instruction depends on the state of the subtract flag N:
    ///
    /// If the subtract flag N is set:
    /// Initialize the adjustment to 0.
    /// If the half-carry flag H is set, then add $6 to the adjustment.
    /// If the carry flag is set, then add $60 to the adjustment.
    /// Subtract the adjustment from A.
    /// Set the carry flag if borrow (i.e. if adjustment > A).
    /// If the subtract flag N is not set:
    /// Initialize the adjustment to 0.
    /// If the half-carry flag H is set or A & $F > $9, then add $6 to the adjustment.
    /// If the carry flag is set or A > $9F, then add $60 to the adjustment.
    /// Add the adjustment to A.
    /// Set the carry flag if overflow from bit 7.
    /// Cycles: 1
    /// Bytes: 1
    /// Flags:
    /// Z Set if result is 0.
    /// H 0
    /// C Set or reset depending on the operation.
    #[inline(always)]
    pub fn execute_daa(&mut self) {
        let mut u: u8 = 0;
        let mut fc: i32 = 0;

        if self.registers.flags.get_h()
            || (!self.registers.flags.get_n() && (self.registers.a & 0xF) > 9)
        {
            u = 6;
        }

        if self.registers.flags.get_c()
            || (!self.registers.flags.get_n() && self.registers.a > 0x99)
        {
            u |= 0x60;
            fc = 1;
        }

        if self.registers.flags.get_n() {
            self.registers.a = self.registers.a.wrapping_sub(u);
        } else {
            self.registers.a = self.registers.a.wrapping_add(u);
        };

        self.registers.flags.set_z(self.registers.a == 0);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(fc != 0);
    }
}
