
use crate::cpu::instructions::{DataDestination};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_dec(&mut self) {
        let mut value = self.step_ctx.fetched_data.value.wrapping_sub(1);

        match self.step_ctx.fetched_data.dest {
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

        if (self.step_ctx.opcode & 0x0B) == 0x0B {
            return;
        }

        self.registers.flags.set_z(value == 0);
        self.registers.flags.set_n(true);
        self.registers.flags.set_h((value & 0x0F) == 0x0F);
    }
}
