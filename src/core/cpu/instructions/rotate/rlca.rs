use crate::cpu::instructions::common::{AddressMode, ExecutableInstruction, FetchedData};
use crate::cpu::Cpu;

#[derive(Debug, Clone, Copy)]
pub struct RlcaInstruction;

impl ExecutableInstruction for RlcaInstruction {
    fn execute(&self, cpu: &mut Cpu, _fetched_data: FetchedData) {
        let mut u: u8 = cpu.registers.a;
        let c: bool = (u >> 7) & 1 != 0;
        u = (u << 1) | c as u8;
        cpu.registers.a = u;

        cpu.registers
            .flags
            .set(false.into(), false.into(), false.into(), Some(c.into()));
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
