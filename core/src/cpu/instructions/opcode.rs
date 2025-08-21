use crate::cpu::instructions::condition::JumpCondition;
use crate::cpu::fetch::AddressMode;
use crate::cpu::instructions::*;
use crate::cpu::RegisterType;

pub const INSTRUCTIONS: [Instruction; INSTRUCTIONS_COUNT] = {
    let mut instructions = {
        let mut array = [Instruction::unknown(0); INSTRUCTIONS_COUNT];
        let mut i = 0;
        while i < INSTRUCTIONS_COUNT {
            array[i] = Instruction::unknown(i as u8);
            i += 1;
        }
        array
    };

    // 0x0X
    instructions[0x00] = Instruction::new(Mnemonic::Nop, None, 0, AddressMode::IMP);
    instructions[0x01] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_D16(RegisterType::BC));
    instructions[0x02] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::MR_R(RegisterType::BC, RegisterType::A),
    );
    instructions[0x03] = Instruction::new(Mnemonic::Inc, None, 0, AddressMode::R(RegisterType::BC));
    instructions[0x04] = Instruction::new(Mnemonic::Inc, None, 0, AddressMode::R(RegisterType::B));
    instructions[0x05] = Instruction::new(Mnemonic::Dec, None, 0, AddressMode::R(RegisterType::B));
    instructions[0x06] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_D8(RegisterType::B));
    instructions[0x07] = Instruction::new(Mnemonic::Rlca, None, 0, AddressMode::IMP);
    instructions[0x08] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::A16_R(RegisterType::SP));
    instructions[0x09] = Instruction::new(
        Mnemonic::Add,
        None,
        0,
        AddressMode::R_R(RegisterType::HL, RegisterType::BC),
    );
    instructions[0x0A] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_MR(RegisterType::A, RegisterType::BC),
    );
    instructions[0x0B] = Instruction::new(Mnemonic::Dec, None, 0, AddressMode::R(RegisterType::BC));
    instructions[0x0C] = Instruction::new(Mnemonic::Inc, None, 0, AddressMode::R(RegisterType::C));
    instructions[0x0D] = Instruction::new(Mnemonic::Dec, None, 0, AddressMode::R(RegisterType::C));
    instructions[0x0E] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_D8(RegisterType::C));
    instructions[0x0F] = Instruction::new(Mnemonic::Rrca, None, 0, AddressMode::IMP);

    // 0x1X
    instructions[0x10] = Instruction::new(Mnemonic::Stop, None, 0, AddressMode::IMP);
    instructions[0x11] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_D16(RegisterType::DE));
    instructions[0x12] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::MR_R(RegisterType::DE, RegisterType::A),
    );
    instructions[0x13] = Instruction::new(Mnemonic::Inc, None, 0, AddressMode::R(RegisterType::DE));
    instructions[0x14] = Instruction::new(Mnemonic::Inc, None, 0, AddressMode::R(RegisterType::D));
    instructions[0x15] = Instruction::new(Mnemonic::Dec, None, 0, AddressMode::R(RegisterType::D));
    instructions[0x16] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_D8(RegisterType::D));
    instructions[0x17] = Instruction::new(Mnemonic::RLA, None, 0, AddressMode::IMP);
    instructions[0x18] = Instruction::new(Mnemonic::Jr, None, 0, AddressMode::D8);
    instructions[0x19] = Instruction::new(
        Mnemonic::Add,
        None,
        0,
        AddressMode::R_R(RegisterType::HL, RegisterType::DE),
    );
    instructions[0x1A] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_MR(RegisterType::A, RegisterType::DE),
    );
    instructions[0x1B] = Instruction::new(Mnemonic::Dec, None, 0, AddressMode::R(RegisterType::DE));
    instructions[0x1C] = Instruction::new(Mnemonic::Inc, None, 0, AddressMode::R(RegisterType::E));
    instructions[0x1D] = Instruction::new(Mnemonic::Dec, None, 0, AddressMode::R(RegisterType::E));
    instructions[0x1E] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_D8(RegisterType::E));
    instructions[0x1F] = Instruction::new(Mnemonic::Rra, None, 0, AddressMode::IMP);

    // 0x2X
    instructions[0x20] =
        Instruction::new(Mnemonic::Jr, Some(JumpCondition::NZ), 0, AddressMode::D8);
    instructions[0x21] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_D16(RegisterType::HL));
    instructions[0x22] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::HLI_R(RegisterType::A));
    instructions[0x23] = Instruction::new(Mnemonic::Inc, None, 0, AddressMode::R(RegisterType::HL));
    instructions[0x24] = Instruction::new(Mnemonic::Inc, None, 0, AddressMode::R(RegisterType::H));
    instructions[0x25] = Instruction::new(Mnemonic::Dec, None, 0, AddressMode::R(RegisterType::H));
    instructions[0x26] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_D8(RegisterType::H));
    instructions[0x27] = Instruction::new(Mnemonic::Daa, None, 0, AddressMode::IMP);
    instructions[0x28] = Instruction::new(Mnemonic::Jr, Some(JumpCondition::Z), 0, AddressMode::D8);
    instructions[0x29] = Instruction::new(
        Mnemonic::Add,
        None,
        0,
        AddressMode::R_R(RegisterType::HL, RegisterType::HL),
    );
    instructions[0x2A] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_HLI(RegisterType::A));
    instructions[0x2B] = Instruction::new(Mnemonic::Dec, None, 0, AddressMode::R(RegisterType::HL));
    instructions[0x2C] = Instruction::new(Mnemonic::Inc, None, 0, AddressMode::R(RegisterType::L));
    instructions[0x2D] = Instruction::new(Mnemonic::Dec, None, 0, AddressMode::R(RegisterType::L));
    instructions[0x2E] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_D8(RegisterType::L));
    instructions[0x2F] = Instruction::new(Mnemonic::Cpl, None, 0, AddressMode::IMP);

    // 0x3X
    instructions[0x30] =
        Instruction::new(Mnemonic::Jr, Some(JumpCondition::NC), 0, AddressMode::D8);
    instructions[0x31] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_D16(RegisterType::SP));
    instructions[0x32] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::HLD_R(RegisterType::A));
    instructions[0x33] = Instruction::new(Mnemonic::Inc, None, 0, AddressMode::R(RegisterType::SP));
    instructions[0x34] =
        Instruction::new(Mnemonic::Inc, None, 0, AddressMode::MR(RegisterType::HL));
    instructions[0x35] =
        Instruction::new(Mnemonic::Dec, None, 0, AddressMode::MR(RegisterType::HL));
    instructions[0x36] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::MR_D8(RegisterType::HL));
    instructions[0x37] = Instruction::new(Mnemonic::Scf, None, 0, AddressMode::IMP);
    instructions[0x38] = Instruction::new(Mnemonic::Jr, Some(JumpCondition::C), 0, AddressMode::D8);
    instructions[0x39] = Instruction::new(
        Mnemonic::Add,
        None,
        0,
        AddressMode::R_R(RegisterType::HL, RegisterType::SP),
    );
    instructions[0x3A] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_HLD(RegisterType::A));
    instructions[0x3B] = Instruction::new(Mnemonic::Dec, None, 0, AddressMode::R(RegisterType::SP));
    instructions[0x3C] = Instruction::new(Mnemonic::Inc, None, 0, AddressMode::R(RegisterType::A));
    instructions[0x3D] = Instruction::new(Mnemonic::Dec, None, 0, AddressMode::R(RegisterType::A));
    instructions[0x3E] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_D8(RegisterType::A));
    instructions[0x3F] = Instruction::new(Mnemonic::Ccf, None, 0, AddressMode::IMP);

    // 0x4X
    instructions[0x40] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::B, RegisterType::B),
    );
    instructions[0x41] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::B, RegisterType::C),
    );
    instructions[0x42] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::B, RegisterType::D),
    );
    instructions[0x43] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::B, RegisterType::E),
    );
    instructions[0x44] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::B, RegisterType::H),
    );
    instructions[0x45] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::B, RegisterType::L),
    );
    instructions[0x46] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_MR(RegisterType::B, RegisterType::HL),
    );
    instructions[0x47] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::B, RegisterType::A),
    );
    instructions[0x48] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::C, RegisterType::B),
    );
    instructions[0x49] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::C, RegisterType::C),
    );
    instructions[0x4A] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::C, RegisterType::D),
    );
    instructions[0x4B] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::C, RegisterType::E),
    );
    instructions[0x4C] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::C, RegisterType::H),
    );
    instructions[0x4D] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::C, RegisterType::L),
    );
    instructions[0x4E] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_MR(RegisterType::C, RegisterType::HL),
    );
    instructions[0x4F] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::C, RegisterType::A),
    );

    // 0x5X
    instructions[0x50] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::D, RegisterType::B),
    );
    instructions[0x51] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::D, RegisterType::C),
    );
    instructions[0x52] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::D, RegisterType::D),
    );
    instructions[0x53] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::D, RegisterType::E),
    );
    instructions[0x54] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::D, RegisterType::H),
    );
    instructions[0x55] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::D, RegisterType::L),
    );
    instructions[0x56] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_MR(RegisterType::D, RegisterType::HL),
    );
    instructions[0x57] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::D, RegisterType::A),
    );
    instructions[0x58] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::E, RegisterType::B),
    );
    instructions[0x59] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::E, RegisterType::C),
    );
    instructions[0x5A] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::E, RegisterType::D),
    );
    instructions[0x5B] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::E, RegisterType::E),
    );
    instructions[0x5C] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::E, RegisterType::H),
    );
    instructions[0x5D] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::E, RegisterType::L),
    );
    instructions[0x5E] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_MR(RegisterType::E, RegisterType::HL),
    );
    instructions[0x5F] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::E, RegisterType::A),
    );

    // 0x6X
    instructions[0x60] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::H, RegisterType::B),
    );
    instructions[0x61] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::H, RegisterType::C),
    );
    instructions[0x62] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::H, RegisterType::D),
    );
    instructions[0x63] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::H, RegisterType::E),
    );
    instructions[0x64] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::H, RegisterType::H),
    );
    instructions[0x65] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::H, RegisterType::L),
    );
    instructions[0x66] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_MR(RegisterType::H, RegisterType::HL),
    );
    instructions[0x67] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::H, RegisterType::A),
    );
    instructions[0x68] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::L, RegisterType::B),
    );
    instructions[0x69] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::L, RegisterType::C),
    );
    instructions[0x6A] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::L, RegisterType::D),
    );
    instructions[0x6B] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::L, RegisterType::E),
    );
    instructions[0x6C] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::L, RegisterType::H),
    );
    instructions[0x6D] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::L, RegisterType::L),
    );
    instructions[0x6E] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_MR(RegisterType::L, RegisterType::HL),
    );
    instructions[0x6F] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::L, RegisterType::A),
    );

    // 0x7X
    instructions[0x76] = Instruction::new(Mnemonic::Halt, None, 0, AddressMode::IMP);
    instructions[0x70] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::MR_R(RegisterType::HL, RegisterType::B),
    );
    instructions[0x71] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::MR_R(RegisterType::HL, RegisterType::C),
    );
    instructions[0x72] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::MR_R(RegisterType::HL, RegisterType::D),
    );
    instructions[0x73] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::MR_R(RegisterType::HL, RegisterType::E),
    );
    instructions[0x74] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::MR_R(RegisterType::HL, RegisterType::H),
    );
    instructions[0x75] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::MR_R(RegisterType::HL, RegisterType::L),
    );
    instructions[0x77] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::MR_R(RegisterType::HL, RegisterType::A),
    );
    instructions[0x78] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::B),
    );
    instructions[0x79] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::C),
    );
    instructions[0x7A] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::D),
    );
    instructions[0x7B] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::E),
    );
    instructions[0x7C] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::H),
    );
    instructions[0x7D] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::L),
    );
    instructions[0x7E] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    );
    instructions[0x7F] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::A),
    );

    // 0x8X
    instructions[0x80] = Instruction::new(
        Mnemonic::Add,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::B),
    );
    instructions[0x81] = Instruction::new(
        Mnemonic::Add,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::C),
    );
    instructions[0x82] = Instruction::new(
        Mnemonic::Add,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::D),
    );
    instructions[0x83] = Instruction::new(
        Mnemonic::Add,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::E),
    );
    instructions[0x84] = Instruction::new(
        Mnemonic::Add,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::H),
    );
    instructions[0x85] = Instruction::new(
        Mnemonic::Add,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::L),
    );
    instructions[0x86] = Instruction::new(
        Mnemonic::Add,
        None,
        0,
        AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    );
    instructions[0x87] = Instruction::new(
        Mnemonic::Add,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::A),
    );
    instructions[0x88] = Instruction::new(
        Mnemonic::Adc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::B),
    );
    instructions[0x89] = Instruction::new(
        Mnemonic::Adc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::C),
    );
    instructions[0x8A] = Instruction::new(
        Mnemonic::Adc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::D),
    );
    instructions[0x8B] = Instruction::new(
        Mnemonic::Adc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::E),
    );
    instructions[0x8C] = Instruction::new(
        Mnemonic::Adc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::H),
    );
    instructions[0x8D] = Instruction::new(
        Mnemonic::Adc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::L),
    );
    instructions[0x8E] = Instruction::new(
        Mnemonic::Adc,
        None,
        0,
        AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    );
    instructions[0x8F] = Instruction::new(
        Mnemonic::Adc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::A),
    );

    // 0x9X
    instructions[0x90] = Instruction::new(
        Mnemonic::Sub,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::B),
    );
    instructions[0x91] = Instruction::new(
        Mnemonic::Sub,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::C),
    );
    instructions[0x92] = Instruction::new(
        Mnemonic::Sub,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::D),
    );
    instructions[0x93] = Instruction::new(
        Mnemonic::Sub,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::E),
    );
    instructions[0x94] = Instruction::new(
        Mnemonic::Sub,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::H),
    );
    instructions[0x95] = Instruction::new(
        Mnemonic::Sub,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::L),
    );
    instructions[0x96] = Instruction::new(
        Mnemonic::Sub,
        None,
        0,
        AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    );
    instructions[0x97] = Instruction::new(
        Mnemonic::Sub,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::A),
    );
    instructions[0x98] = Instruction::new(
        Mnemonic::Sbc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::B),
    );
    instructions[0x99] = Instruction::new(
        Mnemonic::Sbc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::C),
    );
    instructions[0x9A] = Instruction::new(
        Mnemonic::Sbc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::D),
    );
    instructions[0x9B] = Instruction::new(
        Mnemonic::Sbc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::E),
    );
    instructions[0x9C] = Instruction::new(
        Mnemonic::Sbc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::H),
    );
    instructions[0x9D] = Instruction::new(
        Mnemonic::Sbc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::L),
    );
    instructions[0x9E] = Instruction::new(
        Mnemonic::Sbc,
        None,
        0,
        AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    );
    instructions[0x9F] = Instruction::new(
        Mnemonic::Sbc,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::A),
    );

    // 0xAX
    instructions[0xA0] = Instruction::new(
        Mnemonic::And,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::B),
    );
    instructions[0xA1] = Instruction::new(
        Mnemonic::And,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::C),
    );
    instructions[0xA2] = Instruction::new(
        Mnemonic::And,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::D),
    );
    instructions[0xA3] = Instruction::new(
        Mnemonic::And,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::E),
    );
    instructions[0xA4] = Instruction::new(
        Mnemonic::And,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::H),
    );
    instructions[0xA5] = Instruction::new(
        Mnemonic::And,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::L),
    );
    instructions[0xA6] = Instruction::new(
        Mnemonic::And,
        None,
        0,
        AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    );
    instructions[0xA7] = Instruction::new(
        Mnemonic::And,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::A),
    );
    instructions[0xA8] = Instruction::new(
        Mnemonic::Xor,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::B),
    );
    instructions[0xA9] = Instruction::new(
        Mnemonic::Xor,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::C),
    );
    instructions[0xAA] = Instruction::new(
        Mnemonic::Xor,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::D),
    );
    instructions[0xAB] = Instruction::new(
        Mnemonic::Xor,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::E),
    );
    instructions[0xAC] = Instruction::new(
        Mnemonic::Xor,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::H),
    );
    instructions[0xAD] = Instruction::new(
        Mnemonic::Xor,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::L),
    );
    instructions[0xAE] = Instruction::new(
        Mnemonic::Xor,
        None,
        0,
        AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    );
    instructions[0xAF] = Instruction::new(
        Mnemonic::Xor,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::A),
    );

    // 0xBX
    instructions[0xB0] = Instruction::new(
        Mnemonic::Or,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::B),
    );
    instructions[0xB1] = Instruction::new(
        Mnemonic::Or,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::C),
    );
    instructions[0xB2] = Instruction::new(
        Mnemonic::Or,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::D),
    );
    instructions[0xB3] = Instruction::new(
        Mnemonic::Or,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::E),
    );
    instructions[0xB4] = Instruction::new(
        Mnemonic::Or,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::H),
    );
    instructions[0xB5] = Instruction::new(
        Mnemonic::Or,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::L),
    );
    instructions[0xB6] = Instruction::new(
        Mnemonic::Or,
        None,
        0,
        AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    );
    instructions[0xB7] = Instruction::new(
        Mnemonic::Or,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::A),
    );
    instructions[0xB8] = Instruction::new(
        Mnemonic::Cp,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::B),
    );
    instructions[0xB9] = Instruction::new(
        Mnemonic::Cp,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::C),
    );
    instructions[0xBA] = Instruction::new(
        Mnemonic::Cp,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::D),
    );
    instructions[0xBB] = Instruction::new(
        Mnemonic::Cp,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::E),
    );
    instructions[0xBC] = Instruction::new(
        Mnemonic::Cp,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::H),
    );
    instructions[0xBD] = Instruction::new(
        Mnemonic::Cp,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::L),
    );
    instructions[0xBE] = Instruction::new(
        Mnemonic::Cp,
        None,
        0,
        AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    );
    instructions[0xBF] = Instruction::new(
        Mnemonic::Cp,
        None,
        0,
        AddressMode::R_R(RegisterType::A, RegisterType::A),
    );

    // 0xCX
    instructions[0xC0] =
        Instruction::new(Mnemonic::Ret, Some(JumpCondition::NZ), 0, AddressMode::IMP);
    instructions[0xC1] = Instruction::new(Mnemonic::Pop, None, 0, AddressMode::R(RegisterType::BC));
    instructions[0xC2] =
        Instruction::new(Mnemonic::Jp, Some(JumpCondition::NZ), 0, AddressMode::D16);
    instructions[0xC3] = Instruction::new(Mnemonic::Jp, None, 0, AddressMode::D16);
    instructions[0xC5] =
        Instruction::new(Mnemonic::Push, None, 0, AddressMode::R(RegisterType::BC));
    instructions[0xC6] =
        Instruction::new(Mnemonic::Add, None, 0, AddressMode::R_D8(RegisterType::A));
    instructions[0xC7] = Instruction::new(Mnemonic::Rst, None, 0x00, AddressMode::IMP);
    instructions[0xC4] =
        Instruction::new(Mnemonic::Call, Some(JumpCondition::NZ), 0, AddressMode::D16);
    instructions[0xC8] =
        Instruction::new(Mnemonic::Ret, Some(JumpCondition::Z), 0, AddressMode::IMP);
    instructions[0xC9] = Instruction::new(Mnemonic::Ret, None, 0, AddressMode::IMP);
    instructions[0xCC] =
        Instruction::new(Mnemonic::Call, Some(JumpCondition::Z), 0, AddressMode::D16);
    instructions[0xCD] = Instruction::new(Mnemonic::Call, None, 0, AddressMode::D16);
    instructions[0xCA] =
        Instruction::new(Mnemonic::Jp, Some(JumpCondition::Z), 0, AddressMode::D16);
    instructions[0xCB] = Instruction::new(Mnemonic::Prefix, None, 0, AddressMode::D8);
    instructions[0xCE] =
        Instruction::new(Mnemonic::Adc, None, 0, AddressMode::R_D8(RegisterType::A));
    instructions[0xCF] = Instruction::new(Mnemonic::Rst, None, 0x08, AddressMode::IMP);

    // 0xDX
    instructions[0xD0] =
        Instruction::new(Mnemonic::Ret, Some(JumpCondition::NC), 0, AddressMode::IMP);
    instructions[0xD1] = Instruction::new(Mnemonic::Pop, None, 0, AddressMode::R(RegisterType::DE));
    instructions[0xD2] =
        Instruction::new(Mnemonic::Jp, Some(JumpCondition::NC), 0, AddressMode::D16);
    instructions[0xD4] =
        Instruction::new(Mnemonic::Call, Some(JumpCondition::NC), 0, AddressMode::D16);
    instructions[0xD5] =
        Instruction::new(Mnemonic::Push, None, 0, AddressMode::R(RegisterType::DE));
    instructions[0xD6] =
        Instruction::new(Mnemonic::Sub, None, 0, AddressMode::R_D8(RegisterType::A));
    instructions[0xD7] = Instruction::new(Mnemonic::Rst, None, 0x10, AddressMode::IMP);
    instructions[0xD8] =
        Instruction::new(Mnemonic::Ret, Some(JumpCondition::C), 0, AddressMode::IMP);
    instructions[0xD9] = Instruction::new(Mnemonic::Reti, None, 0, AddressMode::IMP);
    instructions[0xDC] =
        Instruction::new(Mnemonic::Call, Some(JumpCondition::C), 0, AddressMode::D16);
    instructions[0xDA] =
        Instruction::new(Mnemonic::Jp, Some(JumpCondition::C), 0, AddressMode::D16);
    instructions[0xDE] =
        Instruction::new(Mnemonic::Sbc, None, 0, AddressMode::R_D8(RegisterType::A));
    instructions[0xDF] = Instruction::new(Mnemonic::Rst, None, 0x18, AddressMode::IMP);

    // 0xEX
    instructions[0xE0] =
        Instruction::new(Mnemonic::Ldh, None, 0, AddressMode::A8_R(RegisterType::A));
    instructions[0xE1] = Instruction::new(Mnemonic::Pop, None, 0, AddressMode::R(RegisterType::HL));
    instructions[0xE2] = Instruction::new(
        Mnemonic::Ldh,
        None,
        0,
        AddressMode::MR_R(RegisterType::C, RegisterType::A),
    );
    instructions[0xE5] =
        Instruction::new(Mnemonic::Push, None, 0, AddressMode::R(RegisterType::HL));
    instructions[0xE6] =
        Instruction::new(Mnemonic::And, None, 0, AddressMode::R_D8(RegisterType::A));
    instructions[0xE7] = Instruction::new(Mnemonic::Rst, None, 0x20, AddressMode::IMP);
    instructions[0xE8] =
        Instruction::new(Mnemonic::Add, None, 0, AddressMode::R_D8(RegisterType::SP));
    instructions[0xEA] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::A16_R(RegisterType::A));
    instructions[0xE9] = Instruction::new(Mnemonic::Jp, None, 0, AddressMode::R(RegisterType::HL));
    instructions[0xEE] =
        Instruction::new(Mnemonic::Xor, None, 0, AddressMode::R_D8(RegisterType::A));
    instructions[0xEF] = Instruction::new(Mnemonic::Rst, None, 0x28, AddressMode::IMP);

    // 0xFX
    instructions[0xF0] =
        Instruction::new(Mnemonic::Ldh, None, 0, AddressMode::R_HA8(RegisterType::A));
    instructions[0xF1] = Instruction::new(Mnemonic::Pop, None, 0, AddressMode::R(RegisterType::AF));
    instructions[0xF2] = Instruction::new(
        Mnemonic::Ldh,
        None,
        0,
        AddressMode::R_HMR(RegisterType::A, RegisterType::C),
    );
    instructions[0xF3] = Instruction::new(Mnemonic::Di, None, 0, AddressMode::IMP);
    instructions[0xF5] =
        Instruction::new(Mnemonic::Push, None, 0, AddressMode::R(RegisterType::AF));
    instructions[0xF6] =
        Instruction::new(Mnemonic::Or, None, 0, AddressMode::R_D8(RegisterType::A));
    instructions[0xF7] = Instruction::new(Mnemonic::Rst, None, 0x30, AddressMode::IMP);
    instructions[0xF8] = Instruction::new(Mnemonic::Ld, None, 0, AddressMode::LH_SPi8);
    instructions[0xF9] = Instruction::new(
        Mnemonic::Ld,
        None,
        0,
        AddressMode::R_R(RegisterType::SP, RegisterType::HL),
    );
    instructions[0xFA] =
        Instruction::new(Mnemonic::Ld, None, 0, AddressMode::R_A16(RegisterType::A));
    instructions[0xFB] = Instruction::new(Mnemonic::Ei, None, 0, AddressMode::IMP);
    instructions[0xFE] =
        Instruction::new(Mnemonic::Cp, None, 0, AddressMode::R_D8(RegisterType::A));
    instructions[0xFF] = Instruction::new(Mnemonic::Rst, None, 0x38, AddressMode::IMP);

    instructions
};
