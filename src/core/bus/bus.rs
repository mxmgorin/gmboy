use crate::bus::vram::VRam;
use crate::core::bus::io::Io;
use crate::core::bus::ram::Ram;
use crate::core::cart::Cart;

#[derive(Debug, PartialEq, Eq)]
pub enum BusAddrLocation {
    /// 0x0000 - 0x3FFF: 16 KiB ROM bank 00.From cartridge, usually a fixed bank.
    RomBank0,
    /// 0x4000 - 0x7FFF: 16 KiB ROM Bank 01–NN. From cartridge, switchable bank via mapper (if any).
    RomBank1,
    /// 8000 - 9FFF
    VRAM,
    /// 0xA000 - 0xBFFF: 8 KiB External RAM. From cartridge, switchable bank if any
    CartRam,
    /// 0xC000 - 0xCFFF
    WRamBank0,
    /// 0xD000 - 0xDFFF: 4 KiB Work RAM (WRAM). In CGB mode, switchable bank 1–7.
    WRamBank1To7,
    /// 0xE000 - 0xFDFF: Echo RAM (mirror of C000–DDFF). Nintendo says use of this area is prohibited.
    EchoRam,
    /// 0xFE00 - 0xFE9F: Object attribute memory (OAM)
    Oam,
    /// 0xFEA0 - 0xFEFF: Nintendo says use of this area is prohibited.
    Unusable,
    /// 0xFF00 - 0xFF7F
    IoRegisters,
    /// 0xFF80 - 0xFFFE: High RAM (HRAM). Aka ZeroPage
    HRam,
    /// 0xFFFF: Interrupt enable register.
    IeRegister,
}

impl From<u16> for BusAddrLocation {
    fn from(address: u16) -> Self {
        match address {
            0x0000..=0x3FFF => BusAddrLocation::RomBank0,
            0x4000..=0x7FFF => BusAddrLocation::RomBank1,
            0x8000..=0x9FFF => BusAddrLocation::VRAM,
            0xA000..=0xBFFF => BusAddrLocation::CartRam,
            0xC000..=0xCFFF => BusAddrLocation::WRamBank0,
            0xD000..=0xDFFF => BusAddrLocation::WRamBank1To7,
            0xE000..=0xFDFF => BusAddrLocation::EchoRam,
            0xFE00..=0xFE9F => BusAddrLocation::Oam,
            0xFEA0..=0xFEFF => BusAddrLocation::Unusable,
            0xFF00..=0xFF7F => BusAddrLocation::IoRegisters,
            0xFF80..=0xFFFE => BusAddrLocation::HRam,
            0xFFFF => BusAddrLocation::IeRegister,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bus {
    cart: Cart,
    ram: Ram,
    vram: VRam,
    pub io: Io,
}

impl Bus {
    pub fn new(cart: Cart, ram: Ram) -> Self {
        Self {
            cart,
            ram,
            vram: VRam::new(),
            io: Io::new(),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let location = BusAddrLocation::from(address);

        match location {
            BusAddrLocation::Oam  => {
                // TODO: Impl
                eprintln!("Can't BUS read {:?} address {:X}", location, address);
                0
            }
            BusAddrLocation::VRAM => self.vram.read(address),
            BusAddrLocation::RomBank0 | BusAddrLocation::RomBank1 | BusAddrLocation::CartRam => {
                self.cart.read(address)
            }
            BusAddrLocation::WRamBank0 | BusAddrLocation::WRamBank1To7 => self.ram.w_ram_read(address),
            BusAddrLocation::EchoRam | BusAddrLocation::Unusable => 0,
            BusAddrLocation::IoRegisters => self.io.read(address),
            BusAddrLocation::HRam => self.ram.h_ram_read(address),
            BusAddrLocation::IeRegister => self.io.interrupts.ie_register,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let location = BusAddrLocation::from(address);

        match location {
            BusAddrLocation::VRAM => self.vram.write(address, value),
            BusAddrLocation::Oam => {
                // TODO: IMPL
                eprintln!("Can't BUS write {:?} address {:X}", location, address)
            }
            BusAddrLocation::RomBank0 | BusAddrLocation::RomBank1 | BusAddrLocation::CartRam => {
                self.cart.write(address, value)
            }
            BusAddrLocation::WRamBank0 | BusAddrLocation::WRamBank1To7 => {
                self.ram.w_ram_write(address, value)
            }
            BusAddrLocation::EchoRam | BusAddrLocation::Unusable => {}
            BusAddrLocation::IoRegisters => self.io.write(address, value),
            BusAddrLocation::HRam => self.ram.h_ram_write(address, value),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ie_register() {
        let location: BusAddrLocation = 0xFFFF.into();

        assert_eq!(location, BusAddrLocation::IeRegister);
    }
}
