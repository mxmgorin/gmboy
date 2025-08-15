use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{DataDestination, FetchedData, RegisterType};
use crate::cpu::{Cpu};

#[derive(Debug, Clone, Copy)]
pub struct AddInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for AddInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        let DataDestination::Register(r) = fetched_data.dest else {
            unreachable!();
        };

        let reg_val = cpu.registers.read_register(r);
        let mut reg_val_u32: u32 = reg_val as u32 + fetched_data.value as u32;
        let is_sp = r == RegisterType::SP;

        if !is_sp && r.is_16bit() {
            cpu.clock.m_cycles(1);
            let h = (reg_val & 0xFFF) + (fetched_data.value & 0xFFF) >= 0x1000;
            let n = (reg_val as u32) + (fetched_data.value as u32);
            let c = n >= 0x10000;

            cpu.registers.flags.set_h(h);
            cpu.registers.flags.set_c(c);
        } else if is_sp {
            cpu.clock.m_cycles(2);
            reg_val_u32 = cpu
                .registers
                .read_register(r)
                .wrapping_add(fetched_data.value as i8 as u16) as u32;

            let h = (reg_val & 0xF) + (fetched_data.value & 0xF) >= 0x10;
            let c = (reg_val & 0xFF) + (fetched_data.value & 0xFF) >= 0x100;

            cpu.registers.flags.set_z(false);
            cpu.registers.flags.set_h(h);
            cpu.registers.flags.set_c(c);
        } else {
            let z = (reg_val_u32 & 0xFF) == 0;
            let h = (reg_val & 0xF) + (fetched_data.value & 0xF) >= 0x10;
            let c = ((reg_val as i32) & 0xFF) + ((fetched_data.value as i32) & 0xFF) >= 0x100;

            cpu.registers.flags.set_z(z);
            cpu.registers.flags.set_h(h);
            cpu.registers.flags.set_c(c);
        }

        cpu.registers.flags.set_n(false);
        cpu.registers.set_register(r, (reg_val_u32 & 0xFFFF) as u16);
    }

    #[inline]
    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
