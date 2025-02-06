use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::{FetchedData, RegisterType};

#[derive(Debug, Clone, Copy)]
pub struct AdcInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for AdcInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        match self.address_mode {
            AddressMode::IMP | AddressMode::D16 | AddressMode::D8 | AddressMode::HL_SPe8 => {
                unreachable!("not used")
            }
            AddressMode::R_HLI(_r)
            | AddressMode::R_HLD(_r)
            | AddressMode::HLI_R(_r)
            | AddressMode::HLD_R(_r) => unreachable!("not used"),
            AddressMode::R_R(r1, _r2) | AddressMode::MR_R(r1, _r2) | AddressMode::R_MR(r1, _r2) => {
                execute_adc(cpu, fetched_data, r1)
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
            | AddressMode::R_A16(r1) => execute_adc(cpu, fetched_data, r1),
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}

fn execute_adc(cpu: &mut Cpu, fetched_data: FetchedData, _r1: RegisterType) {
    let u: u16 = fetched_data.value;
    let a: u16 = cpu.registers.a as u16;
    let c: u16 = cpu.registers.flags.get_c() as u16;

    cpu.registers.a = ((a + u + c) & 0xFF) as u8;

    cpu.registers.flags.set(
        (cpu.registers.a == 0).into(),
        false.into(),
        ((a & 0xF) + (u & 0xF) + c > 0xF).into(),
        (a + u + c > 0xFF).into(),
    );
}
