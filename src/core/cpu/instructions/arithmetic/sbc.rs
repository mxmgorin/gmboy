use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu, CpuCycleCallback};

/// Subtract the value in r8 and the carry flag from A.
/// Cycles: 1
/// Bytes: 1
/// Flags:
/// Z Set if result is 0.
/// N 1
/// H Set if borrow from bit 4.
/// C Set if borrow (i.e. if (r8 + carry) > A).
#[derive(Debug, Clone, Copy)]
pub struct SbcInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for SbcInstruction {
    fn execute(
        &self,
        cpu: &mut Cpu,
        _callback: &mut impl CpuCycleCallback,
        fetched_data: FetchedData,
    ) {
        let DataDestination::Register(r) = fetched_data.dest else {
            unreachable!();
        };

        let c_val = cpu.registers.flags.get_c();
        let val_plus_c = fetched_data.value.wrapping_add(c_val as u16) as u8;
        let r_val = cpu.registers.read_register(r);

        let c_val_i32 = c_val as i32;
        let r_val_i32 = r_val as i32;
        let fetched_val_i32 = fetched_data.value as i32;

        let h = (r_val_i32 & 0xF)
            .wrapping_sub(fetched_val_i32 & 0xF)
            .wrapping_sub(c_val_i32)
            < 0;
        let c = r_val_i32
            .wrapping_sub(fetched_val_i32)
            .wrapping_sub(c_val_i32)
            < 0;

        let result = r_val.wrapping_sub(val_plus_c as u16);

        cpu.registers.set_register(r, result);
        cpu.registers
            .flags
            .set((result == 0).into(), true.into(), h.into(), c.into());
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
