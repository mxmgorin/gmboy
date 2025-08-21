use crate::cpu::instructions::*;
use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn execute_opcode(&mut self) {
        let opcode = self.step_ctx.opcode;
        // SAFETY: array len is 256 and u8 can't be more than 256
        let execute_fn = unsafe { EXECUTE_FNS.get_unchecked(opcode as usize) };
        execute_fn(self);
    }

    fn unknown_opcode(&mut self) {
        panic!("can't execute unknown opcode {}", self.step_ctx.opcode);
    }
}

pub const EXECUTE_FNS: [fn(&mut Cpu); INSTRUCTIONS_COUNT] = {
    let mut array = { [Cpu::unknown_opcode as fn(&mut Cpu); INSTRUCTIONS_COUNT] };

    // 0x0X
    array[0x00] = Cpu::execute_nop;
    array[0x01] = Cpu::fetch_execute_ld_r_d16::<{ RegisterType::BC as u8 }>;
    array[0x02] =
        Cpu::fetch_execute_ld_mr_r::<{ RegisterType::BC as u8 }, { RegisterType::A as u8 }>;
    array[0x03] = Cpu::fetch_execute_inc_r::<{ RegisterType::BC as u8 }>;
    array[0x04] = Cpu::fetch_execute_inc_r::<{ RegisterType::B as u8 }>;
    array[0x05] = Cpu::fetch_execute_dec_r::<{ RegisterType::B as u8 }>;
    array[0x06] = Cpu::fetch_execute_ld_r_d8::<{ RegisterType::B as u8 }>;
    array[0x07] = Cpu::execute_rlca;
    array[0x08] = Cpu::fetch_execute_ld_a16_r::<{ RegisterType::SP as u8 }>;
    array[0x09] =
        Cpu::fetch_and_execute_add_r_r::<{ RegisterType::HL as u8 }, { RegisterType::BC as u8 }>;
    array[0x0A] =
        Cpu::fetch_execute_ld_r_mr::<{ RegisterType::A as u8 }, { RegisterType::BC as u8 }>;
    array[0x0B] = Cpu::fetch_execute_dec_r::<{ RegisterType::BC as u8 }>;
    array[0x0C] = Cpu::fetch_execute_inc_r::<{ RegisterType::C as u8 }>;
    array[0x0D] = Cpu::fetch_execute_dec_r::<{ RegisterType::C as u8 }>;
    array[0x0E] = Cpu::fetch_execute_ld_r_d8::<{ RegisterType::C as u8 }>;
    array[0x0F] = Cpu::execute_rrca;

    // 0x1X
    array[0x10] = Cpu::execute_stop;
    array[0x11] = Cpu::fetch_execute_ld_r_d16::<{ RegisterType::DE as u8 }>;
    array[0x12] =
        Cpu::fetch_execute_ld_mr_r::<{ RegisterType::DE as u8 }, { RegisterType::A as u8 }>;
    array[0x13] = Cpu::fetch_execute_inc_r::<{ RegisterType::DE as u8 }>;
    array[0x14] = Cpu::fetch_execute_inc_r::<{ RegisterType::D as u8 }>;
    array[0x15] = Cpu::fetch_execute_dec_r::<{ RegisterType::D as u8 }>;
    array[0x16] = Cpu::fetch_execute_ld_r_d8::<{ RegisterType::D as u8 }>;
    array[0x17] = Cpu::execute_rla;
    array[0x18] = Cpu::fetch_execute_jr_d8::<{ JumpCondition::None as u8 }>;
    array[0x19] =
        Cpu::fetch_and_execute_add_r_r::<{ RegisterType::HL as u8 }, { RegisterType::DE as u8 }>;
    array[0x1A] =
        Cpu::fetch_execute_ld_r_mr::<{ RegisterType::A as u8 }, { RegisterType::DE as u8 }>;
    array[0x1B] = Cpu::fetch_execute_dec_r::<{ RegisterType::DE as u8 }>;
    array[0x1C] = Cpu::fetch_execute_inc_r::<{ RegisterType::E as u8 }>;
    array[0x1D] = Cpu::fetch_execute_dec_r::<{ RegisterType::E as u8 }>;
    array[0x1E] = Cpu::fetch_execute_ld_r_d8::<{ RegisterType::E as u8 }>;
    array[0x1F] = Cpu::execute_rra;

    // 0x2X
    array[0x20] = Cpu::fetch_execute_jr_d8::<{ JumpCondition::NZ as u8 }>;
    array[0x21] = Cpu::fetch_execute_ld_r_d16::<{ RegisterType::HL as u8 }>;
    array[0x22] =
        Cpu::fetch_execute_ld_mri_r::<{ RegisterType::HL as u8 }, { RegisterType::A as u8 }>;
    array[0x23] = Cpu::fetch_execute_inc_r::<{ RegisterType::HL as u8 }>;
    array[0x24] = Cpu::fetch_execute_inc_r::<{ RegisterType::H as u8 }>;
    array[0x25] = Cpu::fetch_execute_dec_r::<{ RegisterType::H as u8 }>;
    array[0x26] = Cpu::fetch_execute_ld_r_d8::<{ RegisterType::H as u8 }>;
    array[0x27] = Cpu::execute_daa;
    array[0x28] = Cpu::fetch_execute_jr_d8::<{ JumpCondition::Z as u8 }>;
    array[0x29] =
        Cpu::fetch_and_execute_add_r_r::<{ RegisterType::HL as u8 }, { RegisterType::HL as u8 }>;
    array[0x2A] =
        Cpu::fetch_execute_ld_r_mri::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>;
    array[0x2B] = Cpu::fetch_execute_dec_r::<{ RegisterType::HL as u8 }>;
    array[0x2C] = Cpu::fetch_execute_inc_r::<{ RegisterType::L as u8 }>;
    array[0x2D] = Cpu::fetch_execute_dec_r::<{ RegisterType::L as u8 }>;
    array[0x2E] = Cpu::fetch_execute_ld_r_d8::<{ RegisterType::L as u8 }>;
    array[0x2F] = Cpu::execute_cpl;

    // 0x3X
    array[0x30] = Cpu::fetch_execute_jr_d8::<{ JumpCondition::NC as u8 }>;
    array[0x31] = Cpu::fetch_execute_ld_r_d16::<{ RegisterType::SP as u8 }>;
    array[0x32] =
        Cpu::fetch_execute_ld_mrd_r::<{ RegisterType::HL as u8 }, { RegisterType::A as u8 }>;
    array[0x33] = Cpu::fetch_execute_inc_r::<{ RegisterType::SP as u8 }>;
    array[0x34] = Cpu::fetch_execute_inc_mr_hl;
    array[0x35] = Cpu::fetch_execute_dec_mr_hl;
    array[0x36] = Cpu::fetch_execute_ld_mr_d8::<{ RegisterType::HL as u8 }>;
    array[0x37] = Cpu::execute_scf;
    array[0x38] = Cpu::fetch_execute_jr_d8::<{ JumpCondition::C as u8 }>;
    array[0x39] =
        Cpu::fetch_and_execute_add_r_r::<{ RegisterType::HL as u8 }, { RegisterType::SP as u8 }>;
    array[0x3A] =
        Cpu::fetch_execute_ld_r_mrd::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>;
    array[0x3B] = Cpu::fetch_execute_dec_r::<{ RegisterType::SP as u8 }>;
    array[0x3C] = Cpu::fetch_execute_inc_r::<{ RegisterType::A as u8 }>;
    array[0x3D] = Cpu::fetch_execute_dec_r::<{ RegisterType::A as u8 }>;
    array[0x3E] = Cpu::fetch_execute_ld_r_d8::<{ RegisterType::A as u8 }>;
    array[0x3F] = Cpu::execute_ccf;

    // 0x4X
    array[0x40] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::B as u8 }, { RegisterType::B as u8 }>;
    array[0x41] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::B as u8 }, { RegisterType::C as u8 }>;
    array[0x42] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::B as u8 }, { RegisterType::D as u8 }>;
    array[0x43] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::B as u8 }, { RegisterType::E as u8 }>;
    array[0x44] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::B as u8 }, { RegisterType::H as u8 }>;
    array[0x45] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::B as u8 }, { RegisterType::L as u8 }>;
    array[0x46] =
        Cpu::fetch_execute_ld_r_mr::<{ RegisterType::B as u8 }, { RegisterType::HL as u8 }>;
    array[0x47] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::B as u8 }, { RegisterType::A as u8 }>;
    array[0x48] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::C as u8 }, { RegisterType::B as u8 }>;
    array[0x49] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::C as u8 }, { RegisterType::C as u8 }>;
    array[0x4A] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::C as u8 }, { RegisterType::D as u8 }>;
    array[0x4B] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::C as u8 }, { RegisterType::E as u8 }>;
    array[0x4C] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::C as u8 }, { RegisterType::H as u8 }>;
    array[0x4D] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::C as u8 }, { RegisterType::L as u8 }>;
    array[0x4E] =
        Cpu::fetch_execute_ld_r_mr::<{ RegisterType::C as u8 }, { RegisterType::HL as u8 }>;
    array[0x4F] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::C as u8 }, { RegisterType::A as u8 }>;

    // 0x5X
    array[0x50] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::D as u8 }, { RegisterType::B as u8 }>;
    array[0x51] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::D as u8 }, { RegisterType::C as u8 }>;
    array[0x52] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::D as u8 }, { RegisterType::D as u8 }>;
    array[0x53] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::D as u8 }, { RegisterType::E as u8 }>;
    array[0x54] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::D as u8 }, { RegisterType::H as u8 }>;
    array[0x55] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::D as u8 }, { RegisterType::L as u8 }>;
    array[0x56] =
        Cpu::fetch_execute_ld_r_mr::<{ RegisterType::D as u8 }, { RegisterType::HL as u8 }>;
    array[0x57] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::D as u8 }, { RegisterType::A as u8 }>;
    array[0x58] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::E as u8 }, { RegisterType::B as u8 }>;
    array[0x59] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::E as u8 }, { RegisterType::C as u8 }>;
    array[0x5A] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::E as u8 }, { RegisterType::D as u8 }>;
    array[0x5B] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::E as u8 }, { RegisterType::E as u8 }>;
    array[0x5C] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::E as u8 }, { RegisterType::H as u8 }>;
    array[0x5D] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::E as u8 }, { RegisterType::L as u8 }>;
    array[0x5E] =
        Cpu::fetch_execute_ld_r_mr::<{ RegisterType::E as u8 }, { RegisterType::HL as u8 }>;
    array[0x5F] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::E as u8 }, { RegisterType::A as u8 }>;

    // 0x6X
    array[0x60] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::H as u8 }, { RegisterType::B as u8 }>;
    array[0x61] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::H as u8 }, { RegisterType::C as u8 }>;
    array[0x62] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::H as u8 }, { RegisterType::D as u8 }>;
    array[0x63] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::H as u8 }, { RegisterType::E as u8 }>;
    array[0x64] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::H as u8 }, { RegisterType::H as u8 }>;
    array[0x65] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::H as u8 }, { RegisterType::L as u8 }>;
    array[0x66] =
        Cpu::fetch_execute_ld_r_mr::<{ RegisterType::H as u8 }, { RegisterType::HL as u8 }>;
    array[0x67] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::H as u8 }, { RegisterType::A as u8 }>;
    array[0x68] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::L as u8 }, { RegisterType::B as u8 }>;
    array[0x69] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::L as u8 }, { RegisterType::C as u8 }>;
    array[0x6A] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::L as u8 }, { RegisterType::D as u8 }>;
    array[0x6B] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::L as u8 }, { RegisterType::E as u8 }>;
    array[0x6C] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::L as u8 }, { RegisterType::H as u8 }>;
    array[0x6D] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::L as u8 }, { RegisterType::L as u8 }>;
    array[0x6E] =
        Cpu::fetch_execute_ld_r_mr::<{ RegisterType::L as u8 }, { RegisterType::HL as u8 }>;
    array[0x6F] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::L as u8 }, { RegisterType::A as u8 }>;

    // 0x7X
    array[0x76] = Cpu::execute_halt;
    array[0x70] =
        Cpu::fetch_execute_ld_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::B as u8 }>;
    array[0x71] =
        Cpu::fetch_execute_ld_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::C as u8 }>;
    array[0x72] =
        Cpu::fetch_execute_ld_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::D as u8 }>;
    array[0x73] =
        Cpu::fetch_execute_ld_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::E as u8 }>;
    array[0x74] =
        Cpu::fetch_execute_ld_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::H as u8 }>;
    array[0x75] =
        Cpu::fetch_execute_ld_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::L as u8 }>;
    array[0x77] =
        Cpu::fetch_execute_ld_mr_r::<{ RegisterType::HL as u8 }, { RegisterType::A as u8 }>;
    array[0x78] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>;
    array[0x79] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>;
    array[0x7A] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>;
    array[0x7B] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>;
    array[0x7C] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>;
    array[0x7D] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>;
    array[0x7E] =
        Cpu::fetch_execute_ld_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>;
    array[0x7F] = Cpu::fetch_execute_ld_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>;

    // 0x8X
    array[0x80] =
        Cpu::fetch_and_execute_add_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>;
    array[0x81] =
        Cpu::fetch_and_execute_add_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>;
    array[0x82] =
        Cpu::fetch_and_execute_add_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>;
    array[0x83] =
        Cpu::fetch_and_execute_add_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>;
    array[0x84] =
        Cpu::fetch_and_execute_add_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>;
    array[0x85] =
        Cpu::fetch_and_execute_add_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>;
    array[0x86] =
        Cpu::fetch_and_execute_add_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>;
    array[0x87] =
        Cpu::fetch_and_execute_add_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>;
    array[0x88] =
        Cpu::fetch_and_execute_adc_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>;
    array[0x89] =
        Cpu::fetch_and_execute_adc_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>;
    array[0x8A] =
        Cpu::fetch_and_execute_adc_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>;
    array[0x8B] =
        Cpu::fetch_and_execute_adc_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>;
    array[0x8C] =
        Cpu::fetch_and_execute_adc_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>;
    array[0x8D] =
        Cpu::fetch_and_execute_adc_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>;
    array[0x8E] = Cpu::fetch_and_execute_adc_r_mr;
    array[0x8F] =
        Cpu::fetch_and_execute_adc_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>;

    // 0x9X
    array[0x90] =
        Cpu::fetch_execute_sub_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>;
    array[0x91] =
        Cpu::fetch_execute_sub_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>;
    array[0x92] =
        Cpu::fetch_execute_sub_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>;
    array[0x93] =
        Cpu::fetch_execute_sub_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>;
    array[0x94] =
        Cpu::fetch_execute_sub_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>;
    array[0x95] =
        Cpu::fetch_execute_sub_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>;
    array[0x96] =
        Cpu::fetch_execute_sub_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>;
    array[0x97] =
        Cpu::fetch_execute_sub_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>;
    array[0x98] =
        Cpu::fetch_execute_sbc_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>;
    array[0x99] =
        Cpu::fetch_execute_sbc_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>;
    array[0x9A] =
        Cpu::fetch_execute_sbc_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>;
    array[0x9B] =
        Cpu::fetch_execute_sbc_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>;
    array[0x9C] =
        Cpu::fetch_execute_sbc_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>;
    array[0x9D] =
        Cpu::fetch_execute_sbc_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>;
    array[0x9E] =
        Cpu::fetch_execute_sbc_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>;
    array[0x9F] =
        Cpu::fetch_execute_sbc_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>;

    // 0xAX
    array[0xA0] =
        Cpu::fetch_execute_and_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>;
    array[0xA1] =
        Cpu::fetch_execute_and_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>;
    array[0xA2] =
        Cpu::fetch_execute_and_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>;
    array[0xA3] =
        Cpu::fetch_execute_and_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>;
    array[0xA4] =
        Cpu::fetch_execute_and_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>;
    array[0xA5] =
        Cpu::fetch_execute_and_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>;
    array[0xA6] =
        Cpu::fetch_execute_and_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>;
    array[0xA7] =
        Cpu::fetch_execute_and_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>;
    array[0xA8] =
        Cpu::fetch_execute_xor_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>;
    array[0xA9] =
        Cpu::fetch_execute_xor_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>;
    array[0xAA] =
        Cpu::fetch_execute_xor_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>;
    array[0xAB] =
        Cpu::fetch_execute_xor_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>;
    array[0xAC] =
        Cpu::fetch_execute_xor_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>;
    array[0xAD] =
        Cpu::fetch_execute_xor_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>;
    array[0xAE] =
        Cpu::fetch_execute_xor_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>;
    array[0xAF] =
        Cpu::fetch_execute_xor_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>;

    // 0xBX
    array[0xB0] = Cpu::fetch_execute_or_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>;
    array[0xB1] = Cpu::fetch_execute_or_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>;
    array[0xB2] = Cpu::fetch_execute_or_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>;
    array[0xB3] = Cpu::fetch_execute_or_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>;
    array[0xB4] = Cpu::fetch_execute_or_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>;
    array[0xB5] = Cpu::fetch_execute_or_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>;
    array[0xB6] =
        Cpu::fetch_execute_or_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>;
    array[0xB7] = Cpu::fetch_execute_or_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>;
    array[0xB8] = Cpu::fetch_execute_cp_r_r::<{ RegisterType::A as u8 }, { RegisterType::B as u8 }>;
    array[0xB9] = Cpu::fetch_execute_cp_r_r::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>;
    array[0xBA] = Cpu::fetch_execute_cp_r_r::<{ RegisterType::A as u8 }, { RegisterType::D as u8 }>;
    array[0xBB] = Cpu::fetch_execute_cp_r_r::<{ RegisterType::A as u8 }, { RegisterType::E as u8 }>;
    array[0xBC] = Cpu::fetch_execute_cp_r_r::<{ RegisterType::A as u8 }, { RegisterType::H as u8 }>;
    array[0xBD] = Cpu::fetch_execute_cp_r_r::<{ RegisterType::A as u8 }, { RegisterType::L as u8 }>;
    array[0xBE] =
        Cpu::fetch_execute_cp_r_mr::<{ RegisterType::A as u8 }, { RegisterType::HL as u8 }>;
    array[0xBF] = Cpu::fetch_execute_cp_r_r::<{ RegisterType::A as u8 }, { RegisterType::A as u8 }>;

    // 0xCX
    array[0xC0] = Cpu::execute_ret::<{ JumpCondition::NZ as u8 }>;
    array[0xC1] = Cpu::fetch_execute_pop::<{ RegisterType::BC as u8 }>;
    array[0xC2] = Cpu::fetch_execute_jp_d16::<{ JumpCondition::NZ as u8 }>;
    array[0xC3] = Cpu::fetch_execute_jp_d16::<{ JumpCondition::None as u8 }>;
    array[0xC5] = Cpu::fetch_execute_push::<{ RegisterType::BC as u8 }>;
    array[0xC6] = Cpu::fetch_and_execute_add_r_d8::<{ RegisterType::A as u8 }>;
    array[0xC7] = Cpu::execute_rst_0x00;
    array[0xC4] = Cpu::fetch_execute_call_d16::<{ JumpCondition::NZ as u8 }>;
    array[0xC8] = Cpu::execute_ret::<{ JumpCondition::Z as u8 }>;
    array[0xC9] = Cpu::execute_ret::<{ JumpCondition::None as u8 }>;
    array[0xCC] = Cpu::fetch_execute_call_d16::<{ JumpCondition::Z as u8 }>;
    array[0xCD] = Cpu::fetch_execute_call_d16::<{ JumpCondition::None as u8 }>;
    array[0xCA] = Cpu::fetch_execute_jp_d16::<{ JumpCondition::Z as u8 }>;
    array[0xCB] = Cpu::fetch_execute_prefix;
    array[0xCE] = Cpu::fetch_and_execute_adc_r_d8;
    array[0xCF] = Cpu::execute_rst_0x08;

    // 0xDX
    array[0xD0] = Cpu::execute_ret::<{ JumpCondition::NC as u8 }>;
    array[0xD1] = Cpu::fetch_execute_pop::<{ RegisterType::DE as u8 }>;
    array[0xD2] = Cpu::fetch_execute_jp_d16::<{ JumpCondition::NC as u8 }>;
    array[0xD4] = Cpu::fetch_execute_call_d16::<{ JumpCondition::NC as u8 }>;
    array[0xD5] = Cpu::fetch_execute_push::<{ RegisterType::DE as u8 }>;
    array[0xD6] = Cpu::fetch_execute_sub_r_d8::<{ RegisterType::A as u8 }>;
    array[0xD7] = Cpu::execute_rst_0x10;
    array[0xD8] = Cpu::execute_ret::<{ JumpCondition::C as u8 }>;
    array[0xD9] = Cpu::execute_reti;
    array[0xDC] = Cpu::fetch_execute_call_d16::<{ JumpCondition::C as u8 }>;
    array[0xDA] = Cpu::fetch_execute_jp_d16::<{ JumpCondition::C as u8 }>;
    array[0xDE] = Cpu::fetch_execute_sbc_r_d8::<{ RegisterType::A as u8 }>;
    array[0xDF] = Cpu::execute_rst_0x18;

    // 0xEX
    array[0xE0] = Cpu::fetch_execute_ldh_a8_r::<{ RegisterType::A as u8 }>;
    array[0xE1] = Cpu::fetch_execute_pop::<{ RegisterType::HL as u8 }>;
    array[0xE2] =
        Cpu::fetch_execute_ldh_mr_r::<{ RegisterType::C as u8 }, { RegisterType::A as u8 }>;
    array[0xE5] = Cpu::fetch_execute_push::<{ RegisterType::HL as u8 }>;
    array[0xE6] = Cpu::fetch_execute_and_r_d8::<{ RegisterType::A as u8 }>;
    array[0xE7] = Cpu::execute_rst_0x20;
    array[0xE8] = Cpu::fetch_and_execute_add_sp;
    array[0xEA] = Cpu::fetch_execute_ld_a16_r::<{ RegisterType::A as u8 }>;
    array[0xE9] = Cpu::execute_jp_no_hl;
    array[0xEE] = Cpu::fetch_execute_xor_r_d8::<{ RegisterType::A as u8 }>;
    array[0xEF] = Cpu::execute_rst_0x28;

    // 0xFX
    array[0xF0] = Cpu::fetch_execute_ldh_r_ha8::<{ RegisterType::A as u8 }>;
    array[0xF1] = Cpu::fetch_execute_pop::<{ RegisterType::AF as u8 }>;
    array[0xF2] =
        Cpu::fetch_execute_ldh_r_hmr::<{ RegisterType::A as u8 }, { RegisterType::C as u8 }>;
    array[0xF3] = Cpu::execute_di;
    array[0xF5] = Cpu::fetch_execute_push::<{ RegisterType::AF as u8 }>;
    array[0xF6] = Cpu::fetch_execute_or_r_d8::<{ RegisterType::A as u8 }>;
    array[0xF7] = Cpu::execute_rst_0x30;
    array[0xF8] = Cpu::fetch_execute_ld_lh_spi8;
    array[0xF9] =
        Cpu::fetch_execute_ld_r_r::<{ RegisterType::SP as u8 }, { RegisterType::HL as u8 }>;
    array[0xFA] = Cpu::fetch_execute_ld_r_a16::<{ RegisterType::A as u8 }>;
    array[0xFB] = Cpu::execute_ei;
    array[0xFE] = Cpu::fetch_execute_cp_r_d8::<{ RegisterType::A as u8 }>;
    array[0xFF] = Cpu::execute_rst_0x38;

    array
};
