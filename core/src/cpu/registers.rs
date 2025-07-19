use crate::cpu::instructions::RegisterType;
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

    pub fn get_z(&self) -> bool {
        get_bit_flag(self.byte, ZERO_FLAG_BYTE_POSITION)
    }

    pub fn get_n(&self) -> bool {
        get_bit_flag(self.byte, SUBTRACT_FLAG_BYTE_POSITION)
    }

    pub fn get_h(&self) -> bool {
        get_bit_flag(self.byte, HALF_CARRY_FLAG_BYTE_POSITION)
    }

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
        write!(f, "{}", str)
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Self {
        // values after boot rom
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

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.flags.byte as u16
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

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

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.flags.byte = (value & 0xFF) as u8;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

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
        let mut regs = Registers::new();
        regs.flags.byte = 0b10000000;
        assert!(regs.flags.get_z());

        regs.flags.byte = 0b00000000;
        assert!(!regs.flags.get_z());
    }

    #[test]
    fn test_get_flag_c() {
        let mut regs = Registers::new();
        regs.flags.byte = 0b00010000;
        assert!(regs.flags.get_c());

        regs.flags.byte = 0b00000000;
        assert!(!regs.flags.get_c());
    }

    #[test]
    fn test_set_flags() {
        let mut regs = Registers::new();
        regs.flags.byte = 0b10000000;

        regs.flags.set(None, None, None, Some(true));

        assert!(regs.flags.get_z());
        println!("{:#b}", regs.flags.byte)
    }
}
