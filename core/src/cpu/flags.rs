#[cfg(feature = "lazy-flags")]
use crate::cpu::flags_ctx::FlagsCtx;
use crate::cpu::flags_op::FlagsOp;
use crate::{get_bit_flag, set_bit};
use serde::{Deserialize, Serialize};

pub const ZERO_FLAG_BYTE_POSITION: u8 = 7;
pub const NEGATIVE_FLAG_BYTE_POSITION: u8 = 6;
pub const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
pub const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flags {
    byte: u8,
    #[cfg(feature = "lazy-flags")]
    pending: Option<FlagsCtx>,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            byte: 0xB0,
            #[cfg(feature = "lazy-flags")]
            pending: None,
        }
    }
}

impl Flags {
    pub fn new(byte: u8) -> Flags {
        Self {
            byte,
            #[cfg(feature = "lazy-flags")]
            pending: None,
        }
    }

    #[inline(always)]
    pub fn get_byte(&mut self) -> u8 {
        #[cfg(feature = "lazy-flags")]
        self.compute_pending();
        self.byte
    }

    #[inline(always)]
    pub const fn set_byte(&mut self, byte: u8) {
        #[cfg(feature = "lazy-flags")]
        self.pending.take();

        self.byte = byte;
    }

    #[inline(always)]
    pub const fn set_byte_raw(&mut self, byte: u8) {
        self.byte = byte;
    }

    #[inline(always)]
    pub fn get_z(&mut self) -> bool {
        #[cfg(feature = "lazy-flags")]
        self.compute_pending();

        get_bit_flag(self.byte, ZERO_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_hnc(&mut self) -> (bool, bool, bool) {
        #[cfg(feature = "lazy-flags")]
        self.compute_pending();

        (
            get_bit_flag(self.byte, HALF_CARRY_FLAG_BYTE_POSITION),
            get_bit_flag(self.byte, NEGATIVE_FLAG_BYTE_POSITION),
            get_bit_flag(self.byte, CARRY_FLAG_BYTE_POSITION),
        )
    }

    #[inline(always)]
    fn get_znhc(&mut self) -> (bool, bool, bool, bool) {
        #[cfg(feature = "lazy-flags")]
        self.compute_pending();

        (
            get_bit_flag(self.byte, ZERO_FLAG_BYTE_POSITION),
            get_bit_flag(self.byte, NEGATIVE_FLAG_BYTE_POSITION),
            get_bit_flag(self.byte, HALF_CARRY_FLAG_BYTE_POSITION),
            get_bit_flag(self.byte, CARRY_FLAG_BYTE_POSITION),
        )
    }

    #[inline(always)]
    pub fn set_zhc(&mut self, z: bool, h: bool, c: bool) {
        #[cfg(feature = "lazy-flags")]
        self.compute_pending();

        self.set_z_raw(z);
        self.set_h_raw(h);
        self.set_c_raw(c);
    }

    #[inline(always)]
    pub fn set_znhc(&mut self, z: bool, n: bool, h: bool, c: bool) {
        #[cfg(feature = "lazy-flags")]
        self.pending.take();

        self.set_z_raw(z);
        self.set_n_raw(n);
        self.set_h_raw(h);
        self.set_c_raw(c);
    }

    #[inline(always)]
    pub fn set_znh(&mut self, z: bool, n: bool, h: bool) {
        #[cfg(feature = "lazy-flags")]
        self.compute_pending();

        self.set_z_raw(z);
        self.set_n_raw(n);
        self.set_h_raw(h);
    }

    #[inline(always)]
    pub fn get_c(&mut self) -> bool {
        #[cfg(feature = "lazy-flags")]
        self.compute_pending();

        get_bit_flag(self.byte, CARRY_FLAG_BYTE_POSITION)
    }

    #[cfg(feature = "lazy-flags")]
    #[inline(always)]
    fn compute_pending(&mut self) {
        #[cfg(feature = "lazy-flags")]
        if let Some(pending) = self.pending.take() {
            pending.compute(self);
        }
    }

    #[inline(always)]
    pub const fn set_z_raw(&mut self, v: bool) {
        set_bit(&mut self.byte, ZERO_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub const fn set_n_raw(&mut self, v: bool) {
        set_bit(&mut self.byte, NEGATIVE_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub const fn set_h_raw(&mut self, v: bool) {
        set_bit(&mut self.byte, HALF_CARRY_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub const fn set_c_raw(&mut self, v: bool) {
        set_bit(&mut self.byte, CARRY_FLAG_BYTE_POSITION, v);
    }

    pub fn display(&mut self) -> String {
        let (z, n, h, c) = self.get_znhc();

        [(z, 'Z'), (n, 'N'), (h, 'H'), (c, 'C')]
            .iter()
            .map(|&(flag, c)| if flag { c } else { '-' })
            .collect()
    }

    #[inline(always)]
    pub fn op_add8(&mut self, lhs: u8, rhs: u8, carry: u8, result: u8) {
        #[cfg(feature = "lazy-flags")]
        self.pending
            .replace(FlagsCtx::new_add8(lhs, rhs, carry, result));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::add8(self, lhs, rhs, result, carry);
    }

    #[inline(always)]
    pub fn op_add16(&mut self, lhs: u16, rhs: u16) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_add16(lhs, rhs));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::add16(self, lhs, rhs);
    }

    #[inline(always)]
    pub fn op_add_sp_e8(&mut self, lhs: u16, rhs: u16) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_add_sp_e8(lhs, rhs));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::add_sp_e8(self, lhs, rhs);
    }

    #[inline(always)]
    pub fn op_sub8(&mut self, lhs: u8, rhs: u8, carry: u8, result: u8) {
        #[cfg(feature = "lazy-flags")]
        self.pending
            .replace(FlagsCtx::new_sub8(lhs, rhs, carry, result));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::sub8(self, lhs, rhs, result, carry);
    }

    #[inline(always)]
    pub fn op_dec8(&mut self, lhs: u8, result: u8) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_dec8(lhs, result));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::dec8(self, lhs, result);
    }

    #[inline(always)]
    pub fn op_inc8(&mut self, lhs: u8, result: u8) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_inc8(lhs, result));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::inc8(self, lhs, result);
    }

