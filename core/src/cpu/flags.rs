use crate::{get_bit_flag, set_bit};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flags {
    pub byte: u8,
}

impl Flags {
    pub const fn boot() -> Flags {
        Self { byte: 0xB0 }
    }

    #[inline]
    pub fn set_z(&mut self, v: bool) {
        set_bit(&mut self.byte, ZERO_FLAG_BYTE_POSITION, v);
    }

    #[inline]
    pub fn set_n(&mut self, v: bool) {
        set_bit(&mut self.byte, SUBTRACT_FLAG_BYTE_POSITION, v);
    }

    #[inline]
    pub fn set_h(&mut self, v: bool) {
        set_bit(&mut self.byte, HALF_CARRY_FLAG_BYTE_POSITION, v);
    }

    #[inline]
    pub fn set_c(&mut self, v: bool) {
        set_bit(&mut self.byte, CARRY_FLAG_BYTE_POSITION, v);
    }

    #[inline]
    pub fn get_z(&self) -> bool {
        get_bit_flag(self.byte, ZERO_FLAG_BYTE_POSITION)
    }

    #[inline]
    pub fn get_n(&self) -> bool {
        get_bit_flag(self.byte, SUBTRACT_FLAG_BYTE_POSITION)
    }

    #[inline]
    pub fn get_h(&self) -> bool {
        get_bit_flag(self.byte, HALF_CARRY_FLAG_BYTE_POSITION)
    }

    #[inline]
    pub fn get_c(&self) -> bool {
        get_bit_flag(self.byte, CARRY_FLAG_BYTE_POSITION)
    }
}

impl Display for Flags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str: String = [
            (self.get_z(), 'Z'),
            (self.get_n(), 'N'),
            (self.get_h(), 'H'),
            (self.get_c(), 'C'),
        ]
        .iter()
        .map(|&(flag, c)| if flag { c } else { '-' })
        .collect();
        write!(f, "{str}")
    }
}
