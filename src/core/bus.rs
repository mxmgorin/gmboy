use crate::core::cart::Cart;
use crate::core::ram::Ram;

#[derive(Debug, Clone)]
pub struct Bus {
    cart: Cart,
    ram: Ram,
    ie_register: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddrLocation {
    RomBank0,              // 0x0000 - 0x3FFF
    RomBank1,              // 0x4000 - 0x7FFF
    ChrRam,                // 0x8000 - 0x97FF
    BgMap1,                // 0x9800 - 0x9BFF
    BgMap2,                // 0x9C00 - 0x9FFF
    CartRam,               // 0xA000 - 0xBFFF
    RamBank0,              // 0xC000 - 0xCFFF
    RamBank1To7,           // 0xD000 - 0xDFFF
    EchoRam,               // 0xE000 - 0xFDFF
    ObjectAttributeMemory, // 0xFE00 - 0xFE9F
    Unusable,              // 0xFEA0 - 0xFEFF
    IoRegisters,           // 0xFF00 - 0xFF7F
    /// Aka High RAM (HRAM)
    ZeroPage, // 0xFF80 - 0xFFFE
    IeRegister,            // 0xFFFF
}

impl From<u16> for AddrLocation {
    fn from(address: u16) -> Self {
        match address {
            0x0000..=0x3FFF => AddrLocation::RomBank0,
            0x4000..=0x7FFF => AddrLocation::RomBank1,
            0x8000..=0x97FF => AddrLocation::ChrRam,
            0x9800..=0x9BFF => AddrLocation::BgMap1,
            0x9C00..=0x9FFF => AddrLocation::BgMap2,
            0xA000..=0xBFFF => AddrLocation::CartRam,
            0xC000..=0xCFFF => AddrLocation::RamBank0,
            0xD000..=0xDFFF => AddrLocation::RamBank1To7,
            0xE000..=0xFDFF => AddrLocation::EchoRam,
            0xFE00..=0xFE9F => AddrLocation::ObjectAttributeMemory,
            0xFEA0..=0xFEFF => AddrLocation::Unusable,
            0xFF00..=0xFF7F => AddrLocation::IoRegisters,
            0xFF80..=0xFFFE => AddrLocation::ZeroPage,
            0xFFFF => AddrLocation::IeRegister,
        }
    }
}

impl Bus {
    pub fn new(cart: Cart, ram: Ram) -> Self {
        Self {
            cart,
            ram,
            ie_register: 0,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let location = AddrLocation::from(address);

        match location {
            AddrLocation::RomBank0 | AddrLocation::RomBank1 | AddrLocation::CartRam => {
                self.cart.read(address)
            }
            AddrLocation::ChrRam => panic!("Can't bus read address {:X}", address),
            AddrLocation::BgMap1 => panic!("Can't bus read address {:X}", address),
            AddrLocation::BgMap2 => panic!("Can't bus read address {:X}", address),
            AddrLocation::RamBank0 => panic!("Can't bus read address {:X}", address),
            AddrLocation::RamBank1To7 | AddrLocation::EchoRam => self.ram.w_ram_read(address),
            AddrLocation::ObjectAttributeMemory => panic!("Can't bus read address {:X}", address),
            AddrLocation::Unusable => 0,
            AddrLocation::IoRegisters => {
                // TODO: impl
                eprintln!("Can't bus read IoRegisters address {:X}", address);
                0
            }
            AddrLocation::ZeroPage => self.ram.h_ram_read(address),
            AddrLocation::IeRegister => self.ie_register,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let location = AddrLocation::from(address);

        match location {
            AddrLocation::RomBank0 | AddrLocation::RomBank1 | AddrLocation::CartRam => {
                self.cart.write(address, value)
            }
            AddrLocation::ChrRam => panic!("Can't bus write address {:X}", address),
            AddrLocation::BgMap1 => panic!("Can't bus write address {:X}", address),
            AddrLocation::BgMap2 => panic!("Can't bus write address {:X}", address),
            AddrLocation::RamBank0 | AddrLocation::RamBank1To7 => {
                self.ram.w_ram_write(address, value)
            }
            AddrLocation::EchoRam => panic!("Can't bus write address {:X}", address),
            AddrLocation::ObjectAttributeMemory => panic!("Can't bus write address {:X}", address),
            AddrLocation::Unusable => (),
            AddrLocation::IoRegisters => {
                eprint!("Can't bus write IoRegisters address {:X}", address)
            } // TODO: impl
            AddrLocation::ZeroPage => self.ram.h_ram_write(address, value),
            AddrLocation::IeRegister => self.ie_register = value,
        }
    }

    pub fn read16(&self, address: u16) -> u16 {
        let lo = self.read(address) as u16;
        let hi = self.read(address + 1) as u16;

        lo | (hi << 8)
    }

    pub fn write16(&mut self, address: u16, value: u16) {
        self.write(address + 1, ((value >> 8) & 0xFF) as u8);
        self.write(address, (value & 0xFF) as u8);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ie_register() {
        let location: AddrLocation = 0xFFFF.into();

        assert_eq!(location, AddrLocation::IeRegister);
    }
}
