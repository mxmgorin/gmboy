use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::FetchedData;
use crate::cpu::stack::Stack;

#[derive(Debug, Clone, Copy)]
pub struct PushInstruction {
    pub address_mode: AddressMode,
}

// C5: PUSH BC
// D5: PUSH DE
// E5: PUSH HL
// F5: PUSH AF

impl ExecutableInstruction for PushInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
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
            | AddressMode::MR(_)
            | AddressMode::A16_R(_)
            | AddressMode::R_A16(_) => unreachable!("not used"),
            AddressMode::R(r1) => {
                cpu.update_cycles(1);

                let hi = (cpu.registers.read_register(r1) >> 8) & 0xFF;
                Stack::push(cpu, hi as u8);

                let lo = cpu.registers.read_register(r1) & 0xFF;
                Stack::push(cpu, lo as u8);
            }
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
