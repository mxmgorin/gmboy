use crate::core::cpu::instructions::common::address_mode::AddressMode;
use crate::core::cpu::instructions::common::condition_type::ConditionType;
use crate::core::cpu::instructions::common::instruction::{Instruction, RegisterType};
use crate::core::cpu::instructions::*;
use crate::cpu::instructions::rra::RraInstruction;

const INSTRUCTIONS_LEN: usize = 0xFF;

pub const INSTRUCTIONS_BY_OPCODES: [Instruction; INSTRUCTIONS_LEN] = {
    let mut instructions = {
        let mut array = [Instruction::Unknown(0); INSTRUCTIONS_LEN];
        let mut i = 0;
        while i < INSTRUCTIONS_LEN {
            array[i] = Instruction::Unknown(i as u8);
            i += 1;
        }
        array
    };

    // 0x0X
    instructions[0x00] = Instruction::Nop(NopInstruction);
    instructions[0x01] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_D16(RegisterType::BC),
    });
    instructions[0x02] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::MR_R(RegisterType::BC, RegisterType::A),
    });
    instructions[0x03] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::R(RegisterType::BC),
    });
    instructions[0x04] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::R(RegisterType::B),
    });
    instructions[0x05] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::R(RegisterType::B),
    });
    instructions[0x06] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_D8(RegisterType::B),
    });
    //instructions[0x07] = Instruction::RLCA(LdInstruction { address_mode: AddressMode::R_D8(RegisterType::B) });
    instructions[0x08] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::A16_R(RegisterType::SP),
    });
    //instructions[0x09] = Instruction::Add(LdInstruction { address_mode: AddressMode::A16_R(RegisterType::SP) });
    instructions[0x0A] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::BC),
    });
    instructions[0x0B] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::R(RegisterType::BC),
    });
    instructions[0x0C] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::R(RegisterType::C),
    });
    instructions[0x0D] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::R(RegisterType::C),
    });
    instructions[0x0E] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_D8(RegisterType::C),
    });
    //instructions[0x0F] = Instruction::RRCA(LdInstruction {
    //    address_mode: AddressMode::R_D8(RegisterType::C),
    //});

    // 0x1X
    //instructions[0x10] = Instruction::STOP(LdInstruction {
    //    address_mode: AddressMode::R_D16(RegisterType::DE),
    //});
    instructions[0x11] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_D16(RegisterType::DE),
    });
    instructions[0x12] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::MR_R(RegisterType::DE, RegisterType::A),
    });
    instructions[0x13] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::R(RegisterType::DE),
    });
    instructions[0x14] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::R(RegisterType::D),
    });
    instructions[0x15] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::R(RegisterType::D),
    });
    instructions[0x16] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_D8(RegisterType::D),
    });
    //instructions[0x17] = Instruction::RLA(LdInstruction {
    //    address_mode: AddressMode::R_D8(RegisterType::D),
    //});
    instructions[0x18] = Instruction::Jr(JrInstruction {
        condition_type: None,
    });
    //instructions[0x19] = Instruction::ADD(LdInstruction {
    //    address_mode: AddressMode::R_D8(RegisterType::D),
    //});
    instructions[0x1A] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_A16(RegisterType::D),
    });
    instructions[0x1B] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::R(RegisterType::DE),
    });
    instructions[0x1C] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::R(RegisterType::E),
    });
    instructions[0x1D] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::R(RegisterType::E),
    });
    instructions[0x1E] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_D8(RegisterType::E),
    });
    instructions[0x1F] = Instruction::Rra(RraInstruction);

    // 0x2X
    instructions[0x20] = Instruction::Jr(JrInstruction {
        condition_type: Some(ConditionType::NZ),
    });
    instructions[0x21] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_D16(RegisterType::HL),
    });
    instructions[0x22] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_HLI(RegisterType::HL, RegisterType::A),
    });
    instructions[0x23] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::MR(RegisterType::HL),
    });
    instructions[0x24] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::R(RegisterType::H),
    });
    instructions[0x25] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::R(RegisterType::H),
    });
    instructions[0x26] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_D8(RegisterType::H),
    });
    instructions[0x27] = Instruction::Daa(DaaInstruction);
    instructions[0x28] = Instruction::Jr(JrInstruction {
        condition_type: Some(ConditionType::Z),
    });
    //instructions[0x29] = Instruction::ADD(LdInstruction {
    //    address_mode: AddressMode::R_D8(RegisterType::H),
    //});
    instructions[0x2A] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_HLI(RegisterType::A, RegisterType::HL),
    });
    instructions[0x2B] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::R(RegisterType::HL),
    });
    instructions[0x2C] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::R(RegisterType::L),
    });
    instructions[0x2D] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::R(RegisterType::L),
    });
    instructions[0x2E] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_D8(RegisterType::L),
    });
    instructions[0x2F] = Instruction::Cpl(CplInstruction);

    // 0x3X
    instructions[0x30] = Instruction::Jr(JrInstruction {
        condition_type: Some(ConditionType::NC),
    });
    instructions[0x31] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_D16(RegisterType::SP),
    });
    instructions[0x32] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_HLD(RegisterType::HL, RegisterType::A),
    });
    instructions[0x33] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::R(RegisterType::SP),
    });
    instructions[0x35] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::MR(RegisterType::HL),
    });
    instructions[0x36] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::MR_D8(RegisterType::HL),
    });
    instructions[0x38] = Instruction::Jr(JrInstruction {
        condition_type: Some(ConditionType::C),
    });
    instructions[0x3A] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_HLD(RegisterType::A, RegisterType::HL),
    });
    instructions[0x3B] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::R(RegisterType::SP),
    });
    instructions[0x3D] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::R(RegisterType::A),
    });
    instructions[0x3E] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_D8(RegisterType::A),
    });
    instructions[0x3F] = Instruction::Ccf(CcfInstruction);

    // 0x4X
    instructions[0x40] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::B, RegisterType::B),
    });
    instructions[0x41] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::B, RegisterType::C),
    });
    instructions[0x42] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::B, RegisterType::D),
    });
    instructions[0x43] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::B, RegisterType::E),
    });
    instructions[0x44] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::B, RegisterType::H),
    });
    instructions[0x45] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::B, RegisterType::L),
    });
    instructions[0x46] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_MR(RegisterType::B, RegisterType::HL),
    });
    instructions[0x47] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::B, RegisterType::A),
    });
    instructions[0x48] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::C, RegisterType::B),
    });
    instructions[0x49] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::C, RegisterType::C),
    });
    instructions[0x4A] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::C, RegisterType::D),
    });
    instructions[0x4B] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::C, RegisterType::E),
    });
    instructions[0x4C] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::C, RegisterType::H),
    });
    instructions[0x4D] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::C, RegisterType::L),
    });
    instructions[0x4E] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_MR(RegisterType::C, RegisterType::HL),
    });
    instructions[0x4F] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::C, RegisterType::A),
    });

    // 0x5X
    instructions[0x50] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::D, RegisterType::B),
    });
    instructions[0x51] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::D, RegisterType::C),
    });
    instructions[0x52] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::D, RegisterType::D),
    });
    instructions[0x53] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::D, RegisterType::E),
    });
    instructions[0x54] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::D, RegisterType::H),
    });
    instructions[0x55] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::D, RegisterType::L),
    });
    instructions[0x56] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_MR(RegisterType::D, RegisterType::HL),
    });
    instructions[0x57] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::D, RegisterType::A),
    });
    instructions[0x58] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::E, RegisterType::B),
    });
    instructions[0x59] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::E, RegisterType::C),
    });
    instructions[0x5A] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::E, RegisterType::D),
    });
    instructions[0x5B] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::E, RegisterType::E),
    });
    instructions[0x5C] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::E, RegisterType::H),
    });
    instructions[0x5D] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::E, RegisterType::L),
    });
    instructions[0x5E] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_MR(RegisterType::E, RegisterType::HL),
    });
    instructions[0x5F] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::E, RegisterType::A),
    });

    // 0x6X
    instructions[0x60] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::D, RegisterType::B),
    });
    instructions[0x61] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::H, RegisterType::C),
    });
    instructions[0x62] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::H, RegisterType::D),
    });
    instructions[0x63] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::H, RegisterType::E),
    });
    instructions[0x64] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::H, RegisterType::H),
    });
    instructions[0x65] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::H, RegisterType::L),
    });
    instructions[0x66] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_MR(RegisterType::H, RegisterType::HL),
    });
    instructions[0x67] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::H, RegisterType::A),
    });
    instructions[0x68] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::L, RegisterType::B),
    });
    instructions[0x69] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::L, RegisterType::C),
    });
    instructions[0x6A] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::L, RegisterType::D),
    });
    instructions[0x6B] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::L, RegisterType::E),
    });
    instructions[0x6C] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::L, RegisterType::H),
    });
    instructions[0x6D] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::L, RegisterType::L),
    });
    instructions[0x6E] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_MR(RegisterType::L, RegisterType::HL),
    });
    instructions[0x6F] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::L, RegisterType::A),
    });

    // 0x7X
    instructions[0x76] = Instruction::Halt(HaltInstruction);
    instructions[0x70] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::MR_R(RegisterType::D, RegisterType::B),
    });
    instructions[0x71] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::C),
    });
    instructions[0x72] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::D),
    });
    instructions[0x73] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::E),
    });
    instructions[0x74] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::H),
    });
    instructions[0x75] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::L),
    });
    instructions[0x77] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::A),
    });
    instructions[0x78] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
    });
    instructions[0x79] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
    });
    instructions[0x7A] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
    });
    instructions[0x7B] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
    });
    instructions[0x7C] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
    });
    instructions[0x7D] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
    });
    instructions[0x7E] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    });
    instructions[0x7F] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
    });

    // 0x8X

    // 0x9X

    // 0xAX
    instructions[0xAF] = Instruction::Xor(XorInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
    });

    // 0xBX

    // 0xCX
    instructions[0xC2] = Instruction::Jp(JpInstruction {
        address_mode: AddressMode::D16,
        condition_type: Some(ConditionType::NZ),
    });
    instructions[0xC3] = Instruction::Jp(JpInstruction {
        address_mode: AddressMode::D16,
        condition_type: None,
    });
    instructions[0xC4] = Instruction::Call(CallInstruction {
        condition_type: Some(ConditionType::NZ),
    });
    instructions[0xCC] = Instruction::Call(CallInstruction {
        condition_type: Some(ConditionType::Z),
    });
    instructions[0xCD] = Instruction::Call(CallInstruction {
        condition_type: None,
    });
    instructions[0xCA] = Instruction::Jp(JpInstruction {
        address_mode: AddressMode::D16,
        condition_type: Some(ConditionType::Z),
    });

    // 0xDX
    instructions[0xD2] = Instruction::Jp(JpInstruction {
        address_mode: AddressMode::D16,
        condition_type: Some(ConditionType::NC),
    });
    instructions[0xD4] = Instruction::Call(CallInstruction {
        condition_type: Some(ConditionType::NC),
    });
    instructions[0xDC] = Instruction::Call(CallInstruction {
        condition_type: Some(ConditionType::C),
    });
    instructions[0xDA] = Instruction::Jp(JpInstruction {
        address_mode: AddressMode::D16,
        condition_type: Some(ConditionType::C),
    });

    // 0xEX
    instructions[0xE0] = Instruction::Ldh(LdhInstruction {
        address_mode: AddressMode::A8_R(RegisterType::A),
    });
    instructions[0xE2] = Instruction::Ldh(LdhInstruction {
        address_mode: AddressMode::MR_R(RegisterType::C, RegisterType::A),
    });
    instructions[0xEA] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::A16_R(RegisterType::A),
    });

    instructions[0xE9] = Instruction::Jp(JpInstruction {
        address_mode: AddressMode::R(RegisterType::HL),
        condition_type: None,
    });

    // 0xFX
    instructions[0xF0] = Instruction::Ldh(LdhInstruction {
        address_mode: AddressMode::R_A8(RegisterType::A),
    });
    // LDH A,[C]
    // This is sometimes written as ‘LD A,[$FF00+C]’.
    instructions[0xF2] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::C),
    });
    instructions[0xF3] = Instruction::Di(DiInstruction {});
    instructions[0xF8] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::HL_SPR(RegisterType::HL, RegisterType::SP),
    });
    instructions[0xF9] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::SP, RegisterType::HL),
    });
    instructions[0xFA] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_A16(RegisterType::A),
    });
    instructions[0xF3] = Instruction::Di(DiInstruction);

    // todo: Add more instructions here...

    instructions
};
