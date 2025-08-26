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
    pub fn set(&mut self, ctx: FlagsCtx) {
        self.pending = ctx;
    }

    #[inline(always)]
    pub fn force_set(&mut self, ctx: FlagsCtx) {
        self.pending = ctx;
        self.compute_pending();
    }

    #[inline(always)]
    pub fn get_byte(&mut self) -> u8 {
        self.compute_pending();
        self.byte
    }

    #[inline(always)]
    pub const fn set_byte(&mut self, byte: u8) {
        self.pending.op = FlagsOp::Nop;
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
        self.pending.op = FlagsOp::Nop;
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
    pub fn compute(self, flags: &mut Flags) {
        let index = self.op as usize;

        unsafe {
            COMPUTE_TABLE.get_unchecked(index)(self.data, flags);
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
}

type ComputeFlagsFn = fn(FlagsData, &mut Flags);

const COMPUTE_TABLE: [ComputeFlagsFn; 16] = [
    FlagsOp::add8,      // 0
    FlagsOp::add16,     // 1
    FlagsOp::add_sp_e8, // 2
    FlagsOp::sub8,      // 3
    FlagsOp::inc8,      // 4
    FlagsOp::dec8,      // 5
    FlagsOp::rla,       // 6
    FlagsOp::and,       // 7
    FlagsOp::cpl,       // 8
    FlagsOp::or,        // 9
    FlagsOp::rlca,      // 10
    FlagsOp::rra,       // 11
    FlagsOp::ccf,       // 12
    FlagsOp::ccf,       // 13
    FlagsOp::ld,        // 14
    FlagsOp::nop,
];
