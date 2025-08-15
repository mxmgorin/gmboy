use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu};

#[derive(Debug, Clone, Copy)]
pub struct SubInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for SubInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        let DataDestination::Register(r) = fetched_data.dest else {
            unreachable!();
        };

        let reg_val = cpu.registers.read_register(r);
        let result = reg_val.wrapping_sub(fetched_data.value);

        let reg_val_i32 = reg_val as i32;
        let fetched_val_i32 = result as i32;

        let h = ((reg_val_i32 & 0xF).wrapping_sub(fetched_val_i32 & 0xF)) < 0;
        let c = reg_val_i32.wrapping_sub(fetched_val_i32) < 0;

        cpu.registers.set_register(r, result);

        cpu.registers.flags.set_z(result == 0);
        cpu.registers.flags.set_n(true);
        cpu.registers.flags.set_h(h);
        cpu.registers.flags.set_c(c);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
