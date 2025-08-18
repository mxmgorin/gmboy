use crate::cpu::instructions::address_mode::AddressMode;
use crate::cpu::instructions::condition_type::ConditionType;
use crate::cpu::instructions::*;
use crate::cpu::instructions::{RegisterType};
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
    instructions[0x00] = InstructionWrapper::new(
        InstructionType::Nop,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_nop,
        Cpu::fetch_impl,
    );
    instructions[0x01] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_D16(RegisterType::BC)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_d16(RegisterType::BC),
    );
    instructions[0x02] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::MR_R(RegisterType::BC, RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_mr_r(RegisterType::BC, RegisterType::A),
    );
    instructions[0x03] = InstructionWrapper::new(
        InstructionType::Inc,
        InstructionArgs::default(AddressMode::R(RegisterType::BC)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_r(RegisterType::BC),
    );
    instructions[0x04] = InstructionWrapper::new(
        InstructionType::Inc,
        InstructionArgs::default(AddressMode::R(RegisterType::B)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_r(RegisterType::B),
    );
    instructions[0x05] = InstructionWrapper::new(
        InstructionType::Dec,
        InstructionArgs::default(AddressMode::R(RegisterType::B)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_r(RegisterType::B),
    );
    instructions[0x06] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::B)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_d8(RegisterType::B),
    );
    instructions[0x07] = InstructionWrapper::new(
        InstructionType::Rlca,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_rlca,
        Cpu::fetch_impl,
    );
    instructions[0x08] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::A16_R(RegisterType::SP)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_a16_r(RegisterType::SP),
    );
    instructions[0x09] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_R(RegisterType::HL, RegisterType::BC)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_r(RegisterType::HL, RegisterType::BC),
    );
    instructions[0x0A] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::A, RegisterType::BC)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::BC),
    );
    instructions[0x0B] = InstructionWrapper::new(
        InstructionType::Dec,
        InstructionArgs::default(AddressMode::R(RegisterType::BC)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_r(RegisterType::BC),
    );
    instructions[0x0C] = InstructionWrapper::new(
        InstructionType::Inc,
        InstructionArgs::default(AddressMode::R(RegisterType::C)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_r(RegisterType::C),
    );
    instructions[0x0D] = InstructionWrapper::new(
        InstructionType::Dec,
        InstructionArgs::default(AddressMode::R(RegisterType::C)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_r(RegisterType::C),
    );
    instructions[0x0E] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::C)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_d8(RegisterType::C),
    );
    instructions[0x0F] = InstructionWrapper::new(
        InstructionType::Rrca,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_rrca,
        Cpu::fetch_impl,
    );

    // 0x1X
    instructions[0x10] = InstructionWrapper::new(
        InstructionType::Stop,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_stop,
        Cpu::fetch_impl,
    );
    instructions[0x11] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_D16(RegisterType::DE)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_d16(RegisterType::DE),
    );
    instructions[0x12] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::MR_R(RegisterType::DE, RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_mr_r(RegisterType::DE, RegisterType::A),
    );
    instructions[0x13] = InstructionWrapper::new(
        InstructionType::Inc,
        InstructionArgs::default(AddressMode::R(RegisterType::DE)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_r(RegisterType::DE),
    );
    instructions[0x14] = InstructionWrapper::new(
        InstructionType::Inc,
        InstructionArgs::default(AddressMode::R(RegisterType::D)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_r(RegisterType::D),
    );
    instructions[0x15] = InstructionWrapper::new(
        InstructionType::Dec,
        InstructionArgs::default(AddressMode::R(RegisterType::D)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_r(RegisterType::D),
    );
    instructions[0x16] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::D)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_d8(RegisterType::D),
    );
    instructions[0x17] = InstructionWrapper::new(
        InstructionType::RLA,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_rla,
        Cpu::fetch_impl,
    );
    instructions[0x18] = InstructionWrapper::new(
        InstructionType::Jr,
        InstructionArgs::default(AddressMode::D8),
        Cpu::execute_jr,
        Cpu::fetch_d8,
    );
    instructions[0x19] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_R(RegisterType::HL, RegisterType::DE)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_r(RegisterType::HL, RegisterType::DE),
    );
    instructions[0x1A] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::A, RegisterType::DE)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::DE),
    );
    instructions[0x1B] = InstructionWrapper::new(
        InstructionType::Dec,
        InstructionArgs::default(AddressMode::R(RegisterType::DE)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_r(RegisterType::DE),
    );
    instructions[0x1C] = InstructionWrapper::new(
        InstructionType::Inc,
        InstructionArgs::default(AddressMode::R(RegisterType::E)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_r(RegisterType::E),
    );
    instructions[0x1D] = InstructionWrapper::new(
        InstructionType::Dec,
        InstructionArgs::default(AddressMode::R(RegisterType::E)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_r(RegisterType::E),
    );
    instructions[0x1E] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::E)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_d8(RegisterType::E),
    );
    instructions[0x1F] = InstructionWrapper::new(
        InstructionType::Rra,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_rra,
        Cpu::fetch_impl,
    );

    // 0x2X
    instructions[0x20] = InstructionWrapper::new(
        InstructionType::Jr,
        InstructionArgs::new(Some(ConditionType::NZ), 0, AddressMode::D8),
        Cpu::execute_jr,
        Cpu::fetch_d8,
    );
    instructions[0x21] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_D16(RegisterType::HL)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_d16(RegisterType::HL),
    );
    instructions[0x22] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::HLI_R(RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_hli_r(RegisterType::A),
    );
    instructions[0x23] = InstructionWrapper::new(
        InstructionType::Inc,
        InstructionArgs::default(AddressMode::R(RegisterType::HL)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_r(RegisterType::HL),
    );
    instructions[0x24] = InstructionWrapper::new(
        InstructionType::Inc,
        InstructionArgs::default(AddressMode::R(RegisterType::H)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_r(RegisterType::H),
    );
    instructions[0x25] = InstructionWrapper::new(
        InstructionType::Dec,
        InstructionArgs::default(AddressMode::R(RegisterType::H)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_r(RegisterType::H),
    );
    instructions[0x26] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::H)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_d8(RegisterType::H),
    );
    instructions[0x27] = InstructionWrapper::new(
        InstructionType::Daa,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_daa,
        Cpu::fetch_impl,
    );
    instructions[0x28] = InstructionWrapper::new(
        InstructionType::Jr,
        InstructionArgs::new(Some(ConditionType::Z), 0, AddressMode::D8),
        Cpu::execute_jr,
        Cpu::fetch_d8,
    );
    instructions[0x29] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_R(RegisterType::HL, RegisterType::HL)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_r(RegisterType::HL, RegisterType::HL),
    );
    instructions[0x2A] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_HLI(RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_hli(RegisterType::A),
    );
    instructions[0x2B] = InstructionWrapper::new(
        InstructionType::Dec,
        InstructionArgs::default(AddressMode::R(RegisterType::HL)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_r(RegisterType::HL),
    );
    instructions[0x2C] = InstructionWrapper::new(
        InstructionType::Inc,
        InstructionArgs::default(AddressMode::R(RegisterType::L)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_r(RegisterType::L),
    );
    instructions[0x2D] = InstructionWrapper::new(
        InstructionType::Dec,
        InstructionArgs::default(AddressMode::R(RegisterType::L)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_r(RegisterType::L),
    );
    instructions[0x2E] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::L)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_d8(RegisterType::L),
    );
    instructions[0x2F] = InstructionWrapper::new(
        InstructionType::Cpl,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_cpl,
        Cpu::fetch_impl,
    );

    // 0x3X
    instructions[0x30] = InstructionWrapper::new(
        InstructionType::Jr,
        InstructionArgs::new(Some(ConditionType::NC), 0, AddressMode::D8),
        Cpu::execute_jr,
        Cpu::fetch_d8,
    );
    instructions[0x31] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_D16(RegisterType::SP)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_d16(RegisterType::SP),
    );
    instructions[0x32] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::HLD_R(RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_hld_r(RegisterType::A),
    );
    instructions[0x33] = InstructionWrapper::new(
        InstructionType::Inc,
        InstructionArgs::default(AddressMode::R(RegisterType::SP)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_r(RegisterType::SP),
    );
    instructions[0x34] = InstructionWrapper::new(
        InstructionType::Inc,
        InstructionArgs::default(AddressMode::MR(RegisterType::HL)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_mr(RegisterType::HL),
    );
    instructions[0x35] = InstructionWrapper::new(
        InstructionType::Dec,
        InstructionArgs::default(AddressMode::MR(RegisterType::HL)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_mr(RegisterType::HL),
    );
    instructions[0x36] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::MR_D8(RegisterType::HL)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_mr_d8(RegisterType::HL),
    );
    instructions[0x37] = InstructionWrapper::new(
        InstructionType::Scf,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_scf,
        Cpu::fetch_impl,
    );
    instructions[0x38] = InstructionWrapper::new(
        InstructionType::Jr,
        InstructionArgs::new(Some(ConditionType::C), 0, AddressMode::D8),
        Cpu::execute_jr,
        Cpu::fetch_d8,
    );
    instructions[0x39] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_R(RegisterType::HL, RegisterType::SP)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_r(RegisterType::HL, RegisterType::SP),
    );
    instructions[0x3A] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_HLD(RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_hld(RegisterType::A),
    );
    instructions[0x3B] = InstructionWrapper::new(
        InstructionType::Dec,
        InstructionArgs::default(AddressMode::R(RegisterType::SP)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_r(RegisterType::SP),
    );
    instructions[0x3C] = InstructionWrapper::new(
        InstructionType::Inc,
        InstructionArgs::default(AddressMode::R(RegisterType::A)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_r(RegisterType::A),
    );
    instructions[0x3D] = InstructionWrapper::new(
        InstructionType::Dec,
        InstructionArgs::default(AddressMode::R(RegisterType::A)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_r(RegisterType::A),
    );
    instructions[0x3E] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0x3F] = InstructionWrapper::new(
        InstructionType::Ccf,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_ccf,
        Cpu::fetch_impl,
    );

    // 0x4X
    instructions[0x40] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::B, RegisterType::B)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::B),
    );
    instructions[0x41] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::B, RegisterType::C)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::C),
    );
    instructions[0x42] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::B, RegisterType::D)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::D),
    );
    instructions[0x43] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::B, RegisterType::E)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::E),
    );
    instructions[0x44] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::B, RegisterType::H)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::H),
    );
    instructions[0x45] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::B, RegisterType::L)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::L),
    );
    instructions[0x46] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::B, RegisterType::HL)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_mr(RegisterType::B, RegisterType::HL),
    );
    instructions[0x47] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::B, RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::B, RegisterType::A),
    );
    instructions[0x48] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::C, RegisterType::B)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::B),
    );
    instructions[0x49] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::C, RegisterType::C)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::C),
    );
    instructions[0x4A] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::C, RegisterType::D)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::D),
    );
    instructions[0x4B] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::C, RegisterType::E)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::E),
    );
    instructions[0x4C] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::C, RegisterType::H)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::H),
    );
    instructions[0x4D] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::C, RegisterType::L)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::L),
    );
    instructions[0x4E] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::C, RegisterType::HL)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_mr(RegisterType::C, RegisterType::HL),
    );
    instructions[0x4F] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::C, RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::C, RegisterType::A),
    );

    // 0x5X
    instructions[0x50] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::D, RegisterType::B)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::B),
    );
    instructions[0x51] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::D, RegisterType::C)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::C),
    );
    instructions[0x52] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::D, RegisterType::D)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::D),
    );
    instructions[0x53] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::D, RegisterType::E)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::E),
    );
    instructions[0x54] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::D, RegisterType::H)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::H),
    );
    instructions[0x55] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::D, RegisterType::L)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::L),
    );
    instructions[0x56] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::D, RegisterType::HL)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_mr(RegisterType::D, RegisterType::HL),
    );
    instructions[0x57] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::D, RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::D, RegisterType::A),
    );
    instructions[0x58] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::E, RegisterType::B)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::B),
    );
    instructions[0x59] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::E, RegisterType::C)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::C),
    );
    instructions[0x5A] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::E, RegisterType::D)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::D),
    );
    instructions[0x5B] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::E, RegisterType::E)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::E),
    );
    instructions[0x5C] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::E, RegisterType::H)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::H),
    );
    instructions[0x5D] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::E, RegisterType::L)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::L),
    );
    instructions[0x5E] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::E, RegisterType::HL)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_mr(RegisterType::E, RegisterType::HL),
    );
    instructions[0x5F] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::E, RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::E, RegisterType::A),
    );

    // 0x6X
    instructions[0x60] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::H, RegisterType::B)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::B),
    );
    instructions[0x61] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::H, RegisterType::C)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::C),
    );
    instructions[0x62] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::H, RegisterType::D)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::D),
    );
    instructions[0x63] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::H, RegisterType::E)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::E),
    );
    instructions[0x64] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::H, RegisterType::H)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::H),
    );
    instructions[0x65] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::H, RegisterType::L)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::L),
    );
    instructions[0x66] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::H, RegisterType::HL)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_mr(RegisterType::H, RegisterType::HL),
    );
    instructions[0x67] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::H, RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::H, RegisterType::A),
    );
    instructions[0x68] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::L, RegisterType::B)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::B),
    );
    instructions[0x69] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::L, RegisterType::C)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::C),
    );
    instructions[0x6A] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::L, RegisterType::D)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::D),
    );
    instructions[0x6B] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::L, RegisterType::E)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::E),
    );
    instructions[0x6C] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::L, RegisterType::H)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::H),
    );
    instructions[0x6D] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::L, RegisterType::L)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::L),
    );
    instructions[0x6E] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::L, RegisterType::HL)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_mr(RegisterType::L, RegisterType::HL),
    );
    instructions[0x6F] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::L, RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::L, RegisterType::A),
    );

    // 0x7X
    instructions[0x76] = InstructionWrapper::new(
        InstructionType::Halt,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_halt,
        Cpu::fetch_impl,
    );
    instructions[0x70] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::MR_R(RegisterType::HL, RegisterType::B)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::B),
    );
    instructions[0x71] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::MR_R(RegisterType::HL, RegisterType::C)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::C),
    );
    instructions[0x72] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::MR_R(RegisterType::HL, RegisterType::D)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::D),
    );
    instructions[0x73] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::MR_R(RegisterType::HL, RegisterType::E)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::E),
    );
    instructions[0x74] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::MR_R(RegisterType::HL, RegisterType::H)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::H),
    );
    instructions[0x75] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::MR_R(RegisterType::HL, RegisterType::L)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::L),
    );
    instructions[0x77] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::MR_R(RegisterType::HL, RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_mr_r(RegisterType::HL, RegisterType::A),
    );
    instructions[0x78] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0x79] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0x7A] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0x7B] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0x7C] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0x7D] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0x7E] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0x7F] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );

    // 0x8X
    instructions[0x80] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0x81] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0x82] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0x83] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0x84] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0x85] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0x86] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0x87] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );
    instructions[0x88] = InstructionWrapper::new(
        InstructionType::Adc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_adc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0x89] = InstructionWrapper::new(
        InstructionType::Adc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_adc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0x8A] = InstructionWrapper::new(
        InstructionType::Adc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_adc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0x8B] = InstructionWrapper::new(
        InstructionType::Adc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_adc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0x8C] = InstructionWrapper::new(
        InstructionType::Adc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_adc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0x8D] = InstructionWrapper::new(
        InstructionType::Adc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_adc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0x8E] = InstructionWrapper::new(
        InstructionType::Adc,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_adc,
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0x8F] = InstructionWrapper::new(
        InstructionType::Adc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_adc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );

    // 0x9X
    instructions[0x90] = InstructionWrapper::new(
        InstructionType::Sub,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_sub,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0x91] = InstructionWrapper::new(
        InstructionType::Sub,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_sub,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0x92] = InstructionWrapper::new(
        InstructionType::Sub,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_sub,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0x93] = InstructionWrapper::new(
        InstructionType::Sub,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_sub,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0x94] = InstructionWrapper::new(
        InstructionType::Sub,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_sub,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0x95] = InstructionWrapper::new(
        InstructionType::Sub,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_sub,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0x96] = InstructionWrapper::new(
        InstructionType::Sub,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_sub,
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0x97] = InstructionWrapper::new(
        InstructionType::Sub,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_sub,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );
    instructions[0x98] = InstructionWrapper::new(
        InstructionType::Sbc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_sbc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0x99] = InstructionWrapper::new(
        InstructionType::Sbc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_sbc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0x9A] = InstructionWrapper::new(
        InstructionType::Sbc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_sbc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0x9B] = InstructionWrapper::new(
        InstructionType::Sbc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_sbc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0x9C] = InstructionWrapper::new(
        InstructionType::Sbc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_sbc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0x9D] = InstructionWrapper::new(
        InstructionType::Sbc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_sbc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0x9E] = InstructionWrapper::new(
        InstructionType::Sbc,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_sbc,
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0x9F] = InstructionWrapper::new(
        InstructionType::Sbc,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_sbc,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );

    // 0xAX
    instructions[0xA0] = InstructionWrapper::new(
        InstructionType::And,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_and,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0xA1] = InstructionWrapper::new(
        InstructionType::And,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_and,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0xA2] = InstructionWrapper::new(
        InstructionType::And,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_and,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0xA3] = InstructionWrapper::new(
        InstructionType::And,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_and,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0xA4] = InstructionWrapper::new(
        InstructionType::And,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_and,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0xA5] = InstructionWrapper::new(
        InstructionType::And,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_and,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0xA6] = InstructionWrapper::new(
        InstructionType::And,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_and,
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0xA7] = InstructionWrapper::new(
        InstructionType::And,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_and,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );
    instructions[0xA8] = InstructionWrapper::new(
        InstructionType::Xor,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_xor,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0xA9] = InstructionWrapper::new(
        InstructionType::Xor,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_xor,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0xAA] = InstructionWrapper::new(
        InstructionType::Xor,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_xor,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0xAB] = InstructionWrapper::new(
        InstructionType::Xor,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_xor,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0xAC] = InstructionWrapper::new(
        InstructionType::Xor,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_xor,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0xAD] = InstructionWrapper::new(
        InstructionType::Xor,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_xor,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0xAE] = InstructionWrapper::new(
        InstructionType::Xor,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_xor,
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0xAF] = InstructionWrapper::new(
        InstructionType::Xor,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_xor,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );

    // 0xBX
    instructions[0xB0] = InstructionWrapper::new(
        InstructionType::Or,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_or,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0xB1] = InstructionWrapper::new(
        InstructionType::Or,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_or,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0xB2] = InstructionWrapper::new(
        InstructionType::Or,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_or,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0xB3] = InstructionWrapper::new(
        InstructionType::Or,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_or,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0xB4] = InstructionWrapper::new(
        InstructionType::Or,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_or,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0xB5] = InstructionWrapper::new(
        InstructionType::Or,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_or,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0xB6] = InstructionWrapper::new(
        InstructionType::Or,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_or,
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0xB7] = InstructionWrapper::new(
        InstructionType::Or,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_or,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );
    instructions[0xB8] = InstructionWrapper::new(
        InstructionType::Cp,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_cp,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::B),
    );
    instructions[0xB9] = InstructionWrapper::new(
        InstructionType::Cp,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_cp,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::C),
    );
    instructions[0xBA] = InstructionWrapper::new(
        InstructionType::Cp,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_cp,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::D),
    );
    instructions[0xBB] = InstructionWrapper::new(
        InstructionType::Cp,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_cp,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::E),
    );
    instructions[0xBC] = InstructionWrapper::new(
        InstructionType::Cp,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_cp,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::H),
    );
    instructions[0xBD] = InstructionWrapper::new(
        InstructionType::Cp,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_cp,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::L),
    );
    instructions[0xBE] = InstructionWrapper::new(
        InstructionType::Cp,
        InstructionArgs::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_cp,
        |cpu| cpu.fetch_r_mr(RegisterType::A, RegisterType::HL),
    );
    instructions[0xBF] = InstructionWrapper::new(
        InstructionType::Cp,
        InstructionArgs::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_cp,
        |cpu| cpu.fetch_r_r(RegisterType::A, RegisterType::A),
    );

    // 0xCX
    instructions[0xC0] = InstructionWrapper::new(
        InstructionType::Ret,
        InstructionArgs::new(Some(ConditionType::NZ), 0, AddressMode::IMP),
        Cpu::execute_ret,
        Cpu::fetch_impl,
    );
    instructions[0xC1] = InstructionWrapper::new(
        InstructionType::Pop,
        InstructionArgs::default(AddressMode::R(RegisterType::BC)),
        Cpu::execute_pop,
        |cpu| cpu.fetch_r(RegisterType::BC),
    );
    instructions[0xC2] = InstructionWrapper::new(
        InstructionType::Jp,
        InstructionArgs::new(Some(ConditionType::NZ), 0, AddressMode::D16),
        Cpu::execute_jp,
        Cpu::fetch_d16,
    );
    instructions[0xC3] = InstructionWrapper::new(
        InstructionType::Jp,
        InstructionArgs::default(AddressMode::D16),
        Cpu::execute_jp,
        Cpu::fetch_d16,
    );
    instructions[0xC5] = InstructionWrapper::new(
        InstructionType::Push,
        InstructionArgs::default(AddressMode::R(RegisterType::BC)),
        Cpu::execute_push,
        |cpu| cpu.fetch_r(RegisterType::BC),
    );
    instructions[0xC6] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xC7] = InstructionWrapper::new(
        InstructionType::Rst,
        InstructionArgs::new(None, 0x00, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );
    instructions[0xC4] = InstructionWrapper::new(
        InstructionType::Call,
        InstructionArgs::new(Some(ConditionType::NZ), 0, AddressMode::D16),
        Cpu::execute_call,
        Cpu::fetch_d16,
    );
    instructions[0xC8] = InstructionWrapper::new(
        InstructionType::Ret,
        InstructionArgs::new(Some(ConditionType::Z), 0, AddressMode::IMP),
        Cpu::execute_ret,
        Cpu::fetch_impl,
    );
    instructions[0xC9] = InstructionWrapper::new(
        InstructionType::Ret,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_ret,
        Cpu::fetch_impl,
    );
    instructions[0xCC] = InstructionWrapper::new(
        InstructionType::Call,
        InstructionArgs::new(Some(ConditionType::Z), 0, AddressMode::D16),
        Cpu::execute_call,
        Cpu::fetch_d16,
    );
    instructions[0xCD] = InstructionWrapper::new(
        InstructionType::Call,
        InstructionArgs::default(AddressMode::D16),
        Cpu::execute_call,
        Cpu::fetch_d16,
    );
    instructions[0xCA] = InstructionWrapper::new(
        InstructionType::Jp,
        InstructionArgs::new(Some(ConditionType::Z), 0, AddressMode::D16),
        Cpu::execute_jp,
        Cpu::fetch_d16,
    );
    instructions[0xCB] = InstructionWrapper::new(
        InstructionType::Prefix,
        InstructionArgs::default(AddressMode::D8),
        Cpu::execute_prefix,
        Cpu::fetch_d8,
    );
    instructions[0xCE] = InstructionWrapper::new(
        InstructionType::Adc,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_adc,
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xCF] = InstructionWrapper::new(
        InstructionType::Rst,
        InstructionArgs::new(None, 0x08, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );

    // 0xDX
    instructions[0xD0] = InstructionWrapper::new(
        InstructionType::Ret,
        InstructionArgs::new(Some(ConditionType::NC), 0, AddressMode::IMP),
        Cpu::execute_ret,
        Cpu::fetch_impl,
    );
    instructions[0xD1] = InstructionWrapper::new(
        InstructionType::Pop,
        InstructionArgs::default(AddressMode::R(RegisterType::DE)),
        Cpu::execute_pop,
        |cpu| cpu.fetch_r(RegisterType::DE),
    );
    instructions[0xD2] = InstructionWrapper::new(
        InstructionType::Jp,
        InstructionArgs::new(Some(ConditionType::NC), 0, AddressMode::D16),
        Cpu::execute_jp,
        Cpu::fetch_d16,
    );
    instructions[0xD4] = InstructionWrapper::new(
        InstructionType::Call,
        InstructionArgs::new(Some(ConditionType::NC), 0, AddressMode::D16),
        Cpu::execute_call,
        Cpu::fetch_d16,
    );
    instructions[0xD5] = InstructionWrapper::new(
        InstructionType::Push,
        InstructionArgs::default(AddressMode::R(RegisterType::DE)),
        Cpu::execute_push,
        |cpu| cpu.fetch_r(RegisterType::DE),
    );
    instructions[0xD6] = InstructionWrapper::new(
        InstructionType::Sub,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_sub,
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xD7] = InstructionWrapper::new(
        InstructionType::Rst,
        InstructionArgs::new(None, 0x10, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );
    instructions[0xD8] = InstructionWrapper::new(
        InstructionType::Ret,
        InstructionArgs::new(Some(ConditionType::C), 0, AddressMode::IMP),
        Cpu::execute_ret,
        Cpu::fetch_impl,
    );
    instructions[0xD9] = InstructionWrapper::new(
        InstructionType::Reti,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_reti,
        Cpu::fetch_impl,
    );
    instructions[0xDC] = InstructionWrapper::new(
        InstructionType::Call,
        InstructionArgs::new(Some(ConditionType::C), 0, AddressMode::D16),
        Cpu::execute_call,
        Cpu::fetch_d16,
    );
    instructions[0xDA] = InstructionWrapper::new(
        InstructionType::Jp,
        InstructionArgs::new(Some(ConditionType::C), 0, AddressMode::D16),
        Cpu::execute_jp,
        Cpu::fetch_d16,
    );
    instructions[0xDE] = InstructionWrapper::new(
        InstructionType::Sbc,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_sbc,
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xDF] = InstructionWrapper::new(
        InstructionType::Rst,
        InstructionArgs::new(None, 0x18, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );

    // 0xEX
    instructions[0xE0] = InstructionWrapper::new(
        InstructionType::Ldh,
        InstructionArgs::default(AddressMode::A8_R(RegisterType::A)),
        Cpu::execute_ldh,
        |cpu| cpu.fetch_a8_r(RegisterType::A),
    );
    instructions[0xE1] = InstructionWrapper::new(
        InstructionType::Pop,
        InstructionArgs::default(AddressMode::R(RegisterType::HL)),
        Cpu::execute_pop,
        |cpu| cpu.fetch_r(RegisterType::HL),
    );
    instructions[0xE2] = InstructionWrapper::new(
        InstructionType::Ldh,
        InstructionArgs::default(AddressMode::MR_R(RegisterType::C, RegisterType::A)),
        Cpu::execute_ldh,
        |cpu| cpu.fetch_mr_r(RegisterType::C, RegisterType::A),
    );
    instructions[0xE5] = InstructionWrapper::new(
        InstructionType::Push,
        InstructionArgs::default(AddressMode::R(RegisterType::HL)),
        Cpu::execute_push,
        |cpu| cpu.fetch_r(RegisterType::HL),
    );
    instructions[0xE6] = InstructionWrapper::new(
        InstructionType::And,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_and,
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xE7] = InstructionWrapper::new(
        InstructionType::Rst,
        InstructionArgs::new(None, 0x20, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );
    instructions[0xE8] = InstructionWrapper::new(
        InstructionType::Add,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::SP)),
        Cpu::execute_add,
        |cpu| cpu.fetch_r_d8(RegisterType::SP),
    );
    instructions[0xEA] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::A16_R(RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_a16_r(RegisterType::A),
    );
    instructions[0xE9] = InstructionWrapper::new(
        InstructionType::Jp,
        InstructionArgs::default(AddressMode::R(RegisterType::HL)),
        Cpu::execute_jp,
        |cpu| cpu.fetch_r(RegisterType::HL),
    );
    instructions[0xEE] = InstructionWrapper::new(
        InstructionType::Xor,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_xor,
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xEF] = InstructionWrapper::new(
        InstructionType::Rst,
        InstructionArgs::new(None, 0x28, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );

    // 0xFX
    instructions[0xF0] = InstructionWrapper::new(
        InstructionType::Ldh,
        InstructionArgs::default(AddressMode::R_HA8(RegisterType::A)),
        Cpu::execute_ldh,
        |cpu| cpu.fetch_r_ha8(RegisterType::A),
    );
    instructions[0xF1] = InstructionWrapper::new(
        InstructionType::Pop,
        InstructionArgs::default(AddressMode::R(RegisterType::AF)),
        Cpu::execute_pop,
        |cpu| cpu.fetch_r(RegisterType::AF),
    );
    instructions[0xF2] = InstructionWrapper::new(
        InstructionType::Ldh,
        InstructionArgs::default(AddressMode::R_HMR(RegisterType::A, RegisterType::C)),
        Cpu::execute_ldh,
        |cpu| cpu.fetch_r_hmr(RegisterType::A, RegisterType::C),
    );
    instructions[0xF3] = InstructionWrapper::new(
        InstructionType::Di,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_di,
        Cpu::fetch_impl,
    );
    instructions[0xF5] = InstructionWrapper::new(
        InstructionType::Push,
        InstructionArgs::default(AddressMode::R(RegisterType::AF)),
        Cpu::execute_push,
        |cpu| cpu.fetch_r(RegisterType::AF),
    );
    instructions[0xF6] = InstructionWrapper::new(
        InstructionType::Or,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_or,
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xF7] = InstructionWrapper::new(
        InstructionType::Rst,
        InstructionArgs::new(None, 0x30, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );
    instructions[0xF8] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::LH_SPi8),
        Cpu::execute_ld,
        Cpu::fetch_lh_spi8,
    );
    instructions[0xF9] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_R(RegisterType::SP, RegisterType::HL)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_r(RegisterType::SP, RegisterType::HL),
    );
    instructions[0xFA] = InstructionWrapper::new(
        InstructionType::Ld,
        InstructionArgs::default(AddressMode::R_A16(RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_a16(RegisterType::A),
    );
    instructions[0xFB] = InstructionWrapper::new(
        InstructionType::Ei,
        InstructionArgs::default(AddressMode::IMP),
        Cpu::execute_ei,
        Cpu::fetch_impl,
    );
    instructions[0xFE] = InstructionWrapper::new(
        InstructionType::Cp,
        InstructionArgs::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_cp,
        |cpu| cpu.fetch_r_d8(RegisterType::A),
    );
    instructions[0xFF] = InstructionWrapper::new(
        InstructionType::Rst,
        InstructionArgs::new(None, 0x38, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );

    instructions
};
