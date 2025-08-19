use crate::{get_bit_flag, set_bit};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registers {
    pub a: u8,
    pub flags: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

/// Represents the various CPU registers in a Game Boy CPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RegisterType {
    /// Accumulator register, used for arithmetic and logic operations.
    A = 0,
    /// Flags register, holds condition flags (Z, N, H, C).
    F = 1,
    /// General-purpose register B.
    B = 2,
    /// General-purpose register C.
    C = 3,
    /// General-purpose register D.
    D = 4,
    /// General-purpose register E.
    E = 5,
    /// High byte of the HL register pair.
    H = 6,
    /// Low byte of the HL register pair.
    L = 7,
    /// Register pair combining A and F (used for specific operations).
    AF = 8,
    /// Register pair combining B and C (used for addressing or data storage).
    BC = 9,
    /// Register pair combining D and E (used for addressing or data storage).
    DE = 10,
    /// Register pair combining H and L (often used as a memory address pointer).
    HL = 11,
    /// Stack pointer, points to the top of the stack.
    SP = 12,
    /// Program counter, points to the next instruction to be executed.
    PC = 13,
}

impl RegisterType {
    pub const fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::A,
            1 => Self::F,
            2 => Self::B,
            3 => Self::C,
            4 => Self::D,
            5 => Self::E,
            6 => Self::H,
            7 => Self::L,
            8 => Self::AF,
            9 => Self::BC,
            10 => Self::DE,
            11 => Self::HL,
            12 => Self::SP,
            13 => Self::PC,
            _ => panic!("invalid 8-bit register id"),
        }
    }

    pub fn is_16bit(&self) -> bool {
        match self {
            RegisterType::A
            | RegisterType::F
            | RegisterType::B
            | RegisterType::C
            | RegisterType::D
            | RegisterType::E
            | RegisterType::H
            | RegisterType::L => false,
            RegisterType::AF
            | RegisterType::BC
            | RegisterType::DE
            | RegisterType::HL
            | RegisterType::SP
            | RegisterType::PC => true,
        }
    }

    pub const fn get_all() -> &'static [RegisterType] {
        &[
            RegisterType::A,
            RegisterType::F,
            RegisterType::B,
            RegisterType::C,
            RegisterType::D,
            RegisterType::E,
            RegisterType::H,
            RegisterType::L,
            RegisterType::AF,
            RegisterType::BC,
            RegisterType::DE,
            RegisterType::HL,
            RegisterType::SP,
            RegisterType::PC,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flags {
    pub byte: u8,
}

impl Flags {
    pub fn boot() -> Flags {
        Self { byte: 0xB0 }
    }

    pub fn set(&mut self, z: Option<bool>, n: Option<bool>, h: Option<bool>, c: Option<bool>) {
        if let Some(z) = z {
            set_bit(&mut self.byte, ZERO_FLAG_BYTE_POSITION, z);
        }

        if let Some(n) = n {
            set_bit(&mut self.byte, SUBTRACT_FLAG_BYTE_POSITION, n);
        }

        if let Some(h) = h {
            set_bit(&mut self.byte, HALF_CARRY_FLAG_BYTE_POSITION, h);
        }

        if let Some(c) = c {
            set_bit(&mut self.byte, CARRY_FLAG_BYTE_POSITION, c);
        }
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

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0x01,
            flags: Flags::boot(),
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x100,
        }
    }
}

impl Registers {
    #[inline]
    pub fn read_register(&self, register_type: RegisterType) -> u16 {
        match register_type {
            RegisterType::A => self.a as u16,
            RegisterType::F => self.flags.byte as u16,
            RegisterType::B => self.b as u16,
            RegisterType::C => self.c as u16,
            RegisterType::D => self.d as u16,
            RegisterType::E => self.e as u16,
            RegisterType::H => self.h as u16,
            RegisterType::L => self.l as u16,
            RegisterType::AF => self.get_af(),
            RegisterType::BC => self.get_bc(),
            RegisterType::DE => self.get_de(),
            RegisterType::HL => self.get_hl(),
            RegisterType::PC => self.pc,
            RegisterType::SP => self.sp,
        }
    }

    #[inline]
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.flags.byte as u16
    }

    #[inline]
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    #[inline]
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    #[inline]
    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    #[inline]
    pub fn set_register(&mut self, register_type: RegisterType, value: u16) {
        match register_type {
            RegisterType::A => self.a = (value & 0xFF) as u8,
            RegisterType::F => self.flags.byte = (value & 0xFF) as u8,
            RegisterType::B => self.b = (value & 0xFF) as u8,
            RegisterType::C => self.c = (value & 0xFF) as u8,
            RegisterType::D => self.d = (value & 0xFF) as u8,
            RegisterType::E => self.e = (value & 0xFF) as u8,
            RegisterType::H => self.h = (value & 0xFF) as u8,
            RegisterType::L => self.l = (value & 0xFF) as u8,
            RegisterType::AF => self.set_af(value),
            RegisterType::BC => self.set_bc(value),
            RegisterType::DE => self.set_de(value),
            RegisterType::HL => self.set_hl(value),
            RegisterType::PC => self.pc = value,
            RegisterType::SP => self.sp = value,
        }
    }

    #[inline]
    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.flags.byte = (value & 0xFF) as u8;
    }

    #[inline]
    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    #[inline]
    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    #[inline]
    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::registers::Registers;

    #[test]
    fn test_get_flag_z() {
        let mut regs = Registers::default();
        regs.flags.byte = 0b10000000;
        assert!(regs.flags.get_z());

        regs.flags.byte = 0b00000000;
        assert!(!regs.flags.get_z());
    }

    #[test]
    fn test_get_flag_c() {
        let mut regs = Registers::default();
        regs.flags.byte = 0b00010000;
        assert!(regs.flags.get_c());

        regs.flags.byte = 0b00000000;
        assert!(!regs.flags.get_c());
    }

    #[test]
    fn test_set_flags() {
        let mut regs = Registers::default();
        regs.flags.byte = 0b10000000;

        regs.flags.set_c(true);

        assert!(regs.flags.get_z());
    }
}
