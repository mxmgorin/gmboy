use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

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
#[derive(Debug, Clone, Copy)]
pub struct DaaInstruction;

// todo: test
impl ExecutableInstruction for DaaInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        let mut u: u8 = 0;
        let mut fc: i32 = 0;

        if cpu.registers.flags.get_h()
            || (!cpu.registers.flags.get_n() && (cpu.registers.a & 0xF) > 9)
        {
            u = 6;
        }

        if cpu.registers.flags.get_c() || (!cpu.registers.flags.get_n() && cpu.registers.a > 0x99) {
            u |= 0x60;
            fc = 1;
        }

        if cpu.registers.flags.get_n() {
            cpu.registers.a = cpu.registers.a.wrapping_sub(u);
        } else {
            cpu.registers.a = cpu.registers.a.wrapping_add(u);
        };

        cpu.registers.flags.set(
            (cpu.registers.a == 0).into(),
            None,
            false.into(),
            Some(fc != 0),
        );
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
