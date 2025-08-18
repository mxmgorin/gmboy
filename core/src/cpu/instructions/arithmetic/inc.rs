use crate::cpu::instructions::{AddressMode, ExecutableInstruction, InstructionArgs};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_inc(&mut self, fetched_data: FetchedData, _args: InstructionArgs) {
        let mut value = fetched_data.value.wrapping_add(1);

        match fetched_data.dest {
            DataDestination::Register(r) => {
                if r.is_16bit() {
                    self.clock.m_cycles(1);
                }

                self.registers.set_register(r, value);
                value = self.registers.read_register(r);
            }
            DataDestination::Memory(addr) => {
                // uses only HL
                value &= 0xFF; // Ensure it fits into 8 bits
                self.write_to_memory(addr, value as u8);
            }
        }

        if (self.current_opcode & 0x03) == 0x03 {
            return;
        }

        self.registers.flags.set_z(value == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h((value & 0x0F) == 0);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct IncInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for IncInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_inc(fetched_data, InstructionArgs::default(self.address_mode));
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
