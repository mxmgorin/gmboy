use crate::cpu::flags_op::{FlagsCtx, FlagsData, FlagsOp};
use crate::{get_bit_flag, set_bit};
use serde::{Deserialize, Serialize};

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const NEGATIVE_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flags {
    byte: u8,
    pending: FlagsCtx,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            byte: 0xB0,
            pending: FlagsCtx::default(),
        }
    }
}

impl Flags {
    pub fn new(byte: u8) -> Flags {
        Self {
            byte,
            pending: FlagsCtx::default(),
        }
    }

    #[inline(always)]
    pub fn get_byte(&mut self) -> u8 {
        self.compute_pending();
        self.byte
    }

    #[inline(always)]
    pub const fn set_byte(&mut self, byte: u8) {
        self.pending.clear();
        self.byte = byte;
    }

    #[inline(always)]
    pub fn get_z(&mut self) -> bool {
        self.compute_pending();
        get_bit_flag(self.byte, ZERO_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_n(&mut self) -> bool {
        self.compute_pending();
        get_bit_flag(self.byte, NEGATIVE_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_hnc(&mut self) -> (bool, bool, bool) {
        self.compute_pending();
        (
            get_bit_flag(self.byte, HALF_CARRY_FLAG_BYTE_POSITION),
            get_bit_flag(self.byte, NEGATIVE_FLAG_BYTE_POSITION),
            get_bit_flag(self.byte, CARRY_FLAG_BYTE_POSITION),
        )
    }

    #[inline(always)]
    pub fn set_zhc(&mut self, z: bool, h: bool, c: bool) {
        self.compute_pending();
        self.set_z_raw(z);
        self.set_h_raw(h);
        self.set_c_raw(c);
    }

    #[inline(always)]
    pub fn set_znhc(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.pending.clear();
        self.set_z_raw(z);
        self.set_n_raw(n);
        self.set_h_raw(h);
        self.set_c_raw(c);
    }

    #[inline(always)]
    pub fn set_znh(&mut self, z: bool, n: bool, h: bool) {
        self.compute_pending();
        self.set_z_raw(z);
        self.set_n_raw(n);
        self.set_h_raw(h);
    }

    #[inline(always)]
    pub fn get_c(&mut self) -> bool {
        self.compute_pending();
        get_bit_flag(self.byte, CARRY_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_h(&mut self) -> bool {
        self.compute_pending();
        get_bit_flag(self.byte, HALF_CARRY_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    fn compute_pending(&mut self) {
        let pending = std::mem::take(&mut self.pending);
        pending.compute(self);
    }

    #[inline(always)]
    pub fn set_z_raw(&mut self, v: bool) {
        set_bit(&mut self.byte, ZERO_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn set_n_raw(&mut self, v: bool) {
        set_bit(&mut self.byte, NEGATIVE_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn set_h_raw(&mut self, v: bool) {
        set_bit(&mut self.byte, HALF_CARRY_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn set_c_raw(&mut self, v: bool) {
        set_bit(&mut self.byte, CARRY_FLAG_BYTE_POSITION, v);
    }

    pub fn display(&mut self) -> String {
        [
            (self.get_z(), 'Z'),
            (self.get_n(), 'N'),
            (self.get_h(), 'H'),
            (self.get_c(), 'C'),
        ]
        .iter()
        .map(|&(flag, c)| if flag { c } else { '-' })
        .collect()
    }


    #[inline(always)]
    pub fn op_add8(&mut self, lhs: u8, rhs: u8, carry: u8, result: u8) {
        self.pending = FlagsCtx::new_add8(lhs, rhs, carry, result);
    }

    #[inline(always)]
    pub fn op_add16(&mut self, lhs: u16, rhs: u16) {
        self.pending = FlagsCtx::new_add16(lhs, rhs);
    }

    #[inline(always)]
    pub fn op_add_sp_e8(&mut self, lhs: u16, rhs: u16) {
        self.pending = FlagsCtx::new_add_sp_e8(lhs, rhs);
    }

    #[inline(always)]
    pub fn op_sub8(&mut self, lhs: u8, rhs: u8, carry: u8, result: u8) {
        self.pending = FlagsCtx::new_sub8(lhs, rhs, carry, result);
    }

    #[inline(always)]
    pub fn op_dec8(&mut self, lhs: u8, result: u8) {
        self.pending = FlagsCtx::new_dec8(lhs, result);
    }

    #[inline(always)]
    pub fn op_inc8(&mut self, lhs: u8, result: u8) {
        self.pending = FlagsCtx::new_inc8(lhs, result);
    }

    #[inline(always)]
    pub fn op_and(&mut self, result: u8) {
        self.pending = FlagsCtx::new_and(result);
    }

    #[inline(always)]
    pub fn op_cpl(&mut self) {
        self.pending = FlagsCtx::new_cpl();
    }

    #[inline(always)]
    pub fn op_or(&mut self, result: u8) {
        self.pending = FlagsCtx::new_or(result);
    }

    #[inline(always)]
    pub fn force_op_or(&mut self, result: u8) {
        FlagsOp::or(FlagsData::with_result(result), self);
    }

    #[inline(always)]
    pub fn op_rla(&mut self, lhs: u8) {
        self.pending = FlagsCtx::new_rla(lhs);
    }

    #[inline(always)]
    pub fn op_rlca(&mut self, carry: u8) {
        self.pending = FlagsCtx::new_rlca(carry);
    }

    #[inline(always)]
    pub fn op_rra(&mut self, lhs: u8) {
        self.pending = FlagsCtx::new_rra(lhs);
    }

    #[inline(always)]
    pub fn force_op_rra(&mut self, lhs: u8) {
        FlagsOp::rra(FlagsData::with_lhs(lhs as u16), self);
    }

    #[inline(always)]
    pub fn op_ccf(&mut self, carry: u8) {
        self.pending = FlagsCtx::new_ccf(carry);
    }

    #[inline(always)]
    pub fn op_scf(&mut self) {
        self.pending = FlagsCtx::new_scf();
    }

    #[inline(always)]
    pub fn op_ld(&mut self, lhs: u16, rhs: u16) {
        self.pending = FlagsCtx::new_ld(lhs, rhs);
    }
}
