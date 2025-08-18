use crate::cpu::instructions::{AddressMode, ExecutableInstruction, InstructionArgs};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_push(&mut self, fetched_data: FetchedData, _args: InstructionArgs) {
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

#[derive(Debug, Clone, Copy)]
pub struct PushInstruction {
    pub address_mode: AddressMode,
}

// C5: PUSH BC
// D5: PUSH DE
// E5: PUSH HL
// F5: PUSH AF
impl ExecutableInstruction for PushInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_push(fetched_data, InstructionArgs::default(self.address_mode));
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
