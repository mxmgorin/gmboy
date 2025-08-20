
use crate::cpu::instructions::{DataDestination};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_ldh(&mut self) {
        match self.step_ctx.fetched_data.dest {
            DataDestination::Register(_) => {
                self.registers.a = self.step_ctx.fetched_data.value as u8;
            }
            DataDestination::Memory(addr) => {
                self.write_to_memory(addr | 0xFF00, self.step_ctx.fetched_data.value as u8);
            }
        }
    }
}
