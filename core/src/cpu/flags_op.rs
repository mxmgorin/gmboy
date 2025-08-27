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
        let h = ((lhs & 0xF) + (rhs & 0xF) + carry) > 0xF;
        let c = (lhs as u16 + rhs as u16 + carry as u16) > 0xFF;
        flags.set_byte_raw(pack_flags(result == 0, false, h, c));
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
        let h = ((lhs & 0xF) + (rhs & 0xF)) > 0xF;
        let c = ((lhs & 0xFF) + (rhs & 0xFF)) > 0xFF;
        flags.set_byte_raw(pack_flags(false, false, h, c));
    }

    #[inline(always)]
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
    #[inline(always)]
    pub fn sub8(flags: &mut Flags, lhs: u8, rhs: u8, result: u8, carry: u8) {
        let h = (lhs & 0xF) < ((rhs & 0xF) + carry);
        let c = (lhs as u16) < rhs as u16 + carry as u16;
        flags.set_byte_raw(pack_flags(result == 0, true, h, c));
    }

    /// Z=depends on Result, N=0, H=1, C=0
    #[inline(always)]
    pub fn and(flags: &mut Flags, result: u8) {
        flags.set_byte_raw(pack_flags(result == 0, false, true, false));
    }

    #[inline(always)]
    pub fn cpl(flags: &mut Flags) {
        flags.set_n_raw(true);
        flags.set_h_raw(true);
    }

    /// Z=depends on Result, N=0, H=0, C=0
    #[inline(always)]
    pub fn or(flags: &mut Flags, result: u8) {
        flags.set_byte_raw(pack_flags(result == 0, false, false, false));
    }

    /// Z=0, N=0, H=depends on lhs and rhs, C=depends on lhs and rhs
    #[inline(always)]
    pub fn ld(flags: &mut Flags, lhs: u16, rhs: u16) {
        let h = ((lhs & 0xF) + (rhs & 0xF)) >= 0x10;
        let c = ((lhs & 0xFF) + (rhs & 0xFF)) >= 0x100;
        flags.set_byte_raw(pack_flags(false, false, h, c));
    }

    #[inline(always)]
    pub fn ccf(flags: &mut Flags, carry: u8) {
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(carry == 0);
    }

    #[inline(always)]
    pub fn scf(flags: &mut Flags) {
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(true);
    }

    #[inline(always)]
    pub fn rla(flags: &mut Flags, lhs: u8) {
        let c = ((lhs >> 7) & 1) != 0;
        flags.set_byte_raw(pack_flags(false, false, false, c));
    }

    /// Z=0, N=0, H=0, C=depends on carry
    #[inline(always)]
    pub fn rlca(flags: &mut Flags, carry: u8) {
        // C flag comes directly from carry
        let c = carry != 0;
        flags.set_byte_raw(pack_flags(false, false, false, c));
    }

    /// Z=0, N=0, H=0, C=depends on lhs
    #[inline(always)]
    pub fn rra(flags: &mut Flags, lhs: u8) {
        // C flag comes from bit 0 of lhs
        let c = lhs & 1;
        flags.set_byte_raw(pack_flags(false, false, false, c != 0));
    }
}

#[inline(always)]
const fn pack_flags(z: bool, n: bool, h: bool, c: bool) -> u8 {
    ((z as u8) << ZERO_FLAG_BYTE_POSITION)
        | ((n as u8) << NEGATIVE_FLAG_BYTE_POSITION)
        | ((h as u8) << HALF_CARRY_FLAG_BYTE_POSITION)
        | ((c as u8) << CARRY_FLAG_BYTE_POSITION)
}
