use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction, RegisterType};
use crate::core::cpu::{Cpu, FetchedData};

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
                if fetched_data.dest_is_mem {
                    write_dest_mem(cpu, r2, fetched_data);
                    return;
                }

                cpu.registers.set_register(r1, fetched_data.value);
            }
            AddressMode::HL_SPR(r1, r2) => {
                if fetched_data.dest_is_mem {
                    write_dest_mem(cpu, r2, fetched_data);
                    return;
                }

                let h_flag =
                    (cpu.registers.read_register(r2) & 0xF) + (fetched_data.value & 0xF) >= 0x10;
                let c_flag =
                    (cpu.registers.read_register(r2) & 0xFF) + (fetched_data.value & 0xFF) >= 0x100;
                cpu.registers.set_flags(0, 0, h_flag as i8, c_flag as i8);
                cpu.registers
                    .set_register(r1, cpu.registers.read_register(r2) + fetched_data.value);
                // todo: cast fetched_data to u8?
            }
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}

fn write_dest_mem(cpu: &mut Cpu, r2: RegisterType, fetched_data: FetchedData) {
    //LD (BC), A for instance...
    if r2.is_16bit() {
        cpu.update_cycles(1);
        cpu.bus.write16(fetched_data.mem_dest, fetched_data.value);
    } else {
        cpu.bus
            .write(fetched_data.mem_dest, fetched_data.value as u8);
    }

    cpu.update_cycles(1);
}
