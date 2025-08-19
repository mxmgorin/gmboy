use crate::cpu::instructions::fetch::AddressMode;
use crate::cpu::instructions::condition::ConditionType;
use crate::cpu::instructions::*;
use crate::cpu::{Cpu, RegisterType};

const INSTRUCTIONS_LEN: usize = 0xFF + 1;

pub const INSTRUCTIONS_BY_OPCODES: [Instruction; INSTRUCTIONS_LEN] = {
    let mut instructions = {
        let mut array = [Instruction::unknown(0); INSTRUCTIONS_LEN];
        let mut i = 0;
        while i < INSTRUCTIONS_LEN {
            array[i] = Instruction::unknown(i as u8);
            i += 1;
        }
        array
    };

    // 0x0X
    instructions[0x00] = Instruction::new(
        Mnemonic::Nop,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_nop,
        Cpu::fetch_impl,
    );
    instructions[0x01] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_D16(RegisterType::BC)),
        Cpu::execute_ld,
        Cpu::fetch_r_d16::<{RegisterType::BC as u8}>,
    );
    instructions[0x02] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::MR_R(RegisterType::BC, RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_mr_r::<{ RegisterType::BC as u8 }, { RegisterType::A as u8 }>,
    );
    instructions[0x03] = Instruction::new(
        Mnemonic::Inc,
        InstructionSpec::default(AddressMode::R(RegisterType::BC)),
        Cpu::execute_inc,
        Cpu::fetch_r::<{RegisterType::BC as u8}>,
    );
    instructions[0x04] = Instruction::new(
        Mnemonic::Inc,
        InstructionSpec::default(AddressMode::R(RegisterType::B)),
        Cpu::execute_inc,
        Cpu::fetch_r::<{RegisterType::B as u8}>,
    );
    instructions[0x05] = Instruction::new(
        Mnemonic::Dec,
        InstructionSpec::default(AddressMode::R(RegisterType::B)),
        Cpu::execute_dec,
        Cpu::fetch_r::<{RegisterType::B as u8}>,
    );
    instructions[0x06] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::B)),
        Cpu::execute_ld,
        Cpu::fetch_r_d8::<{RegisterType::B as u8}>,
    );
    instructions[0x07] = Instruction::new(
        Mnemonic::Rlca,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_rlca,
        Cpu::fetch_impl,
    );
    instructions[0x08] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::A16_R(RegisterType::SP)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_a16_r(RegisterType::SP),
    );
    instructions[0x09] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_R(RegisterType::HL, RegisterType::BC)),
        Cpu::execute_add,
        Cpu::fetch_r_r::<{ RegisterType::HL as u8 }, { RegisterType::BC as u8 }>,
    );
    instructions[0x0A] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::A, RegisterType::BC)),
        Cpu::execute_ld,
        Cpu::fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::BC as u8 }>,
    );
    instructions[0x0B] = Instruction::new(
        Mnemonic::Dec,
        InstructionSpec::default(AddressMode::R(RegisterType::BC)),
        Cpu::execute_dec,
        Cpu::fetch_r::<{RegisterType::BC as u8}>,
    );
    instructions[0x0C] = Instruction::new(
        Mnemonic::Inc,
        InstructionSpec::default(AddressMode::R(RegisterType::C)),
        Cpu::execute_inc,
        Cpu::fetch_r::<{RegisterType::C as u8}>,
    );
    instructions[0x0D] = Instruction::new(
        Mnemonic::Dec,
        InstructionSpec::default(AddressMode::R(RegisterType::C)),
        Cpu::execute_dec,
        Cpu::fetch_r::<{RegisterType::C as u8}>,
    );
    instructions[0x0E] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::C)),
        Cpu::execute_ld,
        Cpu::fetch_r_d8::<{RegisterType::C as u8}>,
    );
    instructions[0x0F] = Instruction::new(
        Mnemonic::Rrca,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_rrca,
        Cpu::fetch_impl,
    );

    // 0x1X
    instructions[0x10] = Instruction::new(
        Mnemonic::Stop,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_stop,
        Cpu::fetch_impl,
    );
    instructions[0x11] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_D16(RegisterType::DE)),
        Cpu::execute_ld,
        Cpu::fetch_r_d16::<{RegisterType::DE as u8}>,
    );
    instructions[0x12] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::MR_R(RegisterType::DE, RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_mr_r::<{ RegisterType::DE as u8 }, { RegisterType::A as u8 }>,
    );
    instructions[0x13] = Instruction::new(
        Mnemonic::Inc,
        InstructionSpec::default(AddressMode::R(RegisterType::DE)),
        Cpu::execute_inc,
        Cpu::fetch_r::<{RegisterType::DE as u8}>,
    );
    instructions[0x14] = Instruction::new(
        Mnemonic::Inc,
        InstructionSpec::default(AddressMode::R(RegisterType::D)),
        Cpu::execute_inc,
        Cpu::fetch_r::<{RegisterType::D as u8}>,
    );
    instructions[0x15] = Instruction::new(
        Mnemonic::Dec,
        InstructionSpec::default(AddressMode::R(RegisterType::D)),
        Cpu::execute_dec,
        Cpu::fetch_r::<{RegisterType::D as u8}>,
    );
    instructions[0x16] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::D)),
        Cpu::execute_ld,
        Cpu::fetch_r_d8::<{RegisterType::D as u8}>,
    );
    instructions[0x17] = Instruction::new(
        Mnemonic::RLA,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_rla,
        Cpu::fetch_impl,
    );
    instructions[0x18] = Instruction::new(
        Mnemonic::Jr,
        InstructionSpec::default(AddressMode::D8),
        Cpu::execute_jr,
        Cpu::fetch_d8,
    );
    instructions[0x19] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_R(RegisterType::HL, RegisterType::DE)),
        Cpu::execute_add,
        Cpu::fetch_r_r::<{ RegisterType::HL as u8 }, { RegisterType::DE as u8 }>,
    );
    instructions[0x1A] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::A, RegisterType::DE)),
        Cpu::execute_ld,
        Cpu::fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::DE as u8 }>,
    );
    instructions[0x1B] = Instruction::new(
        Mnemonic::Dec,
        InstructionSpec::default(AddressMode::R(RegisterType::DE)),
        Cpu::execute_dec,
        Cpu::fetch_r::<{RegisterType::DE as u8}>,
    );
    instructions[0x1C] = Instruction::new(
        Mnemonic::Inc,
        InstructionSpec::default(AddressMode::R(RegisterType::E)),
        Cpu::execute_inc,
        Cpu::fetch_r::<{RegisterType::E as u8}>,
    );
    instructions[0x1D] = Instruction::new(
        Mnemonic::Dec,
        InstructionSpec::default(AddressMode::R(RegisterType::E)),
        Cpu::execute_dec,
        Cpu::fetch_r::<{RegisterType::E as u8}>,
    );
    instructions[0x1E] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::E)),
        Cpu::execute_ld,
        Cpu::fetch_r_d8::<{RegisterType::E as u8}>,
    );
    instructions[0x1F] = Instruction::new(
        Mnemonic::Rra,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_rra,
        Cpu::fetch_impl,
    );

    // 0x2X
    instructions[0x20] = Instruction::new(
        Mnemonic::Jr,
        InstructionSpec::new(Some(ConditionType::NZ), 0, AddressMode::D8),
        Cpu::execute_jr,
        Cpu::fetch_d8,
    );
    instructions[0x21] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_D16(RegisterType::HL)),
        Cpu::execute_ld,
        Cpu::fetch_r_d16::<{RegisterType::HL as u8}>,
    );
    instructions[0x22] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::HLI_R(RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_hli_r_a,
    );
    instructions[0x23] = Instruction::new(
        Mnemonic::Inc,
        InstructionSpec::default(AddressMode::R(RegisterType::HL)),
        Cpu::execute_inc,
        Cpu::fetch_r::<{RegisterType::HL as u8}>,
    );
    instructions[0x24] = Instruction::new(
        Mnemonic::Inc,
        InstructionSpec::default(AddressMode::R(RegisterType::H)),
        Cpu::execute_inc,
        Cpu::fetch_r::<{RegisterType::H as u8}>,
    );
    instructions[0x25] = Instruction::new(
        Mnemonic::Dec,
        InstructionSpec::default(AddressMode::R(RegisterType::H)),
        Cpu::execute_dec,
        Cpu::fetch_r::<{RegisterType::H as u8}>,
    );
    instructions[0x26] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::H)),
        Cpu::execute_ld,
        Cpu::fetch_r_d8::<{RegisterType::H as u8}>,
    );
    instructions[0x27] = Instruction::new(
        Mnemonic::Daa,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_daa,
        Cpu::fetch_impl,
    );
    instructions[0x28] = Instruction::new(
        Mnemonic::Jr,
        InstructionSpec::new(Some(ConditionType::Z), 0, AddressMode::D8),
        Cpu::execute_jr,
        Cpu::fetch_d8,
    );
    instructions[0x29] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_R(RegisterType::HL, RegisterType::HL)),
        Cpu::execute_add,
        Cpu::fetch_r_r::<{ RegisterType::HL as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0x2A] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_HLI(RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_r_hli_a,
    );
    instructions[0x2B] = Instruction::new(
        Mnemonic::Dec,
        InstructionSpec::default(AddressMode::R(RegisterType::HL)),
        Cpu::execute_dec,
        Cpu::fetch_r::<{RegisterType::HL as u8}>,
    );
    instructions[0x2C] = Instruction::new(
        Mnemonic::Inc,
        InstructionSpec::default(AddressMode::R(RegisterType::L)),
        Cpu::execute_inc,
        Cpu::fetch_r::<{RegisterType::L as u8}>,
    );
    instructions[0x2D] = Instruction::new(
        Mnemonic::Dec,
        InstructionSpec::default(AddressMode::R(RegisterType::L)),
        Cpu::execute_dec,
        Cpu::fetch_r::<{RegisterType::L as u8}>,
    );
    instructions[0x2E] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::L)),
        Cpu::execute_ld,
        Cpu::fetch_r_d8::<{RegisterType::L as u8}>,
    );
    instructions[0x2F] = Instruction::new(
        Mnemonic::Cpl,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_cpl,
        Cpu::fetch_impl,
    );

    // 0x3X
    instructions[0x30] = Instruction::new(
        Mnemonic::Jr,
        InstructionSpec::new(Some(ConditionType::NC), 0, AddressMode::D8),
        Cpu::execute_jr,
        Cpu::fetch_d8,
    );
    instructions[0x31] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_D16(RegisterType::SP)),
        Cpu::execute_ld,
        Cpu::fetch_r_d16::<{RegisterType::SP as u8}>,
    );
    instructions[0x32] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::HLD_R(RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_hld_r_a,
    );
    instructions[0x33] = Instruction::new(
        Mnemonic::Inc,
        InstructionSpec::default(AddressMode::R(RegisterType::SP)),
        Cpu::execute_inc,
        Cpu::fetch_r::<{RegisterType::SP as u8}>,
    );
    instructions[0x34] = Instruction::new(
        Mnemonic::Inc,
        InstructionSpec::default(AddressMode::MR(RegisterType::HL)),
        Cpu::execute_inc,
        |cpu| cpu.fetch_mr(RegisterType::HL),
    );
    instructions[0x35] = Instruction::new(
        Mnemonic::Dec,
        InstructionSpec::default(AddressMode::MR(RegisterType::HL)),
        Cpu::execute_dec,
        |cpu| cpu.fetch_mr(RegisterType::HL),
    );
    instructions[0x36] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::MR_D8(RegisterType::HL)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_mr_d8(RegisterType::HL),
    );
    instructions[0x37] = Instruction::new(
        Mnemonic::Scf,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_scf,
        Cpu::fetch_impl,
    );
    instructions[0x38] = Instruction::new(
        Mnemonic::Jr,
        InstructionSpec::new(Some(ConditionType::C), 0, AddressMode::D8),
        Cpu::execute_jr,
        Cpu::fetch_d8,
    );
    instructions[0x39] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_R(RegisterType::HL, RegisterType::SP)),
        Cpu::execute_add,
        Cpu::fetch_r_r::<{ RegisterType::HL as u8 }, { RegisterType::SP as u8 }>,
    );
    instructions[0x3A] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_HLD(RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_r_hld_a,
    );
    instructions[0x3B] = Instruction::new(
        Mnemonic::Dec,
        InstructionSpec::default(AddressMode::R(RegisterType::SP)),
        Cpu::execute_dec,
        Cpu::fetch_r::<{RegisterType::SP as u8}>,
    );
    instructions[0x3C] = Instruction::new(
        Mnemonic::Inc,
        InstructionSpec::default(AddressMode::R(RegisterType::A)),
        Cpu::execute_inc,
        Cpu::fetch_r::<{RegisterType::A as u8}>,
    );
    instructions[0x3D] = Instruction::new(
        Mnemonic::Dec,
        InstructionSpec::default(AddressMode::R(RegisterType::A)),
        Cpu::execute_dec,
        Cpu::fetch_r::<{RegisterType::A as u8}>,
    );
    instructions[0x3E] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_r_d8::<{RegisterType::A as u8}>,
    );
    instructions[0x3F] = Instruction::new(
        Mnemonic::Ccf,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_ccf,
        Cpu::fetch_impl,
    );

    // 0x4X
    instructions[0x40] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::B, RegisterType::B)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::B as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0x41] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::B, RegisterType::C)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::B as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0x42] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::B, RegisterType::D)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::B as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0x43] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::B, RegisterType::E)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::B as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0x44] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::B, RegisterType::H)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::B as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0x45] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::B, RegisterType::L)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::B as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0x46] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::B, RegisterType::HL)),
        Cpu::execute_ld,
        Cpu::fetch_r_mr::<{ RegisterType::B as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0x47] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::B, RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::B as u8 }, { RegisterType::A as u8 }>,
    );
    instructions[0x48] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::C, RegisterType::B)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::C as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0x49] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::C, RegisterType::C)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::C as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0x4A] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::C, RegisterType::D)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::C as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0x4B] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::C, RegisterType::E)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::C as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0x4C] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::C, RegisterType::H)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::C as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0x4D] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::C, RegisterType::L)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::C as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0x4E] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::C, RegisterType::HL)),
        Cpu::execute_ld,
        Cpu::fetch_r_mr::<{ RegisterType::C as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0x4F] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::C, RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::C as u8 }, { RegisterType::A as u8 }>,
    );

    // 0x5X
    instructions[0x50] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::D, RegisterType::B)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::D as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0x51] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::D, RegisterType::C)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::D as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0x52] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::D, RegisterType::D)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::D as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0x53] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::D, RegisterType::E)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::D as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0x54] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::D, RegisterType::H)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::D as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0x55] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::D, RegisterType::L)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::D as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0x56] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::D, RegisterType::HL)),
        Cpu::execute_ld,
        Cpu::fetch_r_mr::<{ RegisterType::D as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0x57] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::D, RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::D as u8 }, { RegisterType::A as u8 }>,
    );
    instructions[0x58] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::E, RegisterType::B)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::E as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0x59] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::E, RegisterType::C)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::E as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0x5A] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::E, RegisterType::D)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::E as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0x5B] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::E, RegisterType::E)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::E as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0x5C] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::E, RegisterType::H)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::E as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0x5D] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::E, RegisterType::L)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::E as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0x5E] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::E, RegisterType::HL)),
        Cpu::execute_ld,
        Cpu::fetch_r_mr::<{ RegisterType::E as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0x5F] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::E, RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::E as u8 }, { RegisterType::A as u8 }>,
    );

    // 0x6X
    instructions[0x60] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::H, RegisterType::B)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::H as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0x61] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::H, RegisterType::C)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::H as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0x62] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::H, RegisterType::D)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::H as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0x63] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::H, RegisterType::E)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::H as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0x64] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::H, RegisterType::H)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::H as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0x65] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::H, RegisterType::L)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::H as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0x66] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::H, RegisterType::HL)),
        Cpu::execute_ld,
        Cpu::fetch_r_mr::<{ RegisterType::H as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0x67] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::H, RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::H as u8 }, { RegisterType::A as u8 }>,
    );
    instructions[0x68] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::L, RegisterType::B)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::L as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0x69] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::L, RegisterType::C)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::L as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0x6A] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::L, RegisterType::D)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::L as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0x6B] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::L, RegisterType::E)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::L as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0x6C] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::L, RegisterType::H)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::L as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0x6D] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::L, RegisterType::L)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::L as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0x6E] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::L, RegisterType::HL)),
        Cpu::execute_ld,
        Cpu::fetch_r_mr::<{ RegisterType::L as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0x6F] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::L, RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::L as u8 }, { RegisterType::A as u8 }>,
    );

    // 0x7X
    instructions[0x76] = Instruction::new(
        Mnemonic::Halt,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_halt,
        Cpu::fetch_impl,
    );
    instructions[0x70] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::MR_R(RegisterType::HL, RegisterType::B)),
        Cpu::execute_ld,
        Cpu::fetch_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0x71] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::MR_R(RegisterType::HL, RegisterType::C)),
        Cpu::execute_ld,
        Cpu::fetch_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0x72] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::MR_R(RegisterType::HL, RegisterType::D)),
        Cpu::execute_ld,
        Cpu::fetch_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0x73] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::MR_R(RegisterType::HL, RegisterType::E)),
        Cpu::execute_ld,
        Cpu::fetch_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0x74] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::MR_R(RegisterType::HL, RegisterType::H)),
        Cpu::execute_ld,
        Cpu::fetch_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0x75] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::MR_R(RegisterType::HL, RegisterType::L)),
        Cpu::execute_ld,
        Cpu::fetch_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0x77] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::MR_R(RegisterType::HL, RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::A as u8 }>,
    );
    instructions[0x78] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0x79] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0x7A] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0x7B] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0x7C] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0x7D] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0x7E] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_ld,
        Cpu::fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0x7F] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>,
    );

    // 0x8X
    instructions[0x80] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_add,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0x81] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_add,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0x82] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_add,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0x83] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_add,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0x84] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_add,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0x85] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_add,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0x86] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_add,
        Cpu::fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0x87] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_add,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>,
    );
    instructions[0x88] = Instruction::new(
        Mnemonic::Adc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_adc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0x89] = Instruction::new(
        Mnemonic::Adc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_adc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0x8A] = Instruction::new(
        Mnemonic::Adc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_adc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0x8B] = Instruction::new(
        Mnemonic::Adc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_adc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0x8C] = Instruction::new(
        Mnemonic::Adc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_adc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0x8D] = Instruction::new(
        Mnemonic::Adc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_adc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0x8E] = Instruction::new(
        Mnemonic::Adc,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_adc,
        Cpu::fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0x8F] = Instruction::new(
        Mnemonic::Adc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_adc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>,
    );

    // 0x9X
    instructions[0x90] = Instruction::new(
        Mnemonic::Sub,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_sub,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0x91] = Instruction::new(
        Mnemonic::Sub,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_sub,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0x92] = Instruction::new(
        Mnemonic::Sub,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_sub,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0x93] = Instruction::new(
        Mnemonic::Sub,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_sub,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0x94] = Instruction::new(
        Mnemonic::Sub,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_sub,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0x95] = Instruction::new(
        Mnemonic::Sub,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_sub,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0x96] = Instruction::new(
        Mnemonic::Sub,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_sub,
        Cpu::fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0x97] = Instruction::new(
        Mnemonic::Sub,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_sub,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>,
    );
    instructions[0x98] = Instruction::new(
        Mnemonic::Sbc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_sbc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0x99] = Instruction::new(
        Mnemonic::Sbc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_sbc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0x9A] = Instruction::new(
        Mnemonic::Sbc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_sbc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0x9B] = Instruction::new(
        Mnemonic::Sbc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_sbc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0x9C] = Instruction::new(
        Mnemonic::Sbc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_sbc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0x9D] = Instruction::new(
        Mnemonic::Sbc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_sbc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0x9E] = Instruction::new(
        Mnemonic::Sbc,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_sbc,
        Cpu::fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0x9F] = Instruction::new(
        Mnemonic::Sbc,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_sbc,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>,
    );

    // 0xAX
    instructions[0xA0] = Instruction::new(
        Mnemonic::And,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_and,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0xA1] = Instruction::new(
        Mnemonic::And,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_and,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0xA2] = Instruction::new(
        Mnemonic::And,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_and,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0xA3] = Instruction::new(
        Mnemonic::And,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_and,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0xA4] = Instruction::new(
        Mnemonic::And,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_and,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0xA5] = Instruction::new(
        Mnemonic::And,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_and,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0xA6] = Instruction::new(
        Mnemonic::And,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_and,
        Cpu::fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0xA7] = Instruction::new(
        Mnemonic::And,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_and,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>,
    );
    instructions[0xA8] = Instruction::new(
        Mnemonic::Xor,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_xor,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0xA9] = Instruction::new(
        Mnemonic::Xor,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_xor,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0xAA] = Instruction::new(
        Mnemonic::Xor,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_xor,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0xAB] = Instruction::new(
        Mnemonic::Xor,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_xor,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0xAC] = Instruction::new(
        Mnemonic::Xor,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_xor,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0xAD] = Instruction::new(
        Mnemonic::Xor,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_xor,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0xAE] = Instruction::new(
        Mnemonic::Xor,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_xor,
        Cpu::fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0xAF] = Instruction::new(
        Mnemonic::Xor,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_xor,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>,
    );

    // 0xBX
    instructions[0xB0] = Instruction::new(
        Mnemonic::Or,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_or,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0xB1] = Instruction::new(
        Mnemonic::Or,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_or,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0xB2] = Instruction::new(
        Mnemonic::Or,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_or,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0xB3] = Instruction::new(
        Mnemonic::Or,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_or,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0xB4] = Instruction::new(
        Mnemonic::Or,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_or,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0xB5] = Instruction::new(
        Mnemonic::Or,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_or,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0xB6] = Instruction::new(
        Mnemonic::Or,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_or,
        Cpu::fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0xB7] = Instruction::new(
        Mnemonic::Or,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_or,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>,
    );
    instructions[0xB8] = Instruction::new(
        Mnemonic::Cp,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::B)),
        Cpu::execute_cp,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>,
    );
    instructions[0xB9] = Instruction::new(
        Mnemonic::Cp,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::C)),
        Cpu::execute_cp,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>,
    );
    instructions[0xBA] = Instruction::new(
        Mnemonic::Cp,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::D)),
        Cpu::execute_cp,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>,
    );
    instructions[0xBB] = Instruction::new(
        Mnemonic::Cp,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::E)),
        Cpu::execute_cp,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>,
    );
    instructions[0xBC] = Instruction::new(
        Mnemonic::Cp,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::H)),
        Cpu::execute_cp,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>,
    );
    instructions[0xBD] = Instruction::new(
        Mnemonic::Cp,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::L)),
        Cpu::execute_cp,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>,
    );
    instructions[0xBE] = Instruction::new(
        Mnemonic::Cp,
        InstructionSpec::default(AddressMode::R_MR(RegisterType::A, RegisterType::HL)),
        Cpu::execute_cp,
        Cpu::fetch_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0xBF] = Instruction::new(
        Mnemonic::Cp,
        InstructionSpec::default(AddressMode::R_R(RegisterType::A, RegisterType::A)),
        Cpu::execute_cp,
        Cpu::fetch_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>,
    );

    // 0xCX
    instructions[0xC0] = Instruction::new(
        Mnemonic::Ret,
        InstructionSpec::new(Some(ConditionType::NZ), 0, AddressMode::IMP),
        Cpu::execute_ret,
        Cpu::fetch_impl,
    );
    instructions[0xC1] = Instruction::new(
        Mnemonic::Pop,
        InstructionSpec::default(AddressMode::R(RegisterType::BC)),
        Cpu::execute_pop,
        Cpu::fetch_r::<{RegisterType::BC as u8}>,
    );
    instructions[0xC2] = Instruction::new(
        Mnemonic::Jp,
        InstructionSpec::new(Some(ConditionType::NZ), 0, AddressMode::D16),
        Cpu::execute_jp,
        Cpu::fetch_d16,
    );
    instructions[0xC3] = Instruction::new(
        Mnemonic::Jp,
        InstructionSpec::default(AddressMode::D16),
        Cpu::execute_jp,
        Cpu::fetch_d16,
    );
    instructions[0xC5] = Instruction::new(
        Mnemonic::Push,
        InstructionSpec::default(AddressMode::R(RegisterType::BC)),
        Cpu::execute_push,
        Cpu::fetch_r::<{RegisterType::BC as u8}>,
    );
    instructions[0xC6] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_add,
        Cpu::fetch_r_d8::<{RegisterType::A as u8}>,
    );
    instructions[0xC7] = Instruction::new(
        Mnemonic::Rst,
        InstructionSpec::new(None, 0x00, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );
    instructions[0xC4] = Instruction::new(
        Mnemonic::Call,
        InstructionSpec::new(Some(ConditionType::NZ), 0, AddressMode::D16),
        Cpu::execute_call,
        Cpu::fetch_d16,
    );
    instructions[0xC8] = Instruction::new(
        Mnemonic::Ret,
        InstructionSpec::new(Some(ConditionType::Z), 0, AddressMode::IMP),
        Cpu::execute_ret,
        Cpu::fetch_impl,
    );
    instructions[0xC9] = Instruction::new(
        Mnemonic::Ret,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_ret,
        Cpu::fetch_impl,
    );
    instructions[0xCC] = Instruction::new(
        Mnemonic::Call,
        InstructionSpec::new(Some(ConditionType::Z), 0, AddressMode::D16),
        Cpu::execute_call,
        Cpu::fetch_d16,
    );
    instructions[0xCD] = Instruction::new(
        Mnemonic::Call,
        InstructionSpec::default(AddressMode::D16),
        Cpu::execute_call,
        Cpu::fetch_d16,
    );
    instructions[0xCA] = Instruction::new(
        Mnemonic::Jp,
        InstructionSpec::new(Some(ConditionType::Z), 0, AddressMode::D16),
        Cpu::execute_jp,
        Cpu::fetch_d16,
    );
    instructions[0xCB] = Instruction::new(
        Mnemonic::Prefix,
        InstructionSpec::default(AddressMode::D8),
        Cpu::execute_prefix,
        Cpu::fetch_d8,
    );
    instructions[0xCE] = Instruction::new(
        Mnemonic::Adc,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_adc,
        Cpu::fetch_r_d8::<{RegisterType::A as u8}>,
    );
    instructions[0xCF] = Instruction::new(
        Mnemonic::Rst,
        InstructionSpec::new(None, 0x08, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );

    // 0xDX
    instructions[0xD0] = Instruction::new(
        Mnemonic::Ret,
        InstructionSpec::new(Some(ConditionType::NC), 0, AddressMode::IMP),
        Cpu::execute_ret,
        Cpu::fetch_impl,
    );
    instructions[0xD1] = Instruction::new(
        Mnemonic::Pop,
        InstructionSpec::default(AddressMode::R(RegisterType::DE)),
        Cpu::execute_pop,
        Cpu::fetch_r::<{RegisterType::DE as u8}>,
    );
    instructions[0xD2] = Instruction::new(
        Mnemonic::Jp,
        InstructionSpec::new(Some(ConditionType::NC), 0, AddressMode::D16),
        Cpu::execute_jp,
        Cpu::fetch_d16,
    );
    instructions[0xD4] = Instruction::new(
        Mnemonic::Call,
        InstructionSpec::new(Some(ConditionType::NC), 0, AddressMode::D16),
        Cpu::execute_call,
        Cpu::fetch_d16,
    );
    instructions[0xD5] = Instruction::new(
        Mnemonic::Push,
        InstructionSpec::default(AddressMode::R(RegisterType::DE)),
        Cpu::execute_push,
        Cpu::fetch_r::<{RegisterType::DE as u8}>,
    );
    instructions[0xD6] = Instruction::new(
        Mnemonic::Sub,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_sub,
        Cpu::fetch_r_d8::<{RegisterType::A as u8}>,
    );
    instructions[0xD7] = Instruction::new(
        Mnemonic::Rst,
        InstructionSpec::new(None, 0x10, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );
    instructions[0xD8] = Instruction::new(
        Mnemonic::Ret,
        InstructionSpec::new(Some(ConditionType::C), 0, AddressMode::IMP),
        Cpu::execute_ret,
        Cpu::fetch_impl,
    );
    instructions[0xD9] = Instruction::new(
        Mnemonic::Reti,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_reti,
        Cpu::fetch_impl,
    );
    instructions[0xDC] = Instruction::new(
        Mnemonic::Call,
        InstructionSpec::new(Some(ConditionType::C), 0, AddressMode::D16),
        Cpu::execute_call,
        Cpu::fetch_d16,
    );
    instructions[0xDA] = Instruction::new(
        Mnemonic::Jp,
        InstructionSpec::new(Some(ConditionType::C), 0, AddressMode::D16),
        Cpu::execute_jp,
        Cpu::fetch_d16,
    );
    instructions[0xDE] = Instruction::new(
        Mnemonic::Sbc,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_sbc,
        Cpu::fetch_r_d8::<{RegisterType::A as u8}>,
    );
    instructions[0xDF] = Instruction::new(
        Mnemonic::Rst,
        InstructionSpec::new(None, 0x18, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );

    // 0xEX
    instructions[0xE0] = Instruction::new(
        Mnemonic::Ldh,
        InstructionSpec::default(AddressMode::A8_R(RegisterType::A)),
        Cpu::execute_ldh,
        |cpu| cpu.fetch_a8_r(RegisterType::A),
    );
    instructions[0xE1] = Instruction::new(
        Mnemonic::Pop,
        InstructionSpec::default(AddressMode::R(RegisterType::HL)),
        Cpu::execute_pop,
        Cpu::fetch_r::<{RegisterType::HL as u8}>,
    );
    instructions[0xE2] = Instruction::new(
        Mnemonic::Ldh,
        InstructionSpec::default(AddressMode::MR_R(RegisterType::C, RegisterType::A)),
        Cpu::execute_ldh,
        Cpu::fetch_mr_r::<{ RegisterType::C as u8 }, { RegisterType::A as u8 }>,
    );
    instructions[0xE5] = Instruction::new(
        Mnemonic::Push,
        InstructionSpec::default(AddressMode::R(RegisterType::HL)),
        Cpu::execute_push,
        Cpu::fetch_r::<{RegisterType::HL as u8}>,
    );
    instructions[0xE6] = Instruction::new(
        Mnemonic::And,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_and,
        Cpu::fetch_r_d8::<{RegisterType::A as u8}>,
    );
    instructions[0xE7] = Instruction::new(
        Mnemonic::Rst,
        InstructionSpec::new(None, 0x20, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );
    instructions[0xE8] = Instruction::new(
        Mnemonic::Add,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::SP)),
        Cpu::execute_add,
        Cpu::fetch_r_d8::<{RegisterType::SP as u8}>,
    );
    instructions[0xEA] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::A16_R(RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_a16_r(RegisterType::A),
    );
    instructions[0xE9] = Instruction::new(
        Mnemonic::Jp,
        InstructionSpec::default(AddressMode::R(RegisterType::HL)),
        Cpu::execute_jp,
        Cpu::fetch_r::<{RegisterType::HL as u8}>,
    );
    instructions[0xEE] = Instruction::new(
        Mnemonic::Xor,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_xor,
        Cpu::fetch_r_d8::<{RegisterType::A as u8}>,
    );
    instructions[0xEF] = Instruction::new(
        Mnemonic::Rst,
        InstructionSpec::new(None, 0x28, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );

    // 0xFX
    instructions[0xF0] = Instruction::new(
        Mnemonic::Ldh,
        InstructionSpec::default(AddressMode::R_HA8(RegisterType::A)),
        Cpu::execute_ldh,
        |cpu| cpu.fetch_r_ha8(RegisterType::A),
    );
    instructions[0xF1] = Instruction::new(
        Mnemonic::Pop,
        InstructionSpec::default(AddressMode::R(RegisterType::AF)),
        Cpu::execute_pop,
        Cpu::fetch_r::<{RegisterType::AF as u8}>,
    );
    instructions[0xF2] = Instruction::new(
        Mnemonic::Ldh,
        InstructionSpec::default(AddressMode::R_HMR(RegisterType::A, RegisterType::C)),
        Cpu::execute_ldh,
        Cpu::fetch_r_hmr_a_c,
    );
    instructions[0xF3] = Instruction::new(
        Mnemonic::Di,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_di,
        Cpu::fetch_impl,
    );
    instructions[0xF5] = Instruction::new(
        Mnemonic::Push,
        InstructionSpec::default(AddressMode::R(RegisterType::AF)),
        Cpu::execute_push,
        Cpu::fetch_r::<{RegisterType::AF as u8}>,
    );
    instructions[0xF6] = Instruction::new(
        Mnemonic::Or,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_or,
        Cpu::fetch_r_d8::<{RegisterType::A as u8}>,
    );
    instructions[0xF7] = Instruction::new(
        Mnemonic::Rst,
        InstructionSpec::new(None, 0x30, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );
    instructions[0xF8] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::LH_SPi8),
        Cpu::execute_ld,
        Cpu::fetch_lh_spi8,
    );
    instructions[0xF9] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_R(RegisterType::SP, RegisterType::HL)),
        Cpu::execute_ld,
        Cpu::fetch_r_r::<{ RegisterType::SP as u8 }, { RegisterType::HL as u8 }>,
    );
    instructions[0xFA] = Instruction::new(
        Mnemonic::Ld,
        InstructionSpec::default(AddressMode::R_A16(RegisterType::A)),
        Cpu::execute_ld,
        |cpu| cpu.fetch_r_a16(RegisterType::A),
    );
    instructions[0xFB] = Instruction::new(
        Mnemonic::Ei,
        InstructionSpec::default(AddressMode::IMP),
        Cpu::execute_ei,
        Cpu::fetch_impl,
    );
    instructions[0xFE] = Instruction::new(
        Mnemonic::Cp,
        InstructionSpec::default(AddressMode::R_D8(RegisterType::A)),
        Cpu::execute_cp,
        Cpu::fetch_r_d8::<{RegisterType::A as u8}>,
    );
    instructions[0xFF] = Instruction::new(
        Mnemonic::Rst,
        InstructionSpec::new(None, 0x38, AddressMode::IMP),
        Cpu::execute_rst,
        Cpu::fetch_impl,
    );

    instructions
};
