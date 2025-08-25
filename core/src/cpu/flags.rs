use crate::cpu::instructions::arithmetic::add::{Add16FlagsCtx, Add8FlagsCtx, AddSpE8FlagsCtx};
use crate::{get_bit_flag, set_bit};
use serde::{Deserialize, Serialize};
use std::mem;
use crate::cpu::instructions::arithmetic::dec::Dec8FlagsCtx;
use crate::cpu::instructions::arithmetic::inc::Inc8FlagsCtx;
use crate::cpu::instructions::arithmetic::sub::Sub8FlagsCtx;
use crate::cpu::instructions::rotate::rla::RlaFlagsCtx;

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
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
            pending: FlagsCtx::None,
        }
    }
}

impl Flags {
    pub fn new(byte: u8) -> Flags {
        Self {
            byte,
            pending: FlagsCtx::None,
        }
    }

    #[inline(always)]
    pub fn set(&mut self, ctx: FlagsCtx) {
        self.pending = ctx;
    }

    #[inline(always)]
    pub fn get_byte(&mut self) -> u8 {
        self.apply_pending();
        self.byte
    }

    #[inline(always)]
    pub const fn set_byte(&mut self, byte: u8) {
        self.pending = FlagsCtx::None;
        self.byte = byte;
    }

    #[inline(always)]
    pub fn set_z(&mut self, v: bool) {
        self.apply_pending();
        set_bit(&mut self.byte, ZERO_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn set_n(&mut self, v: bool) {
        self.apply_pending();
        set_bit(&mut self.byte, SUBTRACT_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn set_h(&mut self, v: bool) {
        self.apply_pending();
        set_bit(&mut self.byte, HALF_CARRY_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn set_c(&mut self, v: bool) {
        self.apply_pending();
        set_bit(&mut self.byte, CARRY_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn get_z(&mut self) -> bool {
        self.apply_pending();
        get_bit_flag(self.byte, ZERO_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_n(&mut self) -> bool {
        self.apply_pending();
        get_bit_flag(self.byte, SUBTRACT_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_h(&mut self) -> bool {
        self.apply_pending();
        get_bit_flag(self.byte, HALF_CARRY_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_c(&mut self) -> bool {
        self.apply_pending();
        get_bit_flag(self.byte, CARRY_FLAG_BYTE_POSITION)
    }

    #[inline]
    fn apply_pending(&mut self) {
        let pending = mem::replace(&mut self.pending, FlagsCtx::None);
        pending.apply(self);
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
    pub fn set_z_inner(&mut self, v: bool) {
        set_bit(&mut self.byte, ZERO_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn set_n_inner(&mut self, v: bool) {
        set_bit(&mut self.byte, SUBTRACT_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn set_h_inner(&mut self, v: bool) {
        set_bit(&mut self.byte, HALF_CARRY_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn set_c_inner(&mut self, v: bool) {
        set_bit(&mut self.byte, CARRY_FLAG_BYTE_POSITION, v);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlagsCtx {
    None,
    Add8(Add8FlagsCtx),
    Add16(Add16FlagsCtx),
    AddSpE8(AddSpE8FlagsCtx),
    Sub8(Sub8FlagsCtx),
    Inc8(Inc8FlagsCtx),
    Dec8(Dec8FlagsCtx),
    Rla(RlaFlagsCtx),
}

impl FlagsCtx {
    pub fn apply(self, flags: &mut Flags) {
        match self {
            FlagsCtx::None => {}
            FlagsCtx::Add8(x) => x.apply(flags),
            FlagsCtx::Add16(x) => x.apply(flags),
            FlagsCtx::AddSpE8(x) => x.apply(flags),
            FlagsCtx::Sub8(x) => x.apply(flags),
            FlagsCtx::Inc8(x) => x.apply(flags),
            FlagsCtx::Dec8(x) => x.apply(flags),
            FlagsCtx::Rla(x) => x.apply(flags),
        }
    }
}
