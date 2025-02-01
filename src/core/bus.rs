use crate::core::cart::Cart;
use crate::core::ram::Ram;

#[derive(Debug, Clone)]
pub struct Bus {
    cart: Cart,
    ram: Ram,
}

const ROM_BANK0: AddrRange = AddrRange(0x0000, 0x3FFF);
const ROM_BANK1: AddrRange = AddrRange(0x4000, 0x7FFF);
const CHR_RAM: AddrRange = AddrRange(0x8000, 0x97FF);
const BG_MAP1: AddrRange = AddrRange(0x9800, 0x9BFF);
const BG_MAP2: AddrRange = AddrRange(0x9C00, 0x9FFF);
const CART_RAM: AddrRange = AddrRange(0xA000, 0xBFFF);
const RAM_BANK0: AddrRange = AddrRange(0xC000, 0xCFFF);
const RAM_BANK1TO7: AddrRange = AddrRange(0xD000, 0xDFFF);
const ECHO_RAM: AddrRange = AddrRange(0xE000, 0xFDFF);
const OBJECT_ATTRIBUTE_MEMORY: AddrRange = AddrRange(0xFE00, 0xFE9F);
const UNUSABLE: AddrRange = AddrRange(0xFEA0, 0xFEFF);
const IO_REGISTERS: AddrRange = AddrRange(0xFF00, 0xFF7F);
const ZERO_PAGE: AddrRange = AddrRange(0xFF80, 0xFFFE);

impl Bus {
    pub fn new(cart: Cart, ram: Ram) -> Self {
        Self { cart, ram }
    }

    pub fn read(&self, address: u16) -> u8 {
        if ROM_BANK0.contains(address) || ROM_BANK1.contains(address) {
            return self.cart.read(address);
        }

        panic!("Can't bus read read address {address}");
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if ROM_BANK0.contains(address) || ROM_BANK1.contains(address) {
            self.cart.write(address, value);
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

#[derive(Debug, Clone)]
pub struct AddrRange(u16, u16);

impl AddrRange {
    pub fn contains(&self, address: u16) -> bool {
        self.0 <= address && address <= self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        assert!(ROM_BANK0.contains(0x100));

        //assert!(RAM_BANK1.contains(0x100));
        //assert!(RAM_BANK0.contains(0x100));
    }
}
