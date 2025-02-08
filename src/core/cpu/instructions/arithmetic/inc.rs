use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct IncInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for IncInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        let mut value = fetched_data.value.wrapping_add(1);

        match self.address_mode {
            AddressMode::IMP
            | AddressMode::R_D16(_)
            | AddressMode::R_R(_, _)
            | AddressMode::MR_R(_, _)
            | AddressMode::R_D8(_)
            | AddressMode::R_MR(_, _)
            | AddressMode::R_HLI(_)
            | AddressMode::R_HLD(_)
            | AddressMode::HLI_R(_)
            | AddressMode::HLD_R(_)
            | AddressMode::R_A8(_)
            | AddressMode::A8_R(_)
            | AddressMode::HL_SPe8
            | AddressMode::D16
            | AddressMode::D8
            | AddressMode::MR_D8(_)
            | AddressMode::A16_R(_)
            | AddressMode::R_A16(_) => panic!("Not used"),
            AddressMode::MR(_r1) => {
                // uses only HL
                value &= 0xFF; // Ensure it fits into 8 bits
                cpu.write_to_memory(
                    fetched_data.dest.get_addr().expect("must exist for MR"),
                    value as u8,
                );
            }
            AddressMode::R(r1) => {
                if r1.is_16bit() {
                    cpu.m_cycles(1);
                }

                cpu.registers.set_register(r1, value);
                value = cpu.registers.read_register(r1);
            }
        }

        set_flags(cpu, value);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}

fn set_flags(cpu: &mut Cpu, val: u16) {
    if (cpu.current_opcode & 0x03) == 0x03 {
        return;
    }

    cpu.registers.flags.set(
        (val == 0).into(),
        false.into(),
        ((val & 0x0F) == 0).into(),
        None,
    );
}
