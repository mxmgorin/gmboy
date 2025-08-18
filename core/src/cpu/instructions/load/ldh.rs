use crate::cpu::instructions::{AddressMode, ExecutableInstruction, InstructionArgs};
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_ldh(&mut self, fetched_data: FetchedData, _args: InstructionArgs) {
        match fetched_data.dest {
            DataDestination::Register(_) => {
                self.registers.a = fetched_data.value as u8;
            }
            DataDestination::Memory(addr) => {
                self.write_to_memory(addr | 0xFF00, fetched_data.value as u8);
            }
        }
    }
}

/// Load High Memory
#[derive(Debug, Clone, Copy)]
pub struct LdhInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for LdhInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_ldh(fetched_data, InstructionArgs::default(self.address_mode));
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
