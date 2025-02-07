use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct RlaInstruction;

impl ExecutableInstruction for RlaInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        let u: u8 = cpu.registers.a;
        let cf: u8 = cpu.registers.flags.get_c() as u8;
        let c: u8 = (u >> 7) & 1;

        cpu.registers.a = (u << 1) | cf;
        cpu.registers
            .flags
            .set(false.into(), false.into(), false.into(), Some(c != 0));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
