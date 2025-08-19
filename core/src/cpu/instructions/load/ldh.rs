
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_ldh(&mut self, fetched_data: FetchedData) {
        match fetched_data.dest {
            DataDestination::Register(_) => {
                self.registers.a = fetched_data.value as u8;
            }
            DataDestination::Memory(addr) => {
                self.write_to_memory(addr | 0xFF00, fetched_data.value as u8);
            }
        }
    }
}
