use crate::{get_bit_flag, set_bit};
use serde::{Deserialize, Serialize};

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flags {
    byte: u8,
    lazy: LazyFlags,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            byte: 0xB0,
            lazy: LazyFlags::None,
        }
    }
}

impl Flags {
    pub fn new(byte: u8) -> Flags {
        Self {
            byte,
            lazy: LazyFlags::None,
        }
    }

    #[inline(always)]
    pub fn set_lazy(&mut self, lazy: LazyFlags) {
        self.lazy = lazy;
    }

    #[inline(always)]
    pub fn get_byte(&mut self) -> u8 {
        self.flush_lazy();
        self.byte
    }

    #[inline(always)]
    pub const fn set_byte(&mut self, byte: u8) {
        self.lazy = LazyFlags::None;
        self.byte = byte;
    }

    #[inline(always)]
    pub fn set_z(&mut self, v: bool) {
        self.flush_lazy();
        set_bit(&mut self.byte, ZERO_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn set_n(&mut self, v: bool) {
        self.flush_lazy();
        set_bit(&mut self.byte, SUBTRACT_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn set_h(&mut self, v: bool) {
        self.flush_lazy();
        set_bit(&mut self.byte, HALF_CARRY_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn set_c(&mut self, v: bool) {
        self.flush_lazy();
        set_bit(&mut self.byte, CARRY_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    pub fn get_z(&mut self) -> bool {
        self.flush_lazy();
        get_bit_flag(self.byte, ZERO_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_n(&mut self) -> bool {
        self.flush_lazy();
        get_bit_flag(self.byte, SUBTRACT_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_h(&mut self) -> bool {
        self.flush_lazy();
        get_bit_flag(self.byte, HALF_CARRY_FLAG_BYTE_POSITION)
    }

    #[inline(always)]
    pub fn get_c(&mut self) -> bool {
        self.flush_lazy();
        get_bit_flag(self.byte, CARRY_FLAG_BYTE_POSITION)
    }

    #[inline]
    fn flush_lazy(&mut self) {
        match self.lazy {
            LazyFlags::None => {}
            LazyFlags::Add8 {
                lhs,
                rhs,
                carry_in,
                result,
            } => {
                self.set_z_inner(result == 0);
                self.set_n_inner(false);
                self.set_h_inner((lhs & 0xF) + (rhs & 0xF) + carry_in > 0xF);
                self.set_c_inner((lhs as u16 + rhs as u16 + carry_in as u16) > 0xFF);
                self.lazy = LazyFlags::None;
            }
            LazyFlags::Sub8 {
                lhs,
                rhs,
                carry_in,
                result,
            } => {
                self.set_z_inner(result == 0);
                self.set_n_inner(true);
                self.set_h_inner((lhs & 0xF) < ((rhs & 0xF) + carry_in));
                self.set_c_inner((lhs as u16) < (rhs as u16 + carry_in as u16));
                self.lazy = LazyFlags::None;
            }
            LazyFlags::Rla { carry } => {
                self.set_z_inner(false);
                self.set_n_inner(false);
                self.set_h_inner(false);
                self.set_c_inner(carry);
                self.lazy = LazyFlags::None;
            }
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
    fn set_z_inner(&mut self, v: bool) {
        set_bit(&mut self.byte, ZERO_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    fn set_n_inner(&mut self, v: bool) {
        set_bit(&mut self.byte, SUBTRACT_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    fn set_h_inner(&mut self, v: bool) {
        set_bit(&mut self.byte, HALF_CARRY_FLAG_BYTE_POSITION, v);
    }

    #[inline(always)]
    fn set_c_inner(&mut self, v: bool) {
        set_bit(&mut self.byte, CARRY_FLAG_BYTE_POSITION, v);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LazyFlags {
    None,
    Add8 {
        lhs: u8,
        rhs: u8,
        carry_in: u8,
        result: u8,
    },
    Sub8 {
        lhs: u8,
        rhs: u8,
        carry_in: u8,
        result: u8,
    },
    Rla {
        carry: bool,
    },
}
