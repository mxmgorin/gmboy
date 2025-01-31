use crate::core::cpu::Cpu;
use crate::core::instructions::common::{AddressMode, Instruction, InstructionType};

pub const OPCODE: u16 = 0x00;

pub const fn new() -> Instruction {
    Instruction {
        r#type: Some(InstructionType::NOP),
        address_mode: Some(AddressMode::IMP),
        register_1_type: None,
        register_2_type: None,
        condition_type: None,
        param: None,
        execute_fn: execute,
    }
}

pub fn execute(_instruction: &Instruction, _cpu: &mut Cpu) {}
