use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::{FetchedData, RegisterType};

#[derive(Debug, Clone, Copy)]
pub struct SubInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for SubInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        match self.address_mode {
            AddressMode::IMP
            | AddressMode::D16
            | AddressMode::D8
            | AddressMode::HL_SPe8
            | AddressMode::R_HLI(_)
            | AddressMode::R_HLD(_)
            | AddressMode::HLI_R(_)
            | AddressMode::HLD_R(_) => unreachable!("not used"),
            AddressMode::R_R(r1, _r2) | AddressMode::MR_R(r1, _r2) | AddressMode::R_MR(r1, _r2) => {
                execute_sub(cpu, fetched_data, r1)
            }
            AddressMode::R_D8(r1)
            | AddressMode::R(r1)
            | AddressMode::R_D16(r1)
            | AddressMode::R_A8(r1)
            | AddressMode::A8_R(r1)
            | AddressMode::D16_R(r1)
            | AddressMode::MR_D8(r1)
            | AddressMode::MR(r1)
            | AddressMode::A16_R(r1)
            | AddressMode::R_A16(r1) => execute_sub(cpu, fetched_data, r1),
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}

fn execute_sub(cpu: &mut Cpu, fetched_data: FetchedData, r1: RegisterType) {
    let reg_val = cpu.registers.read_register(r1);
    let val = reg_val.wrapping_sub(fetched_data.value);

    let z = val == 0;
    let reg_val_i32 = reg_val as i32;
    let val_i32 = val as i32;
    let h = ((reg_val_i32 & 0xF).wrapping_sub(val_i32 & 0xF)) < 0;
    let c = reg_val_i32.wrapping_sub(val_i32) < 0;

    cpu.registers.f.set(
        (z as i8).into(),
        1.into(),
        (h as i8).into(),
        (c as i8).into(),
    );
}
