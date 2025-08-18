use crate::cpu::instructions::{AddressMode, ExecutableInstruction, InstructionArgs};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_dec(&mut self, fetched_data: FetchedData, _args: InstructionArgs) {
        let mut value = fetched_data.value.wrapping_sub(1);

        match fetched_data.dest {
            DataDestination::Register(r) => {
                if r.is_16bit() {
                    self.clock.m_cycles(1);
                }

                self.registers.set_register(r, value);
                value = self.registers.read_register(r);
            }
            DataDestination::Memory(addr) => {
                self.write_to_memory(addr, value as u8);
            }
        }

        if (self.current_opcode & 0x0B) == 0x0B {
            return;
        }

        self.registers.flags.set_z(value == 0);
        self.registers.flags.set_n(true);
        self.registers.flags.set_h((value & 0x0F) == 0x0F);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DecInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for DecInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_dec(fetched_data, InstructionArgs::default(self.address_mode));
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
