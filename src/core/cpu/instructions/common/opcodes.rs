use crate::core::cpu::instructions::arithmetic::adc::AdcInstruction;
use crate::core::cpu::instructions::arithmetic::add::AddInstruction;
use crate::core::cpu::instructions::arithmetic::cp::CpInstruction;
use crate::core::cpu::instructions::arithmetic::sbc::SbcInstruction;
use crate::core::cpu::instructions::arithmetic::sub::SubInstruction;
use crate::core::cpu::instructions::bitwise::and::AndInstruction;
use crate::core::cpu::instructions::common::address_mode::AddressMode;
use crate::core::cpu::instructions::common::condition_type::ConditionType;
use crate::core::cpu::instructions::common::instruction::{Instruction, RegisterType};
use crate::core::cpu::instructions::jump::ret::RetInstruction;
use crate::core::cpu::instructions::jump::rst::RstInstruction;
use crate::core::cpu::instructions::load::pop::PopInstruction;
use crate::core::cpu::instructions::load::push::PushInstruction;
use crate::core::cpu::instructions::misc::prefix::PrefixInstruction;
use crate::core::cpu::instructions::misc::scf::ScfInstruction;
use crate::core::cpu::instructions::misc::stop::StopInstruction;
use crate::core::cpu::instructions::rotate::rla::RlaInstruction;
use crate::core::cpu::instructions::rotate::rlca::RlcaInstruction;
use crate::core::cpu::instructions::rotate::rra::RraInstruction;
use crate::core::cpu::instructions::rotate::rrca::RrcaInstruction;
use crate::core::cpu::instructions::*;

