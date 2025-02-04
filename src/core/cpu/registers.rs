use crate::core::cpu::instructions::common::RegisterType;
use crate::core::util::{get_bit_flag, reverse_u16, set_bit};

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

#[derive(Debug, Clone)]
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
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
            f: 0xB0,
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
            RegisterType::F => self.f as u16,
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
        (self.a as u16) << 8 | self.f as u16
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
            RegisterType::F => self.f = (value & 0xFF) as u8,
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
        self.f = (value & 0xFF) as u8;
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

    pub fn set_flags(&mut self, z: Option<i8>, n: Option<i8>, h: Option<i8>, c: Option<i8>) {
        if let Some(z) = z {
            set_bit(&mut self.f, ZERO_FLAG_BYTE_POSITION, z != 0);
        }

        if let Some(n) = n {
            set_bit(&mut self.f, SUBTRACT_FLAG_BYTE_POSITION, n != 0);
        }

        if let Some(h) = h {
            set_bit(&mut self.f, HALF_CARRY_FLAG_BYTE_POSITION, h != 0);
        }

        if let Some(c) = c {
            set_bit(&mut self.f, CARRY_FLAG_BYTE_POSITION, c != 0);
        }
    }

    pub fn get_flag_z(&self) -> bool {
        get_bit_flag(self.f, ZERO_FLAG_BYTE_POSITION)
    }

    pub fn get_flag_n(&self) -> bool {
        get_bit_flag(self.f, SUBTRACT_FLAG_BYTE_POSITION)
    }

    pub fn get_flag_h(&self) -> bool {
        get_bit_flag(self.f, HALF_CARRY_FLAG_BYTE_POSITION)
    }

    pub fn get_flag_c(&self) -> bool {
        get_bit_flag(self.f, CARRY_FLAG_BYTE_POSITION)
    }

    pub fn flags_to_string(&self) -> String {
        [
            (self.get_flag_z(), 'Z'),
            (self.get_flag_n(), 'N'),
            (self.get_flag_h(), 'H'),
            (self.get_flag_c(), 'C'),
        ]
        .iter()
        .map(|&(flag, c)| if flag { c } else { '-' })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::registers::Registers;

    #[test]
    fn test_get_flag_z() {
        let mut regs = Registers::new();
        regs.f = 0b10000000;
        assert!(regs.get_flag_z());

        regs.f = 0b00000000;
        assert!(!regs.get_flag_z());
    }

    #[test]
    fn test_get_flag_c() {
        let mut regs = Registers::new();
        regs.f = 0b00010000;
        assert!(regs.get_flag_c());

        regs.f = 0b00000000;
        assert!(!regs.get_flag_c());
    }

    #[test]
    fn test_set_flags() {
        let mut regs = Registers::new();
        regs.f = 0b10000000;

        regs.set_flags(None, None, None, Some(1));

        assert!(regs.get_flag_z());
        println!("{:#b}", regs.f)
    }
}
