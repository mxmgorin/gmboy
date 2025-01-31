use crate::core::instructions::common::{AddressMode, Instruction, RegisterType};
use crate::core::instructions::inc::IncInstruction;
use crate::core::instructions::ld::LdInstruction;
use crate::core::instructions::nop::NopInstruction;

const INSTRUCTIONS_LEN: usize = 0xFF;

pub const INSTRUCTIONS_BY_OPCODES: [Instruction; INSTRUCTIONS_LEN] = {
    let mut instructions = [Instruction::Nop(NopInstruction); INSTRUCTIONS_LEN];

    // 0x0X
    instructions[0x00] = Instruction::Nop(NopInstruction);
    instructions[0x01] = Instruction::Ld(LdInstruction { address_mode: AddressMode::R_D16(RegisterType::BC) });
    instructions[0x02] = Instruction::Ld(LdInstruction { address_mode: AddressMode::MR_R(RegisterType::BC, RegisterType::A) });
    instructions[0x03] = Instruction::Inc(IncInstruction { register_type: RegisterType::BC });
    instructions[0x04] = Instruction::Inc(IncInstruction { register_type: RegisterType::B });

    instructions[0x06] = Instruction::Ld(LdInstruction { address_mode: AddressMode::R_D8(RegisterType::B) });

    // 0x1X
    instructions[0x11] = Instruction::Ld(LdInstruction { address_mode: AddressMode::R_D16(RegisterType::DE) });
    instructions[0x12] = Instruction::Ld(LdInstruction { address_mode: AddressMode::MR_R(RegisterType::DE, RegisterType::A) });
    instructions[0x13] = Instruction::Inc(IncInstruction { register_type: RegisterType::DE });


    // 0x2X
    instructions[0x21] = Instruction::Ld(LdInstruction { address_mode: AddressMode::R_D16(RegisterType::HL) });
    instructions[0x22] = Instruction::Ld(LdInstruction { address_mode: AddressMode::R_HLI(RegisterType::HL, RegisterType::A) });
    instructions[0x23] = Instruction::Inc(IncInstruction { register_type: RegisterType::HL });

    // 0x3X
    instructions[0x31] = Instruction::Ld(LdInstruction { address_mode: AddressMode::R_D16(RegisterType::SP) });
    instructions[0x32] = Instruction::Ld(LdInstruction { address_mode: AddressMode::R_HLD(RegisterType::HL, RegisterType::A) });
    instructions[0x33] = Instruction::Inc(IncInstruction { register_type: RegisterType::SP });

    // 0x4X

    // 0x5X

    // 0x6X

    // 0x7X

    // 0x8X

    // 0x9X

    // 0xAX

    // 0xBX

    // 0xCX
    
    // 0xDX

    // 0xEX

    // 0xFX

    // todo: Add more instructions here...

    instructions
};