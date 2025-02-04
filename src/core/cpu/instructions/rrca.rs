use crate::cpu::instructions::common::{AddressMode, ExecutableInstruction, FetchedData};
use crate::cpu::Cpu;

#[derive(Debug, Clone, Copy)]
pub struct RrcaInstruction;

impl ExecutableInstruction for RrcaInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        let b: u8 = cpu.registers.a & 1;
        cpu.registers.a >>= 1;
        cpu.registers.a |= b << 7;

        cpu.registers
            .flags.set(false.into(), false.into(), false.into(), Some(b != 0));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
