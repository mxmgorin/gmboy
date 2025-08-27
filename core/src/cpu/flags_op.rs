use crate::cpu::flags::{
    Flags, CARRY_FLAG_BYTE_POSITION, HALF_CARRY_FLAG_BYTE_POSITION, NEGATIVE_FLAG_BYTE_POSITION,
    ZERO_FLAG_BYTE_POSITION,
};
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub enum FlagsOp {
    Add8 = 0,
    Add16 = 1,
    AddSpE8 = 2,
    Sub8 = 3,
    Inc8 = 4,
    Dec8 = 5,
    Rla = 6,
    And = 7,
    Cpl = 8,
    Or = 9,
    Rlca = 10,
    Rra = 11,
    Ccf = 12,
    Scf = 13,
    Ld = 14,
}

impl FlagsOp {
    /// Z=depends on Result, N=0, H=depends on lhs and rhs, C=depends on lhs and rhs
    #[inline(always)]
    pub fn add8(flags: &mut Flags, lhs: u8, rhs: u8, result: u8, carry: u8) {
        // Z flag (bit 7)
        let z = (result == 0) as u8;
        // H flag (bit 5): lower nibble carry including carry-in
        let h = (((lhs & 0xF) + (rhs & 0xF) + carry) > 0xF) as u8;
        // C flag (bit 4): full-byte carry including carry-in
        let c = ((lhs as u16 + rhs as u16 + carry as u16) > 0xFF) as u8;
        // Pack all flags: Z=bit7, N=0, H=bit5, C=bit4
        let f = (z << ZERO_FLAG_BYTE_POSITION)
            | (h << HALF_CARRY_FLAG_BYTE_POSITION)
            | (c << CARRY_FLAG_BYTE_POSITION);
        flags.set_byte_raw(f);
    }

    #[inline(always)]
    pub fn add16(flags: &mut Flags, lhs: u16, rhs: u16) {
        flags.set_n_raw(false);
        flags.set_h_raw(((lhs & 0x0FFF) + (rhs & 0x0FFF)) > 0x0FFF);
        flags.set_c_raw((lhs as u32 + rhs as u32) > 0xFFFF);
    }

    /// Z=0, N=0, H=depends on lhs and rhs, C=depends on lhs and rhs
    #[inline(always)]
    pub fn add_sp_e8(flags: &mut Flags, lhs: u16, rhs: u16) {
        // Half-carry (bit 5)
        let h = (((lhs & 0xF) + (rhs & 0xF)) > 0xF) as u8;
        // Carry (bit 4)
        let c = (((lhs & 0xFF) + (rhs & 0xFF)) > 0xFF) as u8;
        // Pack flags: Z=0, N=0, H and C only
        let f = (h << HALF_CARRY_FLAG_BYTE_POSITION) | (c << CARRY_FLAG_BYTE_POSITION);
        flags.set_byte_raw(f);
    }

    pub fn dec8(flags: &mut Flags, lhs: u8, result: u8) {
        flags.set_z_raw(result == 0);
        flags.set_n_raw(true);
        flags.set_h_raw((lhs & 0xF) == 0);
    }

    #[inline(always)]
    pub fn inc8(flags: &mut Flags, lhs: u8, result: u8) {
        flags.set_z_raw(result == 0);
        flags.set_n_raw(false);
        flags.set_h_raw((lhs & 0xF) + 1 > 0xF);
    }

    /// Z=depends on Result, N=1, H=depends on lhs, rhs, carry, C=depends on lhs, rhs, carry
    pub fn sub8(flags: &mut Flags, lhs: u8, rhs: u8, result: u8, carry: u8) {
        // Z flag (bit 7)
        let z = (result == 0) as u8;
        // H flag (bit 5): borrow from lower nibble
        let h = ((lhs & 0xF) < ((rhs & 0xF) + carry)) as u8;
        // C flag (bit 4): borrow from full byte
        let c = ((lhs as u16) < (rhs as u16) + (carry as u16)) as u8;
        // Pack flags: Z=bit7, N=1, H=bit5, C=bit4
        let f = (z << ZERO_FLAG_BYTE_POSITION)
            | (1 << NEGATIVE_FLAG_BYTE_POSITION)
            | (h << HALF_CARRY_FLAG_BYTE_POSITION)
            | (c << CARRY_FLAG_BYTE_POSITION);
        flags.set_byte_raw(f);
    }

    /// Z=depends on Result, N=0, H=1, C=0
    #[inline(always)]
    pub fn and(flags: &mut Flags, result: u8) {
        // Z (bit 7), H (bit 5)
        let z = (result == 0) as u8;
        let f = (z << ZERO_FLAG_BYTE_POSITION) | 0x20;
        flags.set_byte_raw(f);
    }

    pub fn cpl(flags: &mut Flags) {
        flags.set_n_raw(true);
        flags.set_h_raw(true);
    }

    /// Z=depends on Result, N=0, H=0, C=0
    pub fn or(flags: &mut Flags, result: u8) {
        // Z flag is bit 7, others are 0
        let z = (result == 0) as u8;
        let f = z << ZERO_FLAG_BYTE_POSITION; // only Z is conditionally set
        flags.set_byte_raw(f);
    }

    /// Z=0, N=0, H=depends on lhs and rhs, C=depends on lhs and rhs
    #[inline(always)]
    pub fn ld(flags: &mut Flags, lhs: u16, rhs: u16) {
        // Half-carry (bit 5)
        let h = (((lhs & 0xF) + (rhs & 0xF)) >= 0x10) as u8;
        // Carry (bit 4)
        let c = (((lhs & 0xFF) + (rhs & 0xFF)) >= 0x100) as u8;
        // Z=0, N=0, pack H and C
        let f = (h << HALF_CARRY_FLAG_BYTE_POSITION) | (c << CARRY_FLAG_BYTE_POSITION);
        flags.set_byte_raw(f);
    }

    #[inline(always)]
    pub fn ccf(flags: &mut Flags, carry: u8) {
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(carry == 0);
    }

    pub fn scf(flags: &mut Flags) {
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(true);
    }

    #[inline(always)]
    pub fn rla(flags: &mut Flags, lhs: u8) {
        // Carry comes from bit 7 of lhs
        let c = (lhs >> 7) & 1;
        // Only C (bit 4) can be set
        let f = c << CARRY_FLAG_BYTE_POSITION;
        flags.set_byte_raw(f);
    }

    /// Z=0, N=0, H=0, C=depends on carry
    #[inline(always)]
    pub fn rlca(flags: &mut Flags, carry: u8) {
        // C flag comes directly from carry
        let c = (carry != 0) as u8;
        // Only bit 4 (C) can be set
        let f = c << CARRY_FLAG_BYTE_POSITION;
        flags.set_byte_raw(f);
    }

    /// Z=0, N=0, H=0, C=depends on lhs
    #[inline(always)]
    pub fn rra(flags: &mut Flags, lhs: u8) {
        // C flag comes from bit 0 of lhs
        let c = lhs & 1;
        // Only bit 4 (C) can be set
        let f = c << CARRY_FLAG_BYTE_POSITION;
        flags.set_byte_raw(f);
    }
}
