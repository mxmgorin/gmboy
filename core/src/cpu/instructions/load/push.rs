use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu};

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
        let DataDestination::Register(r) = fetched_data.dest else {
            unreachable!();
        };

        cpu.clock.m_cycles(1);

        let hi = (cpu.registers.read_register(r) >> 8) & 0xFF;
        cpu.push(hi as u8);

        let lo = cpu.registers.read_register(r) & 0xFF;
        cpu.push(lo as u8);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
