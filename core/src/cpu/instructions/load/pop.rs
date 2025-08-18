use crate::cpu::instructions::{AddressMode, ExecutableInstruction, InstructionArgs};
use crate::cpu::instructions::{DataDestination, FetchedData, RegisterType};

use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_pop(&mut self, fetched_data: FetchedData, _args: InstructionArgs) {
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

#[derive(Debug, Clone, Copy)]
pub struct PopInstruction {
    pub address_mode: AddressMode,
}

// C1 POP BC
// D1 POP DE
// E1 POP HL
// F1 POP AF
impl ExecutableInstruction for PopInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_pop(fetched_data, InstructionArgs::default(self.address_mode));
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
