use crate::cpu::instructions::jump::ret::RetInstruction;
use crate::cpu::instructions::FetchedData;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct RetiInstruction {
    pub ret_instruction: RetInstruction,
}

impl Default for RetiInstruction {
    fn default() -> Self {
        Self::new()
    }
}

impl RetiInstruction {
    pub const fn new() -> RetiInstruction {
        Self {
            ret_instruction: RetInstruction {
                condition_type: None,
            },
        }
    }
}

impl ExecutableInstruction for RetiInstruction {
    fn execute(&self, cpu: &mut Cpu, callback: &mut impl CpuCallback, fetched_data: FetchedData) {
        cpu.bus.io.interrupts.ime = true;
        self.ret_instruction.execute(cpu, callback, fetched_data);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
