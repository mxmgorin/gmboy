use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu};

#[derive(Debug, Clone, Copy)]
pub struct RlaInstruction;

impl ExecutableInstruction for RlaInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        let u: u8 = cpu.registers.a;
        let cf: u8 = cpu.registers.flags.get_c() as u8;
        let c: u8 = (u >> 7) & 1;

        cpu.registers.a = (u << 1) | cf;

        cpu.registers.flags.set_z(false);
        cpu.registers.flags.set_n(false);
        cpu.registers.flags.set_h(false);
        cpu.registers.flags.set_c(c != 0);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
