use crate::cpu::instructions::{FetchedData};
use crate::cpu::Cpu;

impl Cpu {
    /// Rotate register A right, through the carry flag.
    ///
    ///   ┏━━━━━━━ A ━━━━━━━┓ ┏━ Flags ━┓
    /// ┌─╂→ b7 → ... → b0 ─╂─╂→   C   ─╂─┐
    /// │ ┗━━━━━━━━━━━━━━━━━┛ ┗━━━━━━━━━┛ │
    /// └─────────────────────────────────┘
    /// Cycles: 1
    ///
    /// Bytes: 1
    ///
    /// Flags:
    ///
    /// Z 0
    /// N 0
    /// H 0
    /// C Set according to result.#[inline]
    pub fn execute_rra(&mut self, _fetched_data: FetchedData) {
        let carry: u8 = self.registers.flags.get_c() as u8;
        let new_c: u8 = self.registers.a & 1;

        self.registers.a >>= 1;
        self.registers.a |= carry << 7;

        self.registers.flags.set_z(false);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(new_c != 0);
    }
}
