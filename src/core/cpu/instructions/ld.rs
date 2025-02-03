use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction, RegisterType};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct LdInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for LdInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        match self.address_mode {
            AddressMode::D8 | AddressMode::D16 | AddressMode::IMP => {
                unreachable!("Not used for LD")
            }
            AddressMode::R_D8(r1)
            | AddressMode::R_A8(r1)
            | AddressMode::A8_R(r1)
            | AddressMode::D16_R(r1)
            | AddressMode::MR_D8(r1)
            | AddressMode::MR(r1)
            | AddressMode::A16_R(r1)
            | AddressMode::R_A16(r1)
            | AddressMode::R(r1)
            | AddressMode::R_D16(r1) => {
                cpu.registers.set_register(r1, fetched_data.value);
            }

            AddressMode::R_R(r1, r2)
            | AddressMode::R_MR(r1, r2)
            | AddressMode::MR_R(r1, r2)
            | AddressMode::R_HLI(r1, r2)
            | AddressMode::R_HLD(r1, r2)
            | AddressMode::HLI_R(r1, r2)
            | AddressMode::HLD_R(r1, r2) => {
                if let Some(dest_addr) = fetched_data.dest_addr {
                    write_mem(cpu, r2, dest_addr, fetched_data.value);
                    return;
                }

                cpu.registers.set_register(r1, fetched_data.value);
            }
            // LD HL,SP+e8
            // Add the signed value e8 to SP and copy the result in HL.
            AddressMode::HL_SPR(r1, r2) => {
                let h_flag =
                    (cpu.registers.read_register(r2) & 0xF) + (fetched_data.value & 0xF) >= 0x10;
                let c_flag =
                    (cpu.registers.read_register(r2) & 0xFF) + (fetched_data.value & 0xFF) >= 0x100;
                
                cpu.registers.set_flags(0, 0, h_flag as i8, c_flag as i8);
                let value = fetched_data.value as u8; // truncate to 8 bits (+8e)
                cpu.registers
                    .set_register(r1, cpu.registers.read_register(r2) + value as u16);
            }
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}

fn write_mem(cpu: &mut Cpu, r2: RegisterType, addr: u16, value: u16) {
    if r2.is_16bit() {
        cpu.update_cycles(1);
        cpu.bus.write16(addr, value);
    } else {
        cpu.bus.write(addr, value as u8);
    }

    cpu.update_cycles(1);
}
