use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::FetchedData;

/// Load High Memory
#[derive(Debug, Clone, Copy)]
pub struct LdhInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for LdhInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        match self.address_mode {
            AddressMode::IMP
            | AddressMode::R(_)
            | AddressMode::R_D16(_)
            | AddressMode::R_R(_, _)
            | AddressMode::R_D8(_)
            | AddressMode::R_HLI(_)
            | AddressMode::R_HLD(_)
            | AddressMode::HLI_R(_)
            | AddressMode::HLD_R(_)
            | AddressMode::HL_SPe8
            | AddressMode::D16
            | AddressMode::D8
            | AddressMode::D16_R(_)
            | AddressMode::MR_D8(_)
            | AddressMode::MR(_)
            | AddressMode::A16_R(_)
            | AddressMode::R_A16(_) => unreachable!("not used for LDH instruction"),
            AddressMode::R_A8(_) | AddressMode::R_MR(_, _) => {
                let value = cpu.bus.read(0xFF00 | fetched_data.value);
                cpu.registers.a = value; // uses only A register
            }
            AddressMode::A8_R(_) | AddressMode::MR_R(_, _) => {
                cpu.bus.write(
                    fetched_data.dest_addr.expect("must exist for A8"),
                    cpu.registers.a, // uses only A register
                );
            }
        }

        cpu.update_cycles(1);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
