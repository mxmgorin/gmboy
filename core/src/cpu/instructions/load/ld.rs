use crate::cpu::instructions::{AddressMode, ExecutableInstruction, RegisterType};
use crate::cpu::instructions::{DataDestination, DataSource, FetchedData};
use crate::cpu::{Cpu};

#[derive(Debug, Clone, Copy)]
pub struct LdInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for LdInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        match fetched_data.dest {
            DataDestination::Register(r) => {
                if self.address_mode == AddressMode::LH_SPi8 {
                    let h_flag = (cpu.registers.sp & 0xF) + (fetched_data.value & 0xF) >= 0x10;
                    let c_flag = (cpu.registers.sp & 0xFF) + (fetched_data.value & 0xFF) >= 0x100;

                    cpu.registers.flags.set_z(false);
                    cpu.registers.flags.set_n(false);
                    cpu.registers.flags.set_h(h_flag);
                    cpu.registers.flags.set_c(c_flag);
                    
                    let offset_e = fetched_data.value as i8; // truncate to 8 bits (+8e)

                    cpu.registers.set_register(
                        RegisterType::HL,
                        cpu.registers.sp.wrapping_add(offset_e as u16),
                    );

                    cpu.clock.m_cycles(1);
                } else {
                    if let DataSource::Register(src_r) = fetched_data.source {
                        if r.is_16bit() && src_r.is_16bit() {
                            cpu.clock.m_cycles(1);
                        }
                    }

                    cpu.registers.set_register(r, fetched_data.value);
                }
            }
            DataDestination::Memory(addr) => match fetched_data.source {
                DataSource::Memory(_) => unreachable!(),
                DataSource::Register(r) | DataSource::MemoryRegister(r, _) => {
                    if r.is_16bit() {
                        cpu.write_to_memory(
                            addr + 1,
                            ((fetched_data.value >> 8) & 0xFF) as u8,
                        );
                        cpu.write_to_memory(addr, (fetched_data.value & 0xFF) as u8);
                    } else {
                        cpu.write_to_memory(addr, fetched_data.value as u8);
                    }
                }
                DataSource::Immediate => {
                    cpu.write_to_memory(addr, fetched_data.value as u8);
                }
            },
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
