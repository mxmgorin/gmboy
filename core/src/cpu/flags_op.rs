use crate::cpu::flags::Flags;
use serde::{Deserialize, Serialize};

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

    #[inline(always)]
    pub fn add8(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(data.result == 0);
        flags.set_n_raw(false);
        flags.set_h_raw((data.lhs as u8 & 0xF) + (data.rhs as u8 & 0xF) + data.carry > 0xF);
        flags.set_c_raw((data.lhs + data.rhs + data.carry as u16) > 0xFF);
    }

    #[inline(always)]
    pub fn add16(data: FlagsData, flags: &mut Flags) {
        flags.set_n_raw(false);
        flags.set_h_raw(((data.lhs & 0x0FFF) + (data.rhs & 0x0FFF)) > 0x0FFF);
        flags.set_c_raw((data.lhs as u32 + data.rhs as u32) > 0xFFFF);
    }

    #[inline(always)]
    pub fn add_sp_e8(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(false);
        flags.set_n_raw(false);
        flags.set_h_raw((data.lhs & 0xF) + (data.rhs & 0xF) > 0xF);
        flags.set_c_raw(((data.lhs & 0xFF) + (data.rhs & 0xFF)) > 0xFF);
    }

    pub fn dec8(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(data.result == 0);
        flags.set_n_raw(true);
        flags.set_h_raw((data.lhs & 0xF) == 0);
    }

    #[inline(always)]
    pub fn inc8(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(data.result == 0);
        flags.set_n_raw(false);
        flags.set_h_raw((data.lhs & 0xF) + 1 > 0xF);
    }

    pub fn sub8(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(data.result == 0);
        flags.set_n_raw(true);
        flags.set_h_raw((data.lhs as u8 & 0xF) < ((data.rhs as u8 & 0xF) + data.carry));
        flags.set_c_raw((data.lhs) < (data.rhs + data.carry as u16));
    }

    #[inline(always)]
    pub fn and(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(data.result == 0);
        flags.set_n_raw(false);
        flags.set_h_raw(true);
        flags.set_c_raw(false);
    }

    pub fn cpl(_data: FlagsData, flags: &mut Flags) {
        flags.set_n_raw(true);
        flags.set_h_raw(true);
    }

    pub fn or(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(data.result == 0);
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(false);
    }

    #[inline(always)]
    pub fn ld(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(false);
        flags.set_n_raw(false);
        flags.set_h_raw((data.lhs & 0xF) + (data.rhs & 0xF) >= 0x10);
        flags.set_c_raw((data.lhs & 0xFF) + (data.rhs & 0xFF) >= 0x100);
    }

    #[inline(always)]
    pub fn ccf(data: FlagsData, flags: &mut Flags) {
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(data.carry == 0);
    }

    pub fn scf(_data: FlagsData, flags: &mut Flags) {
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(true);
    }

    #[inline(always)]
    pub fn rla(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(false);
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(((data.lhs >> 7) & 1) != 0);
    }

    #[inline(always)]
    pub fn rlca(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(false);
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw(data.carry != 0);
    }

    #[inline(always)]
    pub fn rra(data: FlagsData, flags: &mut Flags) {
        flags.set_z_raw(false);
        flags.set_n_raw(false);
        flags.set_h_raw(false);
        flags.set_c_raw((data.lhs & 1) != 0);
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
