use crate::core::cart::Cart;
use crate::core::io::Io;
use crate::core::ram::Ram;

impl From<u16> for BusAddrLocation {
    fn from(address: u16) -> Self {
        match address {
            0x0000..=0x3FFF => BusAddrLocation::RomBank0,
            0x4000..=0x7FFF => BusAddrLocation::RomBank1,
            0x8000..=0x97FF => BusAddrLocation::ChrRam,
            0x9800..=0x9BFF => BusAddrLocation::BgMap1,
            0x9C00..=0x9FFF => BusAddrLocation::BgMap2,
            0xA000..=0xBFFF => BusAddrLocation::CartRam,
            0xC000..=0xCFFF => BusAddrLocation::RamBank0,
            0xD000..=0xDFFF => BusAddrLocation::RamBank1To7,
            0xE000..=0xFDFF => BusAddrLocation::EchoRam,
            0xFE00..=0xFE9F => BusAddrLocation::ObjectAttributeMemory,
            0xFEA0..=0xFEFF => BusAddrLocation::Unusable,
            0xFF00..=0xFF7F => BusAddrLocation::IoRegisters,
            0xFF80..=0xFFFE => BusAddrLocation::ZeroPage,
            0xFFFF => BusAddrLocation::IeRegister,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bus {
    cart: Cart,
    ram: Ram,
    pub io: Io,
}

impl Bus {
    pub fn new(cart: Cart, ram: Ram) -> Self {
        Self {
            cart,
            ram,
            io: Io::new(),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let location = BusAddrLocation::from(address);

        match location {
            BusAddrLocation::RomBank0 | BusAddrLocation::RomBank1 | BusAddrLocation::CartRam => {
                self.cart.read(address)
            }
            BusAddrLocation::ChrRam => panic!("Can't bus read address {:X}", address),
            BusAddrLocation::BgMap1 => panic!("Can't bus read address {:X}", address),
            BusAddrLocation::BgMap2 => panic!("Can't bus read address {:X}", address),
            BusAddrLocation::RamBank0 => panic!("Can't bus read address {:X}", address),
            BusAddrLocation::RamBank1To7 => self.ram.w_ram_read(address),
            BusAddrLocation::EchoRam => 0,
            BusAddrLocation::ObjectAttributeMemory => {
                panic!("Can't bus read address {:X}", address)
            }
            BusAddrLocation::Unusable => 0,
            BusAddrLocation::IoRegisters => self.io.read(address),
            BusAddrLocation::ZeroPage => self.ram.h_ram_read(address),
            BusAddrLocation::IeRegister => self.io.interrupts.ie_register,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let location = BusAddrLocation::from(address);

        match location {
            BusAddrLocation::RomBank0 | BusAddrLocation::RomBank1 | BusAddrLocation::CartRam => {
                self.cart.write(address, value)
            }
            BusAddrLocation::ChrRam => panic!("Can't bus write ChrRam address {:X}", address),
            BusAddrLocation::BgMap1 => panic!("Can't bus write BgMap1 address {:X}", address),
            BusAddrLocation::BgMap2 => panic!("Can't bus write BgMap2 address {:X}", address),
            BusAddrLocation::RamBank0 | BusAddrLocation::RamBank1To7 => {
                self.ram.w_ram_write(address, value)
            }
            BusAddrLocation::EchoRam => {},
            BusAddrLocation::ObjectAttributeMemory => {
                panic!("Can't bus write ObjectAttributeMemory address {:X}", address)
            }
            BusAddrLocation::Unusable => (),
            BusAddrLocation::IoRegisters => self.io.write(address, value),
            BusAddrLocation::ZeroPage => self.ram.h_ram_write(address, value),
            BusAddrLocation::IeRegister => self.io.interrupts.ie_register = value,
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

#[derive(Debug, PartialEq, Eq)]
pub enum BusAddrLocation {
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
    /// Interrupt enable register
    IeRegister, // 0xFFFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ie_register() {
        let location: BusAddrLocation = 0xFFFF.into();

        assert_eq!(location, BusAddrLocation::IeRegister);
    }
}
