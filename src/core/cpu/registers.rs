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

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x01,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
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
            RegisterType::AF => reverse_u16(((self.a as u16) << 8) | (self.f as u16)),
            RegisterType::BC => reverse_u16(((self.b as u16) << 8) | (self.c as u16)),
            RegisterType::DE => reverse_u16(((self.d as u16) << 8) | (self.e as u16)),
            RegisterType::HL => reverse_u16(((self.h as u16) << 8) | (self.l as u16)),
            RegisterType::PC => self.pc,
            RegisterType::SP => self.sp,
        }
    }

    pub fn set_register(&mut self, register_type: RegisterType, val: u16) {
        match register_type {
            RegisterType::A => self.a = (val & 0xFF) as u8,
            RegisterType::F => self.f = (val & 0xFF) as u8,
            RegisterType::B => self.b = (val & 0xFF) as u8,
            RegisterType::C => self.c = (val & 0xFF) as u8,
            RegisterType::D => self.d = (val & 0xFF) as u8,
            RegisterType::E => self.e = (val & 0xFF) as u8,
            RegisterType::H => self.h = (val & 0xFF) as u8,
            RegisterType::L => self.l = (val & 0xFF) as u8,
            RegisterType::AF => {
                let reversed = reverse_u16(val);
                self.a = (reversed >> 8) as u8;
                self.f = (reversed & 0xFF) as u8;
            }
            RegisterType::BC => {
                let reversed = reverse_u16(val);
                self.b = (reversed >> 8) as u8;
                self.c = (reversed & 0xFF) as u8;
            }
            RegisterType::DE => {
                let reversed = reverse_u16(val);
                self.d = (reversed >> 8) as u8;
                self.e = (reversed & 0xFF) as u8;
            }
            RegisterType::HL => {
                let reversed = reverse_u16(val);
                self.h = (reversed >> 8) as u8;
                self.l = (reversed & 0xFF) as u8;
            }
            RegisterType::PC => self.pc = val,
            RegisterType::SP => self.sp = val,
        }
    }

    pub fn set_flags(&mut self, z: i8, n: i8, h: i8, c: i8) {
        if z != -1 {
            set_bit(&mut self.f, ZERO_FLAG_BYTE_POSITION, z != 0);
        }

        if n != -1 {
            set_bit(&mut self.f, SUBTRACT_FLAG_BYTE_POSITION, n != 0);
        }

        if h != -1 {
            set_bit(&mut self.f, HALF_CARRY_FLAG_BYTE_POSITION, h != 0);
        }

        if c != -1 {
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
}
