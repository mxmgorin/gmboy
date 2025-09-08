use crate::cpu::flags::Flags;
use crate::cpu::flags_op::FlagsOp;
use serde::{Deserialize, Serialize};

type OpFlagsFn = fn(FlagsData, &mut Flags);

const OP_FNS: [OpFlagsFn; 15] = {
    let mut table = [FlagsData::nop as OpFlagsFn; 15];
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

    table
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagsCtx {
    op: FlagsOp,
    data: FlagsData,
}

impl FlagsCtx {
    #[inline(always)]
    pub fn compute(self, flags: &mut Flags) {
        let index = self.op as usize;

        unsafe {
            OP_FNS.get_unchecked(index)(self.data, flags);
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
    pub fn nop(_: FlagsData, _: &mut Flags) {}
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
