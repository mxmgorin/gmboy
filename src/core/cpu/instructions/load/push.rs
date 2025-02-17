use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::stack::Stack;
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct PushInstruction {
    pub address_mode: AddressMode,
}

// C5: PUSH BC
// D5: PUSH DE
// E5: PUSH HL
// F5: PUSH AF
impl ExecutableInstruction for PushInstruction {
    fn execute(&self, cpu: &mut Cpu, callback: &mut impl CpuCallback, fetched_data: FetchedData) {
        let DataDestination::Register(r) = fetched_data.dest else {
            unreachable!();
        };

        callback.m_cycles(1, &mut cpu.bus);

        let hi = (cpu.registers.read_register(r) >> 8) & 0xFF;
        Stack::push(cpu, hi as u8, callback);

        let lo = cpu.registers.read_register(r) & 0xFF;
        Stack::push(cpu, lo as u8, callback);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
