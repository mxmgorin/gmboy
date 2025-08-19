
use crate::cpu::Cpu;

use crate::cpu::instructions::FetchedData;

impl Cpu {
    /// ComPare the value in A with the value in r8.
    /// This subtracts the value in r8 from A and sets flags accordingly, but discards the result.
    /// Cycles: 1
    /// Bytes: 1
    /// Flags:
    /// Z Set if result is 0.
    /// N 1
    /// H Set if borrow from bit 4.
    /// C Set if borrow (i.e. if r8 > A).
    #[inline]
    pub fn execute_cp(&mut self, fetched_data: FetchedData) {
        let fetched_value_i32 = fetched_data.value as i32;
        let reg_i32 = self.registers.a as i32;
        let result: i32 = reg_i32.wrapping_sub(fetched_value_i32);
        let reg_value_diff = (reg_i32 & 0x0F) - (fetched_value_i32 & 0x0F);

        self.registers.flags.set_z(result == 0);
        self.registers.flags.set_n(true);
        self.registers.flags.set_h(reg_value_diff < 0);
        self.registers.flags.set_c(result < 0);
    }
}
