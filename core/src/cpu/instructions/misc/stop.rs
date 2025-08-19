use crate::cpu::instructions::{FetchedData};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_stop(&mut self, _fetched_data: FetchedData) {}
}
