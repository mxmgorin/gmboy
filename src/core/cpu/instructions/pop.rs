use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::{FetchedData, RegisterType};
use crate::cpu::stack::Stack;

#[derive(Debug, Clone, Copy)]
pub struct PopInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for PopInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        match self.address_mode {
            AddressMode::IMP
            | AddressMode::R_D16(_)
            | AddressMode::R_R(_, _)
            | AddressMode::MR_R(_, _)
            | AddressMode::R_D8(_)
            | AddressMode::R_MR(_, _)
            | AddressMode::R_HLI(_, _)
            | AddressMode::R_HLD(_, _)
            | AddressMode::HLI_R(_, _)
            | AddressMode::HLD_R(_, _)
            | AddressMode::R_A8(_)
            | AddressMode::A8_R(_)
            | AddressMode::HL_SPR(_, _)
            | AddressMode::D16
            | AddressMode::D8
            | AddressMode::D16_R(_)
            | AddressMode::MR_D8(_)
            | AddressMode::MR(_)
            | AddressMode::A16_R(_)
            | AddressMode::R_A16(_) => unreachable!("not used"),
            AddressMode::R(r1) => {
                let lo: u16 = Stack::pop(&mut cpu.registers, &mut cpu.bus) as u16;
                cpu.update_cycles(1);

                let hi: u16 = Stack::pop(&mut cpu.registers, &mut cpu.bus) as u16;
                cpu.update_cycles(1);

                let n = (hi << 8) | lo;
                cpu.registers.set_register(r1, n);

                if r1 == RegisterType::AF {
                    cpu.registers.set_register(r1, n & 0xFFF0);
                }
            }
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
