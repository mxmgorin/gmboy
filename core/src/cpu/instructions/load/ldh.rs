use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu};

/// Load High Memory
#[derive(Debug, Clone, Copy)]
pub struct LdhInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for LdhInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        match fetched_data.dest {
            DataDestination::Register(_) => {
                cpu.registers.a = fetched_data.value as u8;
            }
            DataDestination::Memory(addr) => {
                cpu.write_to_memory(addr | 0xFF00, fetched_data.value as u8);
            }
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
