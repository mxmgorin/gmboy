use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::{FetchedData, RegisterType};
use crate::cpu::stack::Stack;

#[derive(Debug, Clone, Copy)]
pub struct PopInstruction {
    pub address_mode: AddressMode,
}

// C1 POP BC
// D1 POP DE
// E1 POP HL
// F1 POP AF

impl ExecutableInstruction for PopInstruction {
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
            | AddressMode::SPe8
            | AddressMode::D16
            | AddressMode::D8
            | AddressMode::MR_D8(_)
            | AddressMode::MR(_)
            | AddressMode::A16_R(_)
            | AddressMode::R_A16(_) => unreachable!("not used"),
            AddressMode::R(r1) => {
                let lo = Stack::pop(cpu) as u16;
                let hi = Stack::pop(cpu) as u16;
                let addr = (hi << 8) | lo;

                if r1 == RegisterType::AF {
                    cpu.registers.set_register(r1, addr & 0xFFF0);
                } else {
                    cpu.registers.set_register(r1, addr);
                }
            }
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
