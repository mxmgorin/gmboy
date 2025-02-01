use crate::core::instructions::call::CallInstruction;
use crate::core::instructions::ccf::CcfInstruction;
use crate::core::instructions::common::{AddressMode, ConditionType, Instruction, RegisterType};
use crate::core::instructions::cpl::CplInstruction;
use crate::core::instructions::daa::DaaInstruction;
use crate::core::instructions::dec::DecInstruction;
use crate::core::instructions::di::DiInstruction;
use crate::core::instructions::halt::HaltInstruction;
use crate::core::instructions::inc::IncInstruction;
use crate::core::instructions::jp::JpInstruction;
use crate::core::instructions::jr::JrInstruction;
use crate::core::instructions::ld::LdInstruction;
use crate::core::instructions::ldh::LdhInstruction;
use crate::core::instructions::nop::NopInstruction;
use crate::core::instructions::xor::XorInstruction;

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
        register_type: RegisterType::BC,
    });
    instructions[0x04] = Instruction::Inc(IncInstruction {
        register_type: RegisterType::B,
    });
    instructions[0x05] = Instruction::Dec(DecInstruction {
        register_type: RegisterType::B,
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
        address_mode: AddressMode::R_MR(RegisterType::B),
    });
    instructions[0x0B] = Instruction::Dec(DecInstruction {
        register_type: RegisterType::BC,
    });
    instructions[0x0C] = Instruction::Inc(IncInstruction {
        register_type: RegisterType::C,
    });
    instructions[0x0D] = Instruction::Dec(DecInstruction {
        register_type: RegisterType::C,
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
        register_type: RegisterType::DE,
    });
    instructions[0x14] = Instruction::Inc(IncInstruction {
        register_type: RegisterType::D,
    });
    instructions[0x15] = Instruction::Dec(DecInstruction {
        register_type: RegisterType::D,
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
        register_type: RegisterType::DE,
    });
    instructions[0x1C] = Instruction::Inc(IncInstruction {
        register_type: RegisterType::E,
    });
    instructions[0x1D] = Instruction::Dec(DecInstruction {
        register_type: RegisterType::E,
    });
    instructions[0x1E] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_D8(RegisterType::E),
    });
    //instructions[0x1F] = Instruction::RRA(LdInstruction {
    //    address_mode: AddressMode::R_D8(RegisterType::E),
    //});

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
        register_type: RegisterType::HL,
    });
    instructions[0x24] = Instruction::Inc(IncInstruction {
        register_type: RegisterType::H,
    });
    instructions[0x25] = Instruction::Dec(DecInstruction {
        register_type: RegisterType::H,
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
        register_type: RegisterType::HL,
    });
    instructions[0x2C] = Instruction::Inc(IncInstruction {
        register_type: RegisterType::L,
    });
    instructions[0x2D] = Instruction::Dec(DecInstruction {
        register_type: RegisterType::L,
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
        register_type: RegisterType::SP,
    });
    instructions[0x38] = Instruction::Jr(JrInstruction {
        condition_type: Some(ConditionType::C),
    });

    instructions[0x3F] = Instruction::Ccf(CcfInstruction);

    // 0x4X

    // 0x5X

    // 0x6X

    // 0x7X
    instructions[0x76] = Instruction::Halt(HaltInstruction);

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

    instructions[0xE9] = Instruction::Jp(JpInstruction {
        address_mode: AddressMode::R(RegisterType::HL),
        condition_type: None,
    });

    // 0xFX
    instructions[0xF0] = Instruction::Ldh(LdhInstruction {
        address_mode: AddressMode::R_A8(RegisterType::A),
    });

    instructions[0xF3] = Instruction::Di(DiInstruction);

    // todo: Add more instructions here...

    instructions
};
