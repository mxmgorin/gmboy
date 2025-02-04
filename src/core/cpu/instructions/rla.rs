use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct RlaInstruction;

impl ExecutableInstruction for RlaInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        let u: u8 = cpu.registers.a;
        let cf: u8 = cpu.registers.f.get_c() as u8;
        let c: u8 = (u >> 7) & 1;

        cpu.registers.a = (u << 1) | cf;
        cpu.registers
            .f.set(0.into(), 0.into(), 0.into(), Some((c != 0) as i8));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
