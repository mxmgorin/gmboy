use crate::core::cart::Cart;
use crate::core::ram::Ram;

#[derive(Debug, Clone)]
pub struct Bus {
    cart: Cart,
    ram: Ram,
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
    ZeroPage,              // 0xFF80 - 0xFFFE
}

impl TryFrom<u16> for AddrLocation {
    type Error = String;

    fn try_from(address: u16) -> Result<Self, Self::Error> {
        match address {
            0x0000..=0x3FFF => Ok(AddrLocation::RomBank0),
            0x4000..=0x7FFF => Ok(AddrLocation::RomBank1),
            0x8000..=0x97FF => Ok(AddrLocation::ChrRam),
            0x9800..=0x9BFF => Ok(AddrLocation::BgMap1),
            0x9C00..=0x9FFF => Ok(AddrLocation::BgMap2),
            0xA000..=0xBFFF => Ok(AddrLocation::CartRam),
            0xC000..=0xCFFF => Ok(AddrLocation::RamBank0),
            0xD000..=0xDFFF => Ok(AddrLocation::RamBank1To7),
            0xE000..=0xFDFF => Ok(AddrLocation::EchoRam),
            0xFE00..=0xFE9F => Ok(AddrLocation::ObjectAttributeMemory),
            0xFEA0..=0xFEFF => Ok(AddrLocation::Unusable),
            0xFF00..=0xFF7F => Ok(AddrLocation::IoRegisters),
            0xFF80..=0xFFFE => Ok(AddrLocation::ZeroPage),
            _ => Err(format!("0x{:X} address out of range ", address)),
        }
    }
}

impl Bus {
    pub fn new(cart: Cart, ram: Ram) -> Self {
        Self { cart, ram }
    }

    pub fn read(&self, address: u16) -> u8 {
        let location = AddrLocation::try_from(address).unwrap();

        match location {
            AddrLocation::RomBank0 | AddrLocation::RomBank1 | AddrLocation::CartRam => {
                return self.cart.read(address)
            }
            AddrLocation::ChrRam => {}
            AddrLocation::BgMap1 => {}
            AddrLocation::BgMap2 => {}
            AddrLocation::RamBank0 => {}
            AddrLocation::RamBank1To7 | AddrLocation::EchoRam => {
                return self.ram.w_ram_read(address)
            }
            AddrLocation::ObjectAttributeMemory => {}
            AddrLocation::Unusable => {}
            AddrLocation::IoRegisters => {}
            AddrLocation::ZeroPage => {}
        }

        panic!("Can't bus read read address {address}");
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let location = AddrLocation::try_from(address).unwrap();

        match location {
            AddrLocation::RomBank0 | AddrLocation::RomBank1 | AddrLocation::CartRam => {
                self.cart.write(address, value);
                return;
            }
            AddrLocation::ChrRam => {}
            AddrLocation::BgMap1 => {}
            AddrLocation::BgMap2 => {}
            AddrLocation::RamBank0 | AddrLocation::RamBank1To7 => {
                self.ram.w_ram_write(address, value);
                return;
            }
            AddrLocation::EchoRam => {}
            AddrLocation::ObjectAttributeMemory => {}
            AddrLocation::Unusable => {}
            AddrLocation::IoRegisters => {}
            AddrLocation::ZeroPage => {}
        }

        panic!("Can't bus write address {address}");
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
