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
            | AddressMode::MR_D8(_)
            | AddressMode::MR(_)
            | AddressMode::A8_R(_)
            | AddressMode::R(_)
            | AddressMode::D16_R(_)
            | AddressMode::A16_R(_)
            | AddressMode::R_A8(_)
            | AddressMode::R_D16(_)
            | AddressMode::R_A16(_)
            | AddressMode::MR_R(_, _)
            | AddressMode::HLD_R(_) => unreachable!("not used"),
            AddressMode::R_R(r1, _) | AddressMode::R_MR(r1, _) | AddressMode::R_D8(r1) => {
                execute_sub(cpu, fetched_data, r1)
            }
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}

fn execute_sub(cpu: &mut Cpu, fetched_data: FetchedData, r1: RegisterType) {
    let reg_val = cpu.registers.read_register(r1);
    let result = reg_val.wrapping_sub(fetched_data.value);

    let reg_val_i32 = reg_val as i32;
    let fetched_val_i32 = result as i32;

    let h = ((reg_val_i32 & 0xF).wrapping_sub(fetched_val_i32 & 0xF)) < 0;
    let c = reg_val_i32.wrapping_sub(fetched_val_i32) < 0;

    cpu.registers.set_register(r1, result);
    cpu.registers
        .flags
        .set((result == 0).into(), true.into(), h.into(), c.into());
}
