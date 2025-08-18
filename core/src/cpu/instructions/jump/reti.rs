use crate::cpu::instructions::jump::ret::RetInstruction;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{FetchedData, InstructionArgs};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_reti(&mut self, fetched_data: FetchedData, args: InstructionArgs) {
        self.clock.bus.io.interrupts.ime = true;
        self.execute_ret(fetched_data, args);
    }
}

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
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.execute_reti(
            fetched_data,
            InstructionArgs::new(self.ret_instruction.condition_type, 0, self.get_address_mode()),
        );
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::IMP
    }
}
