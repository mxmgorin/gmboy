use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{DataDestination, FetchedData, RegisterType};
use crate::cpu::stack::Stack;
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct PopInstruction {
    pub address_mode: AddressMode,
}

// C1 POP BC
// D1 POP DE
// E1 POP HL
// F1 POP AF
impl ExecutableInstruction for PopInstruction {
    fn execute(&self, cpu: &mut Cpu, callback: &mut impl CpuCallback, fetched_data: FetchedData) {
        let DataDestination::Register(r) = fetched_data.dest else {
            unreachable!();
        };

        let lo = Stack::pop(cpu, callback) as u16;
        let hi = Stack::pop(cpu, callback) as u16;
        let addr = (hi << 8) | lo;

        if r == RegisterType::AF {
            cpu.registers.set_register(r, addr & 0xFFF0);
        } else {
            cpu.registers.set_register(r, addr);
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