const INSTRUCTIONS_LEN: usize = 0xFF + 1;

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
    instructions[0x07] = Instruction::Rlca(RlcaInstruction);
    instructions[0x08] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::A16_R(RegisterType::SP),
    });
    instructions[0x09] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_R(RegisterType::HL, RegisterType::BC),
    });
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
    instructions[0x0F] = Instruction::Rrca(RrcaInstruction);

    // 0x1X
    instructions[0x10] = Instruction::Stop(StopInstruction);
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
    instructions[0x17] = Instruction::Rla(RlaInstruction);
    instructions[0x18] = Instruction::Jr(JrInstruction {
        condition_type: None,
    });
    instructions[0x19] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_R(RegisterType::HL, RegisterType::DE),
    });
    instructions[0x1A] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::DE),
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
        address_mode: AddressMode::HLI_R(RegisterType::A),
    });
    instructions[0x23] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::R(RegisterType::HL),
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
    instructions[0x29] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_R(RegisterType::HL, RegisterType::HL),
    });
    instructions[0x2A] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_HLI(RegisterType::A),
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
        address_mode: AddressMode::HLD_R(RegisterType::A),
    });
    instructions[0x33] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::R(RegisterType::SP),
    });
    instructions[0x34] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::MR(RegisterType::HL),
    });
    instructions[0x35] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::MR(RegisterType::HL),
    });
    instructions[0x36] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::MR_D8(RegisterType::HL),
    });
    instructions[0x37] = Instruction::Scf(ScfInstruction);
    instructions[0x38] = Instruction::Jr(JrInstruction {
        condition_type: Some(ConditionType::C),
    });
    instructions[0x39] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_R(RegisterType::HL, RegisterType::SP),
    });
    instructions[0x3A] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_HLD(RegisterType::A),
    });
    instructions[0x3B] = Instruction::Dec(DecInstruction {
        address_mode: AddressMode::R(RegisterType::SP),
    });
    instructions[0x3C] = Instruction::Inc(IncInstruction {
        address_mode: AddressMode::R(RegisterType::A),
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
        address_mode: AddressMode::R_R(RegisterType::H, RegisterType::B),
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
        address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::B),
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
    instructions[0x80] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
    });
    instructions[0x81] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
    });
    instructions[0x82] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
    });
    instructions[0x83] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
    });
    instructions[0x84] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
    });
    instructions[0x85] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
    });
    instructions[0x86] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    });
    instructions[0x87] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
    });
    instructions[0x88] = Instruction::Adc(AdcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
    });
    instructions[0x89] = Instruction::Adc(AdcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
    });
    instructions[0x8A] = Instruction::Adc(AdcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
    });
    instructions[0x8B] = Instruction::Adc(AdcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
    });
    instructions[0x8C] = Instruction::Adc(AdcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
    });
    instructions[0x8D] = Instruction::Adc(AdcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
    });
    instructions[0x8E] = Instruction::Adc(AdcInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    });
    instructions[0x8F] = Instruction::Adc(AdcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
    });

    // 0x9X
    instructions[0x90] = Instruction::Sub(SubInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
    });
    instructions[0x91] = Instruction::Sub(SubInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
    });
    instructions[0x92] = Instruction::Sub(SubInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
    });
    instructions[0x93] = Instruction::Sub(SubInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
    });
    instructions[0x94] = Instruction::Sub(SubInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
    });
    instructions[0x95] = Instruction::Sub(SubInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
    });
    instructions[0x96] = Instruction::Sub(SubInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    });
    instructions[0x97] = Instruction::Sub(SubInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
    });
    instructions[0x98] = Instruction::Sbc(SbcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
    });
    instructions[0x99] = Instruction::Sbc(SbcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
    });
    instructions[0x9A] = Instruction::Sbc(SbcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
    });
    instructions[0x9B] = Instruction::Sbc(SbcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
    });
    instructions[0x9C] = Instruction::Sbc(SbcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
    });
    instructions[0x9D] = Instruction::Sbc(SbcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
    });
    instructions[0x9E] = Instruction::Sbc(SbcInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    });
    instructions[0x9F] = Instruction::Sbc(SbcInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
    });

    // 0xAX
    instructions[0xA0] = Instruction::And(AndInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
    });
    instructions[0xA1] = Instruction::And(AndInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
    });
    instructions[0xA2] = Instruction::And(AndInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
    });
    instructions[0xA3] = Instruction::And(AndInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
    });
    instructions[0xA4] = Instruction::And(AndInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
    });
    instructions[0xA5] = Instruction::And(AndInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
    });
    instructions[0xA6] = Instruction::And(AndInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    });
    instructions[0xA7] = Instruction::And(AndInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
    });
    instructions[0xA8] = Instruction::Xor(XorInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
    });
    instructions[0xA9] = Instruction::Xor(XorInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
    });
    instructions[0xAA] = Instruction::Xor(XorInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
    });
    instructions[0xAB] = Instruction::Xor(XorInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
    });
    instructions[0xAC] = Instruction::Xor(XorInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
    });
    instructions[0xAD] = Instruction::Xor(XorInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
    });
    instructions[0xAE] = Instruction::Xor(XorInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    });
    instructions[0xAF] = Instruction::Xor(XorInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
    });

    // 0xBX
    instructions[0xB0] = Instruction::Or(OrInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
    });
    instructions[0xB1] = Instruction::Or(OrInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
    });
    instructions[0xB2] = Instruction::Or(OrInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
    });
    instructions[0xB3] = Instruction::Or(OrInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
    });
    instructions[0xB4] = Instruction::Or(OrInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
    });
    instructions[0xB5] = Instruction::Or(OrInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
    });
    instructions[0xB6] = Instruction::Or(OrInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    });
    instructions[0xB7] = Instruction::Or(OrInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
    });
    instructions[0xB8] = Instruction::Cp(CpInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
    });
    instructions[0xB9] = Instruction::Cp(CpInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
    });
    instructions[0xBA] = Instruction::Cp(CpInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
    });
    instructions[0xBB] = Instruction::Cp(CpInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
    });
    instructions[0xBC] = Instruction::Cp(CpInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
    });
    instructions[0xBD] = Instruction::Cp(CpInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
    });
    instructions[0xBE] = Instruction::Cp(CpInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
    });
    instructions[0xBF] = Instruction::Cp(CpInstruction {
        address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
    });

    // 0xCX
    instructions[0xC0] = Instruction::Ret(RetInstruction {
        condition_type: Some(ConditionType::NZ),
    });
    instructions[0xC1] = Instruction::Pop(PopInstruction {
        address_mode: AddressMode::R(RegisterType::BC),
    });
    instructions[0xC2] = Instruction::Jp(JpInstruction {
        address_mode: AddressMode::D16,
        condition_type: Some(ConditionType::NZ),
    });
    instructions[0xC3] = Instruction::Jp(JpInstruction {
        address_mode: AddressMode::D16,
        condition_type: None,
    });
    instructions[0xC5] = Instruction::Push(PushInstruction {
        address_mode: AddressMode::R(RegisterType::BC),
    });
    instructions[0xC6] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::R_D8(RegisterType::A),
    });
    instructions[0xC7] = Instruction::Rst(RstInstruction { address: 0x00 });
    instructions[0xC4] = Instruction::Call(CallInstruction {
        condition_type: Some(ConditionType::NZ),
    });
    instructions[0xC8] = Instruction::Ret(RetInstruction {
        condition_type: Some(ConditionType::Z),
    });
    instructions[0xC9] = Instruction::Ret(RetInstruction {
        condition_type: None,
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
    instructions[0xCB] = Instruction::Prefix(PrefixInstruction);
    instructions[0xCE] = Instruction::Adc(AdcInstruction {
        address_mode: AddressMode::R_D8(RegisterType::A),
    });
    instructions[0xCF] = Instruction::Rst(RstInstruction { address: 0x08 });

    // 0xDX
    instructions[0xD0] = Instruction::Ret(RetInstruction {
        condition_type: Some(ConditionType::NC),
    });
    instructions[0xD1] = Instruction::Pop(PopInstruction {
        address_mode: AddressMode::R(RegisterType::DE),
    });
    instructions[0xD2] = Instruction::Jp(JpInstruction {
        address_mode: AddressMode::D16,
        condition_type: Some(ConditionType::NC),
    });
    instructions[0xD4] = Instruction::Call(CallInstruction {
        condition_type: Some(ConditionType::NC),
    });
    instructions[0xD5] = Instruction::Push(PushInstruction {
        address_mode: AddressMode::R(RegisterType::DE),
    });
    instructions[0xD6] = Instruction::Sub(SubInstruction {
        address_mode: AddressMode::R_D8(RegisterType::A),
    });
    instructions[0xD7] = Instruction::Rst(RstInstruction { address: 0x10 });
    instructions[0xD8] = Instruction::Ret(RetInstruction {
        condition_type: Some(ConditionType::C),
    });
    instructions[0xD9] = Instruction::Reti(RetiInstruction::new());
    instructions[0xDC] = Instruction::Call(CallInstruction {
        condition_type: Some(ConditionType::C),
    });
    instructions[0xDA] = Instruction::Jp(JpInstruction {
        address_mode: AddressMode::D16,
        condition_type: Some(ConditionType::C),
    });
    instructions[0xDE] = Instruction::Sbc(SbcInstruction {
        address_mode: AddressMode::R_D8(RegisterType::A),
    });
    instructions[0xDF] = Instruction::Rst(RstInstruction { address: 0x18 });

    // 0xEX
    instructions[0xE0] = Instruction::Ldh(LdhInstruction {
        address_mode: AddressMode::A8_R(RegisterType::A),
    });
    instructions[0xE1] = Instruction::Pop(PopInstruction {
        address_mode: AddressMode::R(RegisterType::HL),
    });
    instructions[0xE2] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::MR_R(RegisterType::C, RegisterType::A),
    });
    instructions[0xE5] = Instruction::Push(PushInstruction {
        address_mode: AddressMode::R(RegisterType::HL),
    });
    instructions[0xE6] = Instruction::And(AndInstruction {
        address_mode: AddressMode::R_D8(RegisterType::A),
    });
    instructions[0xE7] = Instruction::Rst(RstInstruction { address: 0x20 });
    instructions[0xE8] = Instruction::Add(AddInstruction {
        address_mode: AddressMode::HL_SPe8,
    });
    instructions[0xEA] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::A16_R(RegisterType::A),
    });
    instructions[0xE9] = Instruction::Jp(JpInstruction {
        address_mode: AddressMode::R(RegisterType::HL),
        condition_type: None,
    });
    instructions[0xEE] = Instruction::Xor(XorInstruction {
        address_mode: AddressMode::R_D8(RegisterType::A),
    });
    instructions[0xEF] = Instruction::Rst(RstInstruction { address: 0x28 });

    // 0xFX
    instructions[0xF0] = Instruction::Ldh(LdhInstruction {
        address_mode: AddressMode::R_A8(RegisterType::A),
    });
    instructions[0xF1] = Instruction::Pop(PopInstruction {
        address_mode: AddressMode::R(RegisterType::AF),
    });
    instructions[0xF2] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::C),
    });
    instructions[0xF3] = Instruction::Di(DiInstruction {});
    instructions[0xF5] = Instruction::Push(PushInstruction {
        address_mode: AddressMode::R(RegisterType::AF),
    });
    instructions[0xF6] = Instruction::Or(OrInstruction {
        address_mode: AddressMode::R_D8(RegisterType::A),
    });
    instructions[0xF7] = Instruction::Rst(RstInstruction { address: 0x30 });
    instructions[0xF8] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::HL_SPe8,
    });
    instructions[0xF9] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_R(RegisterType::SP, RegisterType::HL),
    });
    instructions[0xFA] = Instruction::Ld(LdInstruction {
        address_mode: AddressMode::R_A16(RegisterType::A),
    });
    instructions[0xFB] = Instruction::Ei(EiInstruction);
    instructions[0xFE] = Instruction::Cp(CpInstruction {
        address_mode: AddressMode::R_D8(RegisterType::A),
    });
    instructions[0xFF] = Instruction::Rst(RstInstruction { address: 0x38 });

    instructions
};
