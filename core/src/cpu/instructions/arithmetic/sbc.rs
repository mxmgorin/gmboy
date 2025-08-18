use crate::cpu::instructions::{AddressMode, ExecutableInstruction, InstructionArgs};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_sbc(&mut self, fetched_data: FetchedData, _args: InstructionArgs) {
        let DataDestination::Register(r) = fetched_data.dest else {
            unreachable!();
        };

        let c_val = self.registers.flags.get_c();
        let val_plus_c = fetched_data.value.wrapping_add(c_val as u16) as u8;
        let r_val = self.registers.read_register(r);

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

        self.registers.set_register(r, result);

        self.registers.flags.set_z(result == 0);
        self.registers.flags.set_n(true);
        self.registers.flags.set_h(h);
        self.registers.flags.set_c(c);
    }
}

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
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_sbc(fetched_data, InstructionArgs::default(self.address_mode));
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
