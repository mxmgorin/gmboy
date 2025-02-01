use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, ExecutableInstruction};

#[derive(Debug, Clone, Copy)]
pub struct LdhInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for LdhInstruction {
    fn execute(&self, cpu: &mut Cpu) {
        match self.address_mode {
            AddressMode::IMP
            | AddressMode::R(_)
            | AddressMode::R_D16(_)
            | AddressMode::R_R(_, _)
            | AddressMode::MR_R(_, _)
            | AddressMode::R_D8(_)
            | AddressMode::R_MR(_, _)
            | AddressMode::R_HLI(_, _)
            | AddressMode::R_HLD(_, _)
            | AddressMode::HLI_R(_, _)
            | AddressMode::HLD_R(_, _)
            | AddressMode::HL_SPR(_, _)
            | AddressMode::D16
            | AddressMode::D8
            | AddressMode::D16_R(_)
            | AddressMode::MR_D8(_)
            | AddressMode::MR(_)
            | AddressMode::A16_R(_)
            | AddressMode::R_A16(_) => unreachable!("not used for LDH instruction"),
            // always uses A register
            AddressMode::R_A8(_r) => {
                let data = cpu.bus.read(0xFF00 | cpu.fetched_data);
                cpu.registers.a = data;
            }
            AddressMode::A8_R(_r) => {
                cpu.bus.write(cpu.mem_dest, cpu.registers.a);
            }
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
