use crate::{get_bit_flag, set_bit};
use serde::{Deserialize, Serialize};

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const NEGATIVE_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flags {
    byte: u8,
    pending: Option<FlagsCtx>,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            byte: 0xB0,
            pending: None,
        }
    }
}

impl Flags {
    pub fn new(byte: u8) -> Flags {
        Self {
            byte,
            pending: None,
        }
    }

    #[inline(always)]
    pub fn set(&mut self, ctx: FlagsCtx) {
        self.pending = Some(ctx);
    }

    #[inline(always)]
    pub fn force_set(&mut self, ctx: FlagsCtx) {
        self.pending = Some(ctx);
        self.apply_pending();
    }

    #[inline(always)]
    pub fn get_byte(&mut self) -> u8 {
        self.apply_pending();
        self.byte
    }

    #[inline(always)]
    pub const fn set_byte(&mut self, byte: u8) {
        self.pending = None;
        self.byte = byte;
    }

    #[inline(always)]
    pub fn get_z(&mut self) -> bool {
        self.apply_pending();
        get_bit_flag(self.byte, ZERO_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_n(&mut self) -> bool {
        self.apply_pending();
        get_bit_flag(self.byte, NEGATIVE_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_hnc(&mut self) -> (bool, bool, bool) {
        self.apply_pending();
        (self.get_h(), self.get_n(), self.get_c())
    }

    #[inline(always)]
    pub fn set_zhc(&mut self, z: bool, h: bool, c: bool) {
        self.apply_pending();
        self.set_z_raw(z);
        self.set_h_raw(h);
        self.set_c_raw(c);
    }

    #[inline(always)]
    pub fn set_znhc(&mut self, z: bool, n: bool, h: bool, c: bool) {
        self.pending = None;
        self.set_z_raw(z);
        self.set_n_raw(n);
        self.set_h_raw(h);
        self.set_c_raw(c);
    }

    #[inline(always)]
    pub fn set_znh(&mut self, z: bool, n: bool, h: bool) {
        self.apply_pending();
        self.set_z_raw(z);
        self.set_n_raw(n);
        self.set_h_raw(h);
    }

    #[inline(always)]
    pub fn get_c(&mut self) -> bool {
        self.apply_pending();
        get_bit_flag(self.byte, CARRY_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_h(&mut self) -> bool {
        self.apply_pending();
        get_bit_flag(self.byte, HALF_CARRY_FLAG_BYTE_POSITION)
    }

    #[inline]
    fn apply_pending(&mut self) {
        if let Some(pending) = self.pending.take() {
            pending.apply(self);
        }
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

impl FlagsCtx {
    #[inline(always)]
    pub fn apply(self, flags: &mut Flags) {
        let index = self.op as usize;

        unsafe {
            APPLY_TABLE.get_unchecked(index)(self.data, flags);
        }
    }

    #[inline(always)]
    pub fn add8(lhs: u8, rhs: u8, carry_in: u8, result: u8) -> Self {
        Self {
            op: FlagsOp::Add8,
            data: FlagsData {
                lhs: lhs as u16,
                rhs: rhs as u16,
                carry_in,
                result,
            },
        }
    }

    #[inline(always)]
    pub fn add16(lhs: u16, rhs: u16) -> Self {
        Self {
            op: FlagsOp::Add16,
            data: FlagsData {
                lhs,
                rhs,
                carry_in: 0,
                result: 0,
            },
        }
    }

    #[inline(always)]
    pub fn add_sp_e8(lhs: u16, rhs: u16) -> Self {
        Self {
            op: FlagsOp::AddSpE8,
            data: FlagsData {
                lhs,
                rhs,
                carry_in: 0,
                result: 0,
            },
        }
    }

    #[inline(always)]
    pub fn sub8(lhs: u8, rhs: u8, carry_in: u8, result: u8) -> Self {
        Self {
            op: FlagsOp::Sub8,
            data: FlagsData {
                lhs: lhs as u16,
                rhs: rhs as u16,
                carry_in,
                result,
            },
        }
    }

    #[inline(always)]
    pub fn dec8(lhs: u8, result: u8) -> Self {
        Self {
            op: FlagsOp::Dec8,
            data: FlagsData {
                lhs: lhs as u16,
                rhs: 0,
                carry_in: 0,
                result,
            },
        }
    }

    #[inline(always)]
    pub fn inc8(lhs: u8, result: u8) -> Self {
        Self {
            op: FlagsOp::Inc8,
            data: FlagsData {
                lhs: lhs as u16,
                rhs: 0,
                carry_in: 0,
                result,
            },
        }
    }

    #[inline(always)]
    pub fn and(result: u8) -> Self {
        Self {
            op: FlagsOp::And,
            data: FlagsData {
                lhs: 0,
                rhs: 0,
                carry_in: 0,
                result,
            },
        }
    }

    #[inline(always)]
    pub fn cpl() -> Self {
        Self {
            op: FlagsOp::Cpl,
            data: FlagsData::default(),
        }
    }

    #[inline(always)]
    pub fn or(result: u8) -> Self {
        Self {
            op: FlagsOp::Or,
            data: FlagsData {
                lhs: 0,
                rhs: 0,
                carry_in: 0,
                result,
            },
        }
    }

    #[inline(always)]
    pub fn rla(lhs: u8) -> Self {
        Self {
            op: FlagsOp::Rla,
            data: FlagsData {
                lhs: lhs as u16,
                rhs: 0,
                carry_in: 0,
                result: 0,
            },
        }
    }

    #[inline(always)]
    pub fn rlca(carry_in: u8) -> Self {
        Self {
            op: FlagsOp::Rlca,
            data: FlagsData {
                lhs: 0,
                rhs: 0,
                carry_in,
                result: 0,
            },
        }
    }

    #[inline(always)]
    pub fn rra(lhs: u8) -> Self {
        Self {
            op: FlagsOp::Rra,
            data: FlagsData {
                lhs: lhs as u16,
                rhs: 0,
                carry_in: 0,
                result: 0,
            },
        }
    }

    #[inline(always)]
    pub fn ccf(carry_in: u8) -> Self {
        Self {
            op: FlagsOp::Ccf,
            data: FlagsData {
                lhs: 0,
                rhs: 0,
                carry_in,
                result: 0,
            },
        }
    }

    #[inline(always)]
    pub fn scf() -> Self {
        Self {
            op: FlagsOp::Scf,
            data: FlagsData::default(),
        }
    }

    #[inline(always)]
    pub fn ld(lhs: u16, rhs: u16) -> Self {
        Self {
            op: FlagsOp::Ld,
            data: FlagsData {
                lhs,
                rhs,
                carry_in: 0,
                result: 0,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FlagsData {
    pub lhs: u16,
    pub rhs: u16,
    pub carry_in: u8,
    pub result: u8,
}

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
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

type ApplyFlagsFn = fn(FlagsData, &mut Flags);

const APPLY_TABLE: [ApplyFlagsFn; 15] = [
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
];