    #[inline(always)]
    pub fn op_and(&mut self, result: u8) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_and(result));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::and(self, result);
    }

    #[inline(always)]
    pub fn op_cpl(&mut self) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_cpl());

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::cpl(self);
    }

    #[inline(always)]
    pub fn op_or(&mut self, result: u8) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_or(result));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::or(self, result);
    }

    #[inline(always)]
    pub fn force_op_or(&mut self, result: u8) {
        FlagsOp::or(self, result);
    }

    #[inline(always)]
    pub fn op_rla(&mut self, lhs: u8) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_rla(lhs));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::rla(self, lhs);
    }

    #[inline(always)]
    pub fn op_rlca(&mut self, carry: u8) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_rlca(carry));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::rlca(self, carry);
    }

    #[inline(always)]
    pub fn op_rra(&mut self, lhs: u8) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_rra(lhs));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::rra(self, lhs);
    }

    #[inline(always)]
    pub fn force_op_rra(&mut self, lhs: u8) {
        FlagsOp::rra(self, lhs);
    }

    #[inline(always)]
    pub fn op_ccf(&mut self, carry: u8) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_ccf(carry));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::ccf(self, carry);
    }

    #[inline(always)]
    pub fn op_scf(&mut self) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_scf());

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::scf(self);
    }

    #[inline(always)]
    pub fn op_ld(&mut self, lhs: u16, rhs: u16) {
        #[cfg(feature = "lazy-flags")]
        self.pending.replace(FlagsCtx::new_ld(lhs, rhs));

        #[cfg(not(feature = "lazy-flags"))]
        FlagsOp::ld(self, lhs, rhs);
    }
}
