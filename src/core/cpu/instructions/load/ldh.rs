use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::{DataDestination, FetchedData};

/// Load High Memory
#[derive(Debug, Clone, Copy)]
pub struct LdhInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for LdhInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        match fetched_data.dest {
            DataDestination::Register(_) => {
                cpu.registers.a = cpu
                    .bus
                    .read(0xFF00 | fetched_data.source.get_addr().expect("must be set"));
            }
            DataDestination::Memory(addr) => {
                cpu.write_to_memory(addr | 0xFF00, fetched_data.value as u8)
            }
        }

        cpu.m_cycles(1);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
