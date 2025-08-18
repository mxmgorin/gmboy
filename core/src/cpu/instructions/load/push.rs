use crate::cpu::instructions::InstructionSpec;
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_push(&mut self, fetched_data: FetchedData, _args: InstructionSpec) {
        let DataDestination::Register(r) = fetched_data.dest else {
            unreachable!();
        };

        self.clock.m_cycles(1);

        let hi = (self.registers.read_register(r) >> 8) & 0xFF;
        self.push(hi as u8);

        let lo = self.registers.read_register(r) & 0xFF;
        self.push(lo as u8);
    }
}
