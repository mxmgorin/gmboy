use crate::cpu::instructions::{AddressMode, ExecutableInstruction, FetchedData, InstructionArgs};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_rra(&mut self, _fetched_data: FetchedData, _args: InstructionArgs) {
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
/// C Set according to result.
#[derive(Debug, Clone, Copy)]
pub struct RraInstruction;

impl ExecutableInstruction for RraInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        cpu.execute_rra(_fetched_data, InstructionArgs::default(self.get_address_mode()));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
