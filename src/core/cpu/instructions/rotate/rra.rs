use crate::cpu::instructions::{AddressMode, ExecutableInstruction, FetchedData};
use crate::cpu::{Cpu, CpuCallback};

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
    fn execute(&self, cpu: &mut Cpu, _callback: &mut impl CpuCallback, _fetched_data: FetchedData) {
        let carry: u8 = cpu.registers.flags.get_c() as u8;
        let new_c: u8 = cpu.registers.a & 1;

        cpu.registers.a >>= 1;
        cpu.registers.a |= carry << 7;

        cpu.registers
            .flags
            .set(false.into(), false.into(), false.into(), Some(new_c != 0));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
