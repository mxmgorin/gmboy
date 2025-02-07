use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::FetchedData;

/// Load High Memory
#[derive(Debug, Clone, Copy)]
pub struct LdhInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for LdhInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        if let Some(addr) = fetched_data.dest_addr {
            cpu.write_to_memory(addr | 0xFF00, fetched_data.value as u8);
        } else {
            cpu.registers.a = cpu.bus.read(0xFF00 | fetched_data.src_addr.unwrap_or(0xFF));
        }

        cpu.update_cycles(1);
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
