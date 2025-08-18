use crate::cpu::instructions::{AddressMode, ExecutableInstruction, FetchedData, InstructionArgs};
use crate::cpu::{Cpu};

impl Cpu {
    #[inline]
    pub fn execute_or(&mut self, fetched_data: FetchedData, _args: InstructionArgs) {
        let value = fetched_data.value & 0xFF;
        self.registers.a |= value as u8;

        self.registers.flags.set_z(self.registers.a == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(false);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OrInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for OrInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_or(fetched_data, InstructionArgs::default(self.address_mode));
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}
