use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct IncInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for IncInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
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
            | AddressMode::D16_R(_)
            | AddressMode::MR_D8(_)
            | AddressMode::A16_R(_)
            | AddressMode::R_A16(_) => panic!("Not used"),
            AddressMode::MR(_r1) => {
                cpu.update_cycles(1); // always needs because uses only HL reg which is 16 bit

                let mut value = fetched_data.value.wrapping_add(1);
                value &= 0xFF; // Ensure it fits into 8 bits
                cpu.bus.write(
                    fetched_data.dest_addr.expect("must exist for MR"),
                    value as u8,
                );
                set_flags(cpu, value);
            }
            AddressMode::R(r1) => {
                if r1.is_16bit() {
                    cpu.update_cycles(1);
                }

                let value = fetched_data.value.wrapping_add(1);
                cpu.registers.set_register(r1, value);
                let value = cpu.registers.read_register(r1);
                set_flags(cpu, value);
            }
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}

fn set_flags(cpu: &mut Cpu, val: u16) {
    // TODO: move opcode in instruction
    if (cpu.current_opcode & 0x03) == 0x03 {
        return;
    }

    cpu.registers.flags.set(
        Some(val == 0),
        Some(false),
        Some((val & 0x0F) == 0),
        None,
    );
}
