use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{DataDestination, FetchedData, RegisterType};
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct AddInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for AddInstruction {
    fn execute(&self, cpu: &mut Cpu, callback: &mut impl CpuCallback, fetched_data: FetchedData) {
        let DataDestination::Register(r) = fetched_data.dest else {
            unreachable!();
        };

        let reg_val = cpu.registers.read_register(r);
        let mut reg_val_u32: u32 = reg_val as u32 + fetched_data.value as u32;

        if r == RegisterType::SP {
            callback.m_cycles(1, &mut cpu.bus);
            reg_val_u32 = cpu
                .registers
                .read_register(r)
                .wrapping_add(fetched_data.value as i8 as u16) as u32;
        }

        let mut z = if (reg_val_u32 & 0xFF) == 0 {
            Some(true)
        } else {
            Some(false)
        };
        let mut h = (reg_val & 0xF) + (fetched_data.value & 0xF) >= 0x10;
        let mut c = ((reg_val as i32) & 0xFF) + ((fetched_data.value as i32) & 0xFF) >= 0x100;

        if r.is_16bit() {
            callback.m_cycles(1, &mut cpu.bus);
            z = None;
            h = (reg_val & 0xFFF) + (fetched_data.value & 0xFFF) >= 0x1000;
            let n = (reg_val as u32) + (fetched_data.value as u32);
            c = n >= 0x10000;
        }

        if r == RegisterType::SP {
            z = Some(false);
            h = (reg_val & 0xF) + (fetched_data.value & 0xF) >= 0x10;
            c = (reg_val & 0xFF) + (fetched_data.value & 0xFF) >= 0x100;
        }

        cpu.registers.set_register(r, (reg_val_u32 & 0xFFFF) as u16);
        cpu.registers.flags.set(z, false.into(), h.into(), c.into());
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
