use crate::cpu::instructions::InstructionSpec;
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline]
    pub fn execute_pop(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        let DataDestination::Register(r) = fetched_data.dest else {
            unreachable!();
        };

        let lo = self.pop() as u16;
        let hi = self.pop() as u16;
        let addr = (hi << 8) | lo;

        if r == RegisterType::AF {
            self.registers.set_register(r, addr & 0xFFF0);
        } else {
            self.registers.set_register(r, addr);
        }
    }
}
