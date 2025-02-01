use crate::core::cart::Cart;
use crate::core::ram::Ram;

#[derive(Debug, Clone)]
pub struct Bus {
    cart: Cart,
    ram: Ram,
}

const ROM_BANK0: AddressRange = AddressRange {
    start: 0x0000,
    end: 0x3FFF,
};
const ROM_BANK1: AddressRange = AddressRange {
    start: 0x4000,
    end: 0x7FFF,
};
const CHR_RAM: AddressRange = AddressRange {
    start: 0x8000,
    end: 0x97FF,
};
const BG_MAP1: AddressRange = AddressRange {
    start: 0x9800,
    end: 0x9BFF,
};
const BG_MAP2: AddressRange = AddressRange {
    start: 0x9C00,
    end: 0x9FFF,
};

const CART_RAM: AddressRange = AddressRange {
    start: 0xA000,
    end: 0xBFFF,
};
const RAM_BANK0: AddressRange = AddressRange {
    start: 0xC000,
    end: 0xCFFF,
};
const RAM_BANK1TO7: AddressRange = AddressRange {
    start: 0xD000,
    end: 0xDFFF,
};
const ECHO_RAM: AddressRange = AddressRange {
    start: 0xE000,
    end: 0xFDFF,
};
const OBJECT_ATTRIBUTE_MEMORY: AddressRange = AddressRange {
    start: 0xFE00,
    end: 0xFE9F,
};
const UNUSABLE: AddressRange = AddressRange {
    start: 0xFEA0,
    end: 0xFEFF,
};
const IO_REGISTERS: AddressRange = AddressRange {
    start: 0xFF00,
    end: 0xFF7F,
};
const ZERO_PAGE: AddressRange = AddressRange {
    start: 0xFF80,
    end: 0xFFFE,
};

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
pub struct AddressRange {
    pub start: u16,
    pub end: u16,
}

impl AddressRange {
    pub fn contains(&self, address: u16) -> bool {
        self.start <= address && address <= self.end
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