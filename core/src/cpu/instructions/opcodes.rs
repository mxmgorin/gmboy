use crate::cpu::instructions::address_mode::AddressMode;
use crate::cpu::instructions::arithmetic::adc::AdcInstruction;
use crate::cpu::instructions::arithmetic::add::AddInstruction;
use crate::cpu::instructions::arithmetic::cp::CpInstruction;
use crate::cpu::instructions::arithmetic::sbc::SbcInstruction;
use crate::cpu::instructions::arithmetic::sub::SubInstruction;
use crate::cpu::instructions::bitwise::and::AndInstruction;
use crate::cpu::instructions::condition_type::ConditionType;
use crate::cpu::instructions::jump::ret::RetInstruction;
use crate::cpu::instructions::jump::rst::RstInstruction;
use crate::cpu::instructions::load::pop::PopInstruction;
use crate::cpu::instructions::load::push::PushInstruction;
use crate::cpu::instructions::misc::prefix::PrefixInstruction;
use crate::cpu::instructions::misc::scf::ScfInstruction;
use crate::cpu::instructions::misc::stop::StopInstruction;
use crate::cpu::instructions::rotate::rla::RlaInstruction;
use crate::cpu::instructions::rotate::rlca::RlcaInstruction;
use crate::cpu::instructions::rotate::rra::RraInstruction;
use crate::cpu::instructions::rotate::rrca::RrcaInstruction;
use crate::cpu::instructions::*;
use crate::cpu::instructions::{Instruction, RegisterType};
use crate::cpu::Cpu;

const INSTRUCTIONS_LEN: usize = 0xFF + 1;

pub const INSTRUCTIONS_BY_OPCODES: [InstructionWrapper; INSTRUCTIONS_LEN] = {
    let mut instructions = {
        let mut array = [InstructionWrapper::unknown(0); INSTRUCTIONS_LEN];
        let mut i = 0;
        while i < INSTRUCTIONS_LEN {
            array[i] = InstructionWrapper::unknown(i as u8);
            i += 1;
        }
        array
    };

    // 0x0X
    instructions[0x00] = InstructionWrapper::new(Instruction::Nop(NopInstruction), Cpu::fetch_impl);
    instructions[0x01] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_D16(RegisterType::BC),
        }),
        |cpu| cpu.fetch_r_d16(RegisterType::BC),
    );
    instructions[0x02] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::MR_R(RegisterType::BC, RegisterType::A),
        }),
        |cpu| cpu.fetch_mr_r(RegisterType::BC, RegisterType::A),
    );
    instructions[0x03] = InstructionWrapper::new(
        Instruction::Inc(IncInstruction {
            address_mode: AddressMode::R(RegisterType::BC),
        }),
        |cpu| cpu.fetch_r(RegisterType::BC),
    );
    instructions[0x04] = InstructionWrapper::new(
        Instruction::Inc(IncInstruction {
            address_mode: AddressMode::R(RegisterType::B),
        }),
        |cpu| cpu.fetch_r(RegisterType::B),
    );
    instructions[0x05] = InstructionWrapper::new(
        Instruction::Dec(DecInstruction {
            address_mode: AddressMode::R(RegisterType::B),
        }),
        |cpu| cpu.fetch_r(RegisterType::B),
    );
    instructions[0x06] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_D8(RegisterType::B),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::B),
    );
    instructions[0x07] =
        InstructionWrapper::new(Instruction::Rlca(RlcaInstruction), Cpu::fetch_impl);
    instructions[0x08] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::A16_R(RegisterType::SP),
        }),
        |cpu| cpu.fetch_a16_r(RegisterType::SP),
    );
    instructions[0x09] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_R(RegisterType::HL, RegisterType::BC),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::HL, RegisterType::BC),
    );
    instructions[0x0A] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::BC),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::BC),
    );
    instructions[0x0B] = InstructionWrapper::new(
        Instruction::Dec(DecInstruction {
            address_mode: AddressMode::R(RegisterType::BC),
        }),
        |cpu| cpu.fetch_r(RegisterType::BC),
    );
    instructions[0x0C] = InstructionWrapper::new(
        Instruction::Inc(IncInstruction {
            address_mode: AddressMode::R(RegisterType::C),
        }),
        |cpu| cpu.fetch_r(RegisterType::C),
    );
    instructions[0x0D] = InstructionWrapper::new(
        Instruction::Dec(DecInstruction {
            address_mode: AddressMode::R(RegisterType::C),
        }),
        |cpu| cpu.fetch_r(RegisterType::C),
    );
    instructions[0x0E] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_D8(RegisterType::C),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::C),
    );
    instructions[0x0F] =
        InstructionWrapper::new(Instruction::Rrca(RrcaInstruction), Cpu::fetch_impl);

    // 0x1X
    instructions[0x10] =
        InstructionWrapper::new(Instruction::Stop(StopInstruction), Cpu::fetch_impl);
    instructions[0x11] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_D16(RegisterType::DE),
        }),
        |cpu| cpu.fetch_r_d16(RegisterType::DE),
    );
    instructions[0x12] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::MR_R(RegisterType::DE, RegisterType::A),
        }),
        |cpu| cpu.fetch_mr_r(RegisterType::DE, RegisterType::A),
    );
    instructions[0x13] = InstructionWrapper::new(
        Instruction::Inc(IncInstruction {
            address_mode: AddressMode::R(RegisterType::DE),
        }),
        |cpu| cpu.fetch_r(RegisterType::DE),
    );
    instructions[0x14] = InstructionWrapper::new(
        Instruction::Inc(IncInstruction {
            address_mode: AddressMode::R(RegisterType::D),
        }),
        |cpu| cpu.fetch_r(RegisterType::D),
    );
    instructions[0x15] = InstructionWrapper::new(
        Instruction::Dec(DecInstruction {
            address_mode: AddressMode::R(RegisterType::D),
        }),
        |cpu| cpu.fetch_r(RegisterType::D),
    );
    instructions[0x16] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_D8(RegisterType::D),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::D),
    );
    instructions[0x17] = InstructionWrapper::new(Instruction::Rla(RlaInstruction), Cpu::fetch_impl);
    instructions[0x18] = InstructionWrapper::new(
        Instruction::Jr(JrInstruction {
            condition_type: None,
        }),
        Cpu::fetch_d8,
    );
    instructions[0x19] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_R(RegisterType::HL, RegisterType::DE),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::HL, RegisterType::DE),
    );
    instructions[0x1A] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::DE),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::DE),
    );
    instructions[0x1B] = InstructionWrapper::new(
        Instruction::Dec(DecInstruction {
            address_mode: AddressMode::R(RegisterType::DE),
        }),
        |cpu| cpu.fetch_r(RegisterType::DE),
    );
    instructions[0x1C] = InstructionWrapper::new(
        Instruction::Inc(IncInstruction {
            address_mode: AddressMode::R(RegisterType::E),
        }),
        |cpu| cpu.fetch_r(RegisterType::E),
    );
    instructions[0x1D] = InstructionWrapper::new(
        Instruction::Dec(DecInstruction {
            address_mode: AddressMode::R(RegisterType::E),
        }),
        |cpu| cpu.fetch_r(RegisterType::E),
    );
    instructions[0x1E] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_D8(RegisterType::E),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::E),
    );
    instructions[0x1F] = InstructionWrapper::new(Instruction::Rra(RraInstruction), Cpu::fetch_impl);

    // 0x2X
    instructions[0x20] = InstructionWrapper::new(
        Instruction::Jr(JrInstruction {
            condition_type: Some(ConditionType::NZ),
        }),
        Cpu::fetch_d8,
    );
    instructions[0x21] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_D16(RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_d16(RegisterType::HL),
    );
    instructions[0x22] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::HLI_R(RegisterType::A),
        }),
        |cpu| cpu.fetch_hli_r(RegisterType::A),
    );
    instructions[0x23] = InstructionWrapper::new(
        Instruction::Inc(IncInstruction {
            address_mode: AddressMode::R(RegisterType::HL),
        }),
        |cpu| cpu.fetch_r(RegisterType::HL),
    );
    instructions[0x24] = InstructionWrapper::new(
        Instruction::Inc(IncInstruction {
            address_mode: AddressMode::R(RegisterType::H),
        }),
        |cpu| cpu.fetch_r(RegisterType::H),
    );
    instructions[0x25] = InstructionWrapper::new(
        Instruction::Dec(DecInstruction {
            address_mode: AddressMode::R(RegisterType::H),
        }),
        |cpu| cpu.fetch_r(RegisterType::H),
    );
    instructions[0x26] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_D8(RegisterType::H),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::H),
    );
    instructions[0x27] = InstructionWrapper::new(Instruction::Daa(DaaInstruction), Cpu::fetch_impl);
    instructions[0x28] = InstructionWrapper::new(
        Instruction::Jr(JrInstruction {
            condition_type: Some(ConditionType::Z),
        }),
        Cpu::fetch_d8,
    );
    instructions[0x29] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_R(RegisterType::HL, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::HL, RegisterType::HL),
    );
    instructions[0x2A] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_HLI(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_hli(RegisterType::A),
    );
    instructions[0x2B] = InstructionWrapper::new(
        Instruction::Dec(DecInstruction {
            address_mode: AddressMode::R(RegisterType::HL),
        }),
        |cpu| cpu.fetch_r(RegisterType::HL),
    );
    instructions[0x2C] = InstructionWrapper::new(
        Instruction::Inc(IncInstruction {
            address_mode: AddressMode::R(RegisterType::L),
        }),
        |cpu| cpu.fetch_r(RegisterType::L),
    );
    instructions[0x2D] = InstructionWrapper::new(
        Instruction::Dec(DecInstruction {
            address_mode: AddressMode::R(RegisterType::L),
        }),
        |cpu| cpu.fetch_r(RegisterType::L),
    );
    instructions[0x2E] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_D8(RegisterType::L),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::L),
    );
    instructions[0x2F] = InstructionWrapper::new(Instruction::Cpl(CplInstruction), Cpu::fetch_impl);

    // 0x3X
    instructions[0x30] = InstructionWrapper::new(
        Instruction::Jr(JrInstruction {
            condition_type: Some(ConditionType::NC),
        }),
        Cpu::fetch_d8,
    );
    instructions[0x31] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_D16(RegisterType::SP),
        }),
        |cpu| cpu.fetch_r_d16(RegisterType::SP),
    );
    instructions[0x32] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::HLD_R(RegisterType::A),
        }),
        |cpu| cpu.fetch_hld_r(RegisterType::A),
    );
    instructions[0x33] = InstructionWrapper::new(
        Instruction::Inc(IncInstruction {
            address_mode: AddressMode::R(RegisterType::SP),
        }),
        |cpu| cpu.fetch_r(RegisterType::SP),
    );
    instructions[0x34] = InstructionWrapper::new(
        Instruction::Inc(IncInstruction {
            address_mode: AddressMode::MR(RegisterType::HL),
        }),
        |cpu| cpu.fetch_mr(RegisterType::HL),
    );
    instructions[0x35] = InstructionWrapper::new(
        Instruction::Dec(DecInstruction {
            address_mode: AddressMode::MR(RegisterType::HL),
        }),
        |cpu| cpu.fetch_mr(RegisterType::HL),
    );
    instructions[0x36] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::MR_D8(RegisterType::HL),
        }),
        |cpu| cpu.fetch_mr_d8(RegisterType::HL),
    );
    instructions[0x37] = InstructionWrapper::new(Instruction::Scf(ScfInstruction), Cpu::fetch_impl);
    instructions[0x38] = InstructionWrapper::new(
        Instruction::Jr(JrInstruction {
            condition_type: Some(ConditionType::C),
        }),
        Cpu::fetch_d8,
    );
    instructions[0x39] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_R(RegisterType::HL, RegisterType::SP),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::HL, RegisterType::SP),
    );
    instructions[0x3A] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_HLD(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_hld(RegisterType::A),
    );
    instructions[0x3B] = InstructionWrapper::new(
        Instruction::Dec(DecInstruction {
            address_mode: AddressMode::R(RegisterType::SP),
        }),
        |cpu| cpu.fetch_r(RegisterType::SP),
    );
    instructions[0x3C] = InstructionWrapper::new(
        Instruction::Inc(IncInstruction {
            address_mode: AddressMode::R(RegisterType::A),
        }),
        |cpu| cpu.fetch_r(RegisterType::A),
    );
    instructions[0x3D] = InstructionWrapper::new(
        Instruction::Dec(DecInstruction {
            address_mode: AddressMode::R(RegisterType::A),
        }),
        |cpu| cpu.fetch_r(RegisterType::A),
    );
    instructions[0x3E] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_D8(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0x3F] = InstructionWrapper::new(Instruction::Ccf(CcfInstruction), Cpu::fetch_impl);

    // 0x4X
    instructions[0x40] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::B, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::B),
    );
    instructions[0x41] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::B, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::C),
    );
    instructions[0x42] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::B, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::D),
    );
    instructions[0x43] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::B, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::E),
    );
    instructions[0x44] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::B, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::H),
    );
    instructions[0x45] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::B, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::L),
    );
    instructions[0x46] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_MR(RegisterType::B, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::B, RegisterType::HL),
    );
    instructions[0x47] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::B, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::A),
    );
    instructions[0x48] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::C, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::B),
    );
    instructions[0x49] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::C, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::C),
    );
    instructions[0x4A] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::C, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::D),
    );
    instructions[0x4B] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::C, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::E),
    );
    instructions[0x4C] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::C, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::H),
    );
    instructions[0x4D] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::C, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::L),
    );
    instructions[0x4E] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_MR(RegisterType::C, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::C, RegisterType::HL),
    );
    instructions[0x4F] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::C, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::A),
    );

    // 0x5X
    instructions[0x50] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::D, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::B),
    );
    instructions[0x51] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::D, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::C),
    );
    instructions[0x52] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::D, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::D),
    );
    instructions[0x53] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::D, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::E),
    );
    instructions[0x54] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::D, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::H),
    );
    instructions[0x55] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::D, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::L),
    );
    instructions[0x56] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_MR(RegisterType::D, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::D, RegisterType::HL),
    );
    instructions[0x57] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::D, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::A),
    );
    instructions[0x58] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::E, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::B),
    );
    instructions[0x59] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::E, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::C),
    );
    instructions[0x5A] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::E, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::D),
    );
    instructions[0x5B] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::E, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::E),
    );
    instructions[0x5C] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::E, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::H),
    );
    instructions[0x5D] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::E, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::L),
    );
    instructions[0x5E] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_MR(RegisterType::E, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::E, RegisterType::HL),
    );
    instructions[0x5F] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::E, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::A),
    );

    // 0x6X
    instructions[0x60] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::H, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::B),
    );
    instructions[0x61] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::H, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::C),
    );
    instructions[0x62] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::H, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::D),
    );
    instructions[0x63] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::H, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::E),
    );
    instructions[0x64] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::H, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::H),
    );
    instructions[0x65] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::H, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::L),
    );
    instructions[0x66] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_MR(RegisterType::H, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::H, RegisterType::HL),
    );
    instructions[0x67] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::H, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::A),
    );
    instructions[0x68] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::L, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::B),
    );
    instructions[0x69] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::L, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::C),
    );
    instructions[0x6A] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::L, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::D),
    );
    instructions[0x6B] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::L, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::E),
    );
    instructions[0x6C] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::L, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::H),
    );
    instructions[0x6D] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::L, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::L),
    );
    instructions[0x6E] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_MR(RegisterType::L, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::L, RegisterType::HL),
    );
    instructions[0x6F] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::L, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::A),
    );

    // 0x7X
    instructions[0x76] =
        InstructionWrapper::new(Instruction::Halt(HaltInstruction), Cpu::fetch_impl);
    instructions[0x70] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::B),
        }),
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::B),
    );
    instructions[0x71] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::C),
        }),
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::C),
    );
    instructions[0x72] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::D),
        }),
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::D),
    );
    instructions[0x73] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::E),
        }),
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::E),
    );
    instructions[0x74] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::H),
        }),
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::H),
    );
    instructions[0x75] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::L),
        }),
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::L),
    );
    instructions[0x77] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::MR_R(RegisterType::HL, RegisterType::A),
        }),
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::A),
    );
    instructions[0x78] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0x79] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0x7A] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0x7B] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0x7C] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0x7D] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0x7E] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0x7F] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );

    // 0x8X
    instructions[0x80] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0x81] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0x82] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0x83] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0x84] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0x85] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0x86] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0x87] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );
    instructions[0x88] = InstructionWrapper::new(
        Instruction::Adc(AdcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0x89] = InstructionWrapper::new(
        Instruction::Adc(AdcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0x8A] = InstructionWrapper::new(
        Instruction::Adc(AdcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0x8B] = InstructionWrapper::new(
        Instruction::Adc(AdcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0x8C] = InstructionWrapper::new(
        Instruction::Adc(AdcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0x8D] = InstructionWrapper::new(
        Instruction::Adc(AdcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0x8E] = InstructionWrapper::new(
        Instruction::Adc(AdcInstruction {
            address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0x8F] = InstructionWrapper::new(
        Instruction::Adc(AdcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );

    // 0x9X
    instructions[0x90] = InstructionWrapper::new(
        Instruction::Sub(SubInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0x91] = InstructionWrapper::new(
        Instruction::Sub(SubInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0x92] = InstructionWrapper::new(
        Instruction::Sub(SubInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0x93] = InstructionWrapper::new(
        Instruction::Sub(SubInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0x94] = InstructionWrapper::new(
        Instruction::Sub(SubInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0x95] = InstructionWrapper::new(
        Instruction::Sub(SubInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0x96] = InstructionWrapper::new(
        Instruction::Sub(SubInstruction {
            address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0x97] = InstructionWrapper::new(
        Instruction::Sub(SubInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );
    instructions[0x98] = InstructionWrapper::new(
        Instruction::Sbc(SbcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0x99] = InstructionWrapper::new(
        Instruction::Sbc(SbcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0x9A] = InstructionWrapper::new(
        Instruction::Sbc(SbcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0x9B] = InstructionWrapper::new(
        Instruction::Sbc(SbcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0x9C] = InstructionWrapper::new(
        Instruction::Sbc(SbcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0x9D] = InstructionWrapper::new(
        Instruction::Sbc(SbcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0x9E] = InstructionWrapper::new(
        Instruction::Sbc(SbcInstruction {
            address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0x9F] = InstructionWrapper::new(
        Instruction::Sbc(SbcInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );

    // 0xAX
    instructions[0xA0] = InstructionWrapper::new(
        Instruction::And(AndInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0xA1] = InstructionWrapper::new(
        Instruction::And(AndInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0xA2] = InstructionWrapper::new(
        Instruction::And(AndInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0xA3] = InstructionWrapper::new(
        Instruction::And(AndInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0xA4] = InstructionWrapper::new(
        Instruction::And(AndInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0xA5] = InstructionWrapper::new(
        Instruction::And(AndInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0xA6] = InstructionWrapper::new(
        Instruction::And(AndInstruction {
            address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0xA7] = InstructionWrapper::new(
        Instruction::And(AndInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );
    instructions[0xA8] = InstructionWrapper::new(
        Instruction::Xor(XorInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0xA9] = InstructionWrapper::new(
        Instruction::Xor(XorInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0xAA] = InstructionWrapper::new(
        Instruction::Xor(XorInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0xAB] = InstructionWrapper::new(
        Instruction::Xor(XorInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0xAC] = InstructionWrapper::new(
        Instruction::Xor(XorInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0xAD] = InstructionWrapper::new(
        Instruction::Xor(XorInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0xAE] = InstructionWrapper::new(
        Instruction::Xor(XorInstruction {
            address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0xAF] = InstructionWrapper::new(
        Instruction::Xor(XorInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );

    // 0xBX
    instructions[0xB0] = InstructionWrapper::new(
        Instruction::Or(OrInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0xB1] = InstructionWrapper::new(
        Instruction::Or(OrInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0xB2] = InstructionWrapper::new(
        Instruction::Or(OrInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0xB3] = InstructionWrapper::new(
        Instruction::Or(OrInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0xB4] = InstructionWrapper::new(
        Instruction::Or(OrInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0xB5] = InstructionWrapper::new(
        Instruction::Or(OrInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0xB6] = InstructionWrapper::new(
        Instruction::Or(OrInstruction {
            address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0xB7] = InstructionWrapper::new(
        Instruction::Or(OrInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );
    instructions[0xB8] = InstructionWrapper::new(
        Instruction::Cp(CpInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::B),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0xB9] = InstructionWrapper::new(
        Instruction::Cp(CpInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0xBA] = InstructionWrapper::new(
        Instruction::Cp(CpInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::D),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0xBB] = InstructionWrapper::new(
        Instruction::Cp(CpInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::E),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0xBC] = InstructionWrapper::new(
        Instruction::Cp(CpInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::H),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0xBD] = InstructionWrapper::new(
        Instruction::Cp(CpInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::L),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0xBE] = InstructionWrapper::new(
        Instruction::Cp(CpInstruction {
            address_mode: AddressMode::R_MR(RegisterType::A, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0xBF] = InstructionWrapper::new(
        Instruction::Cp(CpInstruction {
            address_mode: AddressMode::R_R(RegisterType::A, RegisterType::A),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );

    // 0xCX
    instructions[0xC0] = InstructionWrapper::new(
        Instruction::Ret(RetInstruction {
            condition_type: Some(ConditionType::NZ),
        }),
        Cpu::fetch_impl,
    );
    instructions[0xC1] = InstructionWrapper::new(
        Instruction::Pop(PopInstruction {
            address_mode: AddressMode::R(RegisterType::BC),
        }),
        |cpu| cpu.fetch_r(RegisterType::BC),
    );
    instructions[0xC2] = InstructionWrapper::new(
        Instruction::Jp(JpInstruction {
            address_mode: AddressMode::D16,
            condition_type: Some(ConditionType::NZ),
        }),
        Cpu::fetch_d16,
    );
    instructions[0xC3] = InstructionWrapper::new(
        Instruction::Jp(JpInstruction {
            address_mode: AddressMode::D16,
            condition_type: None,
        }),
        Cpu::fetch_d16,
    );
    instructions[0xC5] = InstructionWrapper::new(
        Instruction::Push(PushInstruction {
            address_mode: AddressMode::R(RegisterType::BC),
        }),
        |cpu| cpu.fetch_r(RegisterType::BC),
    );
    instructions[0xC6] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_D8(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xC7] = InstructionWrapper::new(
        Instruction::Rst(RstInstruction { address: 0x00 }),
        Cpu::fetch_impl,
    );
    instructions[0xC4] = InstructionWrapper::new(
        Instruction::Call(CallInstruction {
            condition_type: Some(ConditionType::NZ),
        }),
        Cpu::fetch_d16,
    );
    instructions[0xC8] = InstructionWrapper::new(
        Instruction::Ret(RetInstruction {
            condition_type: Some(ConditionType::Z),
        }),
        Cpu::fetch_impl,
    );
    instructions[0xC9] = InstructionWrapper::new(
        Instruction::Ret(RetInstruction {
            condition_type: None,
        }),
        Cpu::fetch_impl,
    );
    instructions[0xCC] = InstructionWrapper::new(
        Instruction::Call(CallInstruction {
            condition_type: Some(ConditionType::Z),
        }),
        Cpu::fetch_d16,
    );
    instructions[0xCD] = InstructionWrapper::new(
        Instruction::Call(CallInstruction {
            condition_type: None,
        }),
        Cpu::fetch_d16,
    );
    instructions[0xCA] = InstructionWrapper::new(
        Instruction::Jp(JpInstruction {
            address_mode: AddressMode::D16,
            condition_type: Some(ConditionType::Z),
        }),
        Cpu::fetch_d16,
    );
    instructions[0xCB] =
        InstructionWrapper::new(Instruction::Prefix(PrefixInstruction), Cpu::fetch_d8);
    instructions[0xCE] = InstructionWrapper::new(
        Instruction::Adc(AdcInstruction {
            address_mode: AddressMode::R_D8(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xCF] = InstructionWrapper::new(
        Instruction::Rst(RstInstruction { address: 0x08 }),
        Cpu::fetch_impl,
    );

    // 0xDX
    instructions[0xD0] = InstructionWrapper::new(
        Instruction::Ret(RetInstruction {
            condition_type: Some(ConditionType::NC),
        }),
        Cpu::fetch_impl,
    );
    instructions[0xD1] = InstructionWrapper::new(
        Instruction::Pop(PopInstruction {
            address_mode: AddressMode::R(RegisterType::DE),
        }),
        |cpu| cpu.fetch_r(RegisterType::DE),
    );
    instructions[0xD2] = InstructionWrapper::new(
        Instruction::Jp(JpInstruction {
            address_mode: AddressMode::D16,
            condition_type: Some(ConditionType::NC),
        }),
        Cpu::fetch_d16,
    );
    instructions[0xD4] = InstructionWrapper::new(
        Instruction::Call(CallInstruction {
            condition_type: Some(ConditionType::NC),
        }),
        Cpu::fetch_d16,
    );
    instructions[0xD5] = InstructionWrapper::new(
        Instruction::Push(PushInstruction {
            address_mode: AddressMode::R(RegisterType::DE),
        }),
        |cpu| cpu.fetch_r(RegisterType::DE),
    );
    instructions[0xD6] = InstructionWrapper::new(
        Instruction::Sub(SubInstruction {
            address_mode: AddressMode::R_D8(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xD7] = InstructionWrapper::new(
        Instruction::Rst(RstInstruction { address: 0x10 }),
        Cpu::fetch_impl,
    );
    instructions[0xD8] = InstructionWrapper::new(
        Instruction::Ret(RetInstruction {
            condition_type: Some(ConditionType::C),
        }),
        Cpu::fetch_impl,
    );
    instructions[0xD9] =
        InstructionWrapper::new(Instruction::Reti(RetiInstruction::new()), Cpu::fetch_impl);
    instructions[0xDC] = InstructionWrapper::new(
        Instruction::Call(CallInstruction {
            condition_type: Some(ConditionType::C),
        }),
        Cpu::fetch_d16,
    );
    instructions[0xDA] = InstructionWrapper::new(
        Instruction::Jp(JpInstruction {
            address_mode: AddressMode::D16,
            condition_type: Some(ConditionType::C),
        }),
        Cpu::fetch_d16,
    );
    instructions[0xDE] = InstructionWrapper::new(
        Instruction::Sbc(SbcInstruction {
            address_mode: AddressMode::R_D8(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xDF] = InstructionWrapper::new(
        Instruction::Rst(RstInstruction { address: 0x18 }),
        Cpu::fetch_impl,
    );

    // 0xEX
    instructions[0xE0] = InstructionWrapper::new(
        Instruction::Ldh(LdhInstruction {
            address_mode: AddressMode::A8_R(RegisterType::A),
        }),
        |cpu| cpu.fetch_a8_r(RegisterType::A),
    );
    instructions[0xE1] = InstructionWrapper::new(
        Instruction::Pop(PopInstruction {
            address_mode: AddressMode::R(RegisterType::HL),
        }),
        |cpu| cpu.fetch_r(RegisterType::HL),
    );
    instructions[0xE2] = InstructionWrapper::new(
        Instruction::Ldh(LdhInstruction {
            address_mode: AddressMode::MR_R(RegisterType::C, RegisterType::A),
        }),
        |cpu| cpu.fetch_mr_r(RegisterType::C, RegisterType::A),
    );
    instructions[0xE5] = InstructionWrapper::new(
        Instruction::Push(PushInstruction {
            address_mode: AddressMode::R(RegisterType::HL),
        }),
        |cpu| cpu.fetch_r(RegisterType::HL),
    );
    instructions[0xE6] = InstructionWrapper::new(
        Instruction::And(AndInstruction {
            address_mode: AddressMode::R_D8(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xE7] = InstructionWrapper::new(
        Instruction::Rst(RstInstruction { address: 0x20 }),
        Cpu::fetch_impl,
    );
    instructions[0xE8] = InstructionWrapper::new(
        Instruction::Add(AddInstruction {
            address_mode: AddressMode::R_D8(RegisterType::SP),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::SP),
    );
    instructions[0xEA] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::A16_R(RegisterType::A),
        }),
        |cpu| cpu.fetch_a16_r(RegisterType::A),
    );
    instructions[0xE9] = InstructionWrapper::new(
        Instruction::Jp(JpInstruction {
            address_mode: AddressMode::R(RegisterType::HL),
            condition_type: None,
        }),
        |cpu| cpu.fetch_r(RegisterType::HL),
    );
    instructions[0xEE] = InstructionWrapper::new(
        Instruction::Xor(XorInstruction {
            address_mode: AddressMode::R_D8(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xEF] = InstructionWrapper::new(
        Instruction::Rst(RstInstruction { address: 0x28 }),
        Cpu::fetch_impl,
    );

    // 0xFX
    instructions[0xF0] = InstructionWrapper::new(
        Instruction::Ldh(LdhInstruction {
            address_mode: AddressMode::R_HA8(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_ha8(RegisterType::A),
    );
    instructions[0xF1] = InstructionWrapper::new(
        Instruction::Pop(PopInstruction {
            address_mode: AddressMode::R(RegisterType::AF),
        }),
        |cpu| cpu.fetch_r(RegisterType::AF),
    );
    instructions[0xF2] = InstructionWrapper::new(
        Instruction::Ldh(LdhInstruction {
            address_mode: AddressMode::R_HMR(RegisterType::A, RegisterType::C),
        }),
        |cpu| cpu.fetch_r_hmr(RegisterType::A, RegisterType::C),
    );
    instructions[0xF3] =
        InstructionWrapper::new(Instruction::Di(DiInstruction {}), Cpu::fetch_impl);
    instructions[0xF5] = InstructionWrapper::new(
        Instruction::Push(PushInstruction {
            address_mode: AddressMode::R(RegisterType::AF),
        }),
        |cpu| cpu.fetch_r(RegisterType::AF),
    );
    instructions[0xF6] = InstructionWrapper::new(
        Instruction::Or(OrInstruction {
            address_mode: AddressMode::R_D8(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xF7] = InstructionWrapper::new(
        Instruction::Rst(RstInstruction { address: 0x30 }),
        Cpu::fetch_impl,
    );
    instructions[0xF8] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::LH_SPi8,
        }),
        Cpu::fetch_lh_spi8,
    );
    instructions[0xF9] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_R(RegisterType::SP, RegisterType::HL),
        }),
        |cpu| cpu.fetch_r_r(RegisterType::SP, RegisterType::HL),
    );
    instructions[0xFA] = InstructionWrapper::new(
        Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_A16(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_a16(RegisterType::A),
    );
    instructions[0xFB] = InstructionWrapper::new(Instruction::Ei(EiInstruction), Cpu::fetch_impl);
    instructions[0xFE] = InstructionWrapper::new(
        Instruction::Cp(CpInstruction {
            address_mode: AddressMode::R_D8(RegisterType::A),
        }),
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xFF] = InstructionWrapper::new(
        Instruction::Rst(RstInstruction { address: 0x38 }),
        Cpu::fetch_impl,
    );

    instructions
};
