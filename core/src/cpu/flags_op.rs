use crate::cpu::flags::{
    Flags, CARRY_FLAG_BYTE_POSITION, HALF_CARRY_FLAG_BYTE_POSITION, NEGATIVE_FLAG_BYTE_POSITION,
    ZERO_FLAG_BYTE_POSITION,
};
use serde::{Deserialize, Serialize};

type ComputeFlagsFn = fn(FlagsData, &mut Flags);

const COMPUTE_FNS: [ComputeFlagsFn; 16] = {
    let mut table = [FlagsOp::nop as ComputeFlagsFn; 16];
    table[FlagsOp::Add8 as usize] = FlagsData::op_add8;
    table[FlagsOp::Add16 as usize] = FlagsData::op_add16;
    table[FlagsOp::AddSpE8 as usize] = FlagsData::op_add_sp_e8;
    table[FlagsOp::Sub8 as usize] = FlagsData::op_sub8;
    table[FlagsOp::Inc8 as usize] = FlagsData::op_inc8;
    table[FlagsOp::Dec8 as usize] = FlagsData::op_dec8;
    table[FlagsOp::Rla as usize] = FlagsData::op_rla;
    table[FlagsOp::And as usize] = FlagsData::op_and;
    table[FlagsOp::Cpl as usize] = FlagsData::op_cpl;
    table[FlagsOp::Or as usize] = FlagsData::op_or;
    table[FlagsOp::Rlca as usize] = FlagsData::op_rlca;
    table[FlagsOp::Rra as usize] = FlagsData::op_rra;
    table[FlagsOp::Ccf as usize] = FlagsData::op_ccf;
    table[FlagsOp::Scf as usize] = FlagsData::op_scf;
    table[FlagsOp::Ld as usize] = FlagsData::op_ld;
    table[FlagsOp::Nop as usize] = FlagsOp::nop;

    table
};

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
    Nop = 15,
}

impl FlagsOp {
    #[inline(always)]
    pub fn nop(_: FlagsData, _: &mut Flags) {}

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
        let c = ((lhs >> 7) & 1) as u8;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagsCtx {
    op: FlagsOp,
    data: FlagsData,
}

impl Default for FlagsCtx {
    fn default() -> Self {
        Self {
            op: FlagsOp::Nop,
            data: Default::default(),
        }
    }
}

impl FlagsCtx {
    #[inline(always)]
    pub const fn clear(&mut self) {
        self.op = FlagsOp::Nop;
    }

    #[inline(always)]
    pub fn compute(self, flags: &mut Flags) {
        let index = self.op as usize;

        unsafe {
            COMPUTE_FNS.get_unchecked(index)(self.data, flags);
        }
    }

    #[inline(always)]
    pub fn new_add8(lhs: u8, rhs: u8, carry: u8, result: u8) -> Self {
        Self {
            op: FlagsOp::Add8,
            data: FlagsData::new(lhs as u16, rhs as u16, carry, result),
        }
    }

    #[inline(always)]
    pub fn new_add16(lhs: u16, rhs: u16) -> Self {
        Self {
            op: FlagsOp::Add16,
            data: FlagsData::with_lhs_rhs(lhs, rhs),
        }
    }

    #[inline(always)]
    pub fn new_add_sp_e8(lhs: u16, rhs: u16) -> Self {
        Self {
            op: FlagsOp::AddSpE8,
            data: FlagsData::with_lhs_rhs(lhs, rhs),
        }
    }

    #[inline(always)]
    pub fn new_sub8(lhs: u8, rhs: u8, carry: u8, result: u8) -> Self {
        Self {
            op: FlagsOp::Sub8,
            data: FlagsData::new(lhs as u16, rhs as u16, carry, result),
        }
    }

    #[inline(always)]
    pub fn new_dec8(lhs: u8, result: u8) -> Self {
        Self {
            op: FlagsOp::Dec8,
            data: FlagsData::with_lhs_result(lhs as u16, result),
        }
    }

    #[inline(always)]
    pub fn new_inc8(lhs: u8, result: u8) -> Self {
        Self {
            op: FlagsOp::Inc8,
            data: FlagsData::with_lhs_result(lhs as u16, result),
        }
    }

    #[inline(always)]
    pub fn new_and(result: u8) -> Self {
        Self {
            op: FlagsOp::And,
            data: FlagsData::with_result(result),
        }
    }

    #[inline(always)]
    pub fn new_cpl() -> Self {
        Self {
            op: FlagsOp::Cpl,
            data: FlagsData::default(),
        }
    }

    #[inline(always)]
    pub fn new_or(result: u8) -> Self {
        Self {
            op: FlagsOp::Or,
            data: FlagsData::with_result(result),
        }
    }

    #[inline(always)]
    pub fn new_rla(lhs: u8) -> Self {
        Self {
            op: FlagsOp::Rla,
            data: FlagsData::with_lhs(lhs as u16),
        }
    }

