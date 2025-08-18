use crate::cpu::instructions::{AddressMode, ExecutableInstruction, InstructionArgs, RegisterType};
use crate::cpu::instructions::{DataDestination, DataSource, FetchedData};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_ld(&mut self, fetched_data: FetchedData, args: InstructionArgs) {
        match fetched_data.dest {
            DataDestination::Register(r) => {
                if args.addr_mode == AddressMode::LH_SPi8 {
                    let h_flag = (self.registers.sp & 0xF) + (fetched_data.value & 0xF) >= 0x10;
                    let c_flag = (self.registers.sp & 0xFF) + (fetched_data.value & 0xFF) >= 0x100;

                    self.registers.flags.set_z(false);
                    self.registers.flags.set_n(false);
                    self.registers.flags.set_h(h_flag);
                    self.registers.flags.set_c(c_flag);

                    let offset_e = fetched_data.value as i8; // truncate to 8 bits (+8e)

                    self.registers.set_register(
                        RegisterType::HL,
                        self.registers.sp.wrapping_add(offset_e as u16),
                    );

                    self.clock.m_cycles(1);
                } else {
                    if let DataSource::Register(src_r) = fetched_data.source {
                        if r.is_16bit() && src_r.is_16bit() {
                            self.clock.m_cycles(1);
                        }
                    }

                    self.registers.set_register(r, fetched_data.value);
                }
            }
            DataDestination::Memory(addr) => match fetched_data.source {
                DataSource::Memory(_) => unreachable!(),
                DataSource::Register(r) | DataSource::MemoryRegister(r, _) => {
                    if r.is_16bit() {
                        self.write_to_memory(
                            addr + 1,
                            ((fetched_data.value >> 8) & 0xFF) as u8,
                        );
                        self.write_to_memory(addr, (fetched_data.value & 0xFF) as u8);
                    } else {
                        self.write_to_memory(addr, fetched_data.value as u8);
                    }
                }
                DataSource::Immediate => {
                    self.write_to_memory(addr, fetched_data.value as u8);
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LdInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for LdInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_ld(fetched_data, InstructionArgs::default(self.address_mode));
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
