use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::{FetchedData, RegisterType};

#[derive(Debug, Clone, Copy)]
pub struct AddInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for AddInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        match self.address_mode {
            AddressMode::IMP
            | AddressMode::D16
            | AddressMode::D8
            | AddressMode::R_HLI(_)
            | AddressMode::R_HLD(_)
            | AddressMode::HLI_R(_)
            | AddressMode::HLD_R(_) => unreachable!("not used"),
            AddressMode::HL_SPe8 => execute_add(cpu, fetched_data, RegisterType::SP),
            AddressMode::R_D8(r1)
            | AddressMode::R(r1)
            | AddressMode::R_D16(r1)
            | AddressMode::R_A8(r1)
            | AddressMode::A8_R(r1)
            | AddressMode::D16_R(r1)
            | AddressMode::MR_D8(r1)
            | AddressMode::MR(r1)
            | AddressMode::A16_R(r1)
            | AddressMode::R_A16(r1)
            | AddressMode::R_R(r1, _)
            | AddressMode::MR_R(r1, _)
            | AddressMode::R_MR(r1, _) => execute_add(cpu, fetched_data, r1),
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}

// todo: test or rewrite casting, do they are correct?
fn execute_add(cpu: &mut Cpu, fetched_data: FetchedData, r1: RegisterType) {
    let reg_val = cpu.registers.read_register(r1);
    let mut reg_val_u32: u32 = reg_val as u32 + fetched_data.value as u32;
    let is_16bit = r1.is_16bit();

    if is_16bit {
        cpu.update_cycles(1);
    }

    if r1 == RegisterType::SP {
        reg_val_u32 = cpu
            .registers
            .read_register(r1)
            .wrapping_add(fetched_data.value as i8 as u16) as u32;
    }

    let mut z = if (reg_val_u32 & 0xFF) == 0 {
        Some(true)
    } else {
        Some(false)
    };
    let mut h = (reg_val & 0xF) + (fetched_data.value & 0xF) >= 0x10;
    let mut c = ((reg_val as i32) & 0xFF) + ((fetched_data.value as i32) & 0xFF) >= 0x100;

    if is_16bit {
        z = None;
        h = (reg_val & 0xFFF) + (fetched_data.value & 0xFFF) >= 0x1000;
        let n = (reg_val as u32) + (fetched_data.value as u32);
        c = n >= 0x10000;
    }

    if r1 == RegisterType::SP {
        z = Some(false);
        h = (reg_val & 0xF) + (fetched_data.value & 0xF) >= 0x10;
        c = (reg_val & 0xFF) + (fetched_data.value & 0xFF) >= 0x100;
    }

    cpu.registers
        .set_register(r1, (reg_val_u32 & 0xFFFF) as u16);
    cpu.registers.flags.set(z, false.into(), h.into(), c.into());
}