    #[inline(always)]
    pub fn new_rlca(carry: u8) -> Self {
        Self {
            op: FlagsOp::Rlca,
            data: FlagsData::with_carry(carry),
        }
    }

    #[inline(always)]
    pub fn new_rra(lhs: u8) -> Self {
        Self {
            op: FlagsOp::Rra,
            data: FlagsData::with_lhs(lhs as u16),
        }
    }

    #[inline(always)]
    pub fn new_ccf(carry: u8) -> Self {
        Self {
            op: FlagsOp::Ccf,
            data: FlagsData::with_carry(carry),
        }
    }

    #[inline(always)]
    pub fn new_scf() -> Self {
        Self {
            op: FlagsOp::Scf,
            data: FlagsData::default(),
        }
    }

    #[inline(always)]
    pub fn new_ld(lhs: u16, rhs: u16) -> Self {
        Self {
            op: FlagsOp::Ld,
            data: FlagsData::with_lhs_rhs(lhs, rhs),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Copy)]
pub struct FlagsData {
    pub lhs: u16,
    pub rhs: u16,
    pub carry: u8,
    pub result: u8,
}

impl FlagsData {
    #[inline(always)]
    pub fn op_add8(data: FlagsData, flags: &mut Flags) {
        FlagsOp::add8(
            flags,
            data.lhs as u8,
            data.rhs as u8,
            data.result,
            data.carry,
        );
    }

    #[inline(always)]
    pub fn op_add16(data: FlagsData, flags: &mut Flags) {
        FlagsOp::add16(flags, data.lhs, data.rhs);
    }

    #[inline(always)]
    pub fn op_add_sp_e8(data: FlagsData, flags: &mut Flags) {
        FlagsOp::add_sp_e8(flags, data.lhs, data.rhs);
    }

    #[inline(always)]
    pub fn op_sub8(data: FlagsData, flags: &mut Flags) {
        FlagsOp::sub8(
            flags,
            data.lhs as u8,
            data.rhs as u8,
            data.result,
            data.carry,
        );
    }

    #[inline(always)]
    pub fn op_inc8(data: FlagsData, flags: &mut Flags) {
        FlagsOp::inc8(flags, data.lhs as u8, data.result);
    }

    #[inline(always)]
    pub fn op_dec8(data: FlagsData, flags: &mut Flags) {
        FlagsOp::dec8(flags, data.lhs as u8, data.result);
    }

    #[inline(always)]
    pub fn op_rla(data: FlagsData, flags: &mut Flags) {
        FlagsOp::rla(flags, data.lhs as u8);
    }

    #[inline(always)]
    pub fn op_and(data: FlagsData, flags: &mut Flags) {
        FlagsOp::and(flags, data.result);
    }

    #[inline(always)]
    pub fn op_cpl(_: FlagsData, flags: &mut Flags) {
        FlagsOp::cpl(flags);
    }

    #[inline(always)]
    pub fn op_or(data: FlagsData, flags: &mut Flags) {
        FlagsOp::or(flags, data.result);
    }

    #[inline(always)]
    pub fn op_rlca(data: FlagsData, flags: &mut Flags) {
        FlagsOp::rlca(flags, data.carry);
    }

    #[inline(always)]
    pub fn op_rra(data: FlagsData, flags: &mut Flags) {
        FlagsOp::rra(flags, data.lhs as u8);
    }

    #[inline(always)]
    pub fn op_ccf(data: FlagsData, flags: &mut Flags) {
        FlagsOp::ccf(flags, data.carry);
    }

    #[inline(always)]
    pub fn op_scf(_: FlagsData, flags: &mut Flags) {
        FlagsOp::scf(flags);
    }

    #[inline(always)]
    pub fn op_ld(data: FlagsData, flags: &mut Flags) {
        FlagsOp::ld(flags, data.lhs, data.rhs);
    }

    #[inline(always)]
    pub fn new(lhs: u16, rhs: u16, carry: u8, result: u8) -> Self {
        Self {
            lhs,
            rhs,
            carry,
            result,
        }
    }

    #[inline(always)]
    pub fn with_lhs_rhs(lhs: u16, rhs: u16) -> Self {
        Self {
            lhs,
            rhs,
            carry: 0,
            result: 0,
        }
    }

    #[inline(always)]
    pub fn with_lhs_result(lhs: u16, result: u8) -> Self {
        Self {
            lhs,
            rhs: 0,
            carry: 0,
            result,
        }
    }

    #[inline(always)]
    pub fn with_carry(carry: u8) -> Self {
        Self {
            lhs: 0,
            rhs: 0,
            carry,
            result: 0,
        }
    }

    #[inline(always)]
    pub fn with_lhs(lhs: u16) -> Self {
        Self {
            lhs,
            rhs: 0,
            carry: 0,
            result: 0,
        }
    }

    #[inline(always)]
    pub fn with_result(result: u8) -> Self {
        Self {
            lhs: 0,
            rhs: 0,
            carry: 0,
            result,
        }
    }
}
