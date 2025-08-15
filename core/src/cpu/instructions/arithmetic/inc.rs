use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu};

#[derive(Debug, Clone, Copy)]
pub struct IncInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for IncInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        let mut value = fetched_data.value.wrapping_add(1);

        match fetched_data.dest {
            DataDestination::Register(r) => {
                if r.is_16bit() {
                    cpu.clock.m_cycles(1);
                }

                cpu.registers.set_register(r, value);
                value = cpu.registers.read_register(r);
            }
            DataDestination::Memory(addr) => {
                // uses only HL
                value &= 0xFF; // Ensure it fits into 8 bits
                cpu.write_to_memory(addr, value as u8);
            }
        }

        if (cpu.current_opcode & 0x03) == 0x03 {
            return;
        }

        cpu.registers.flags.set_z(value == 0);
        cpu.registers.flags.set_n(false);
        cpu.registers.flags.set_h((value & 0x0F) == 0);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
