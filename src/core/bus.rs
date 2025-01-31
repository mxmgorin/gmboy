use crate::core::cart::Cart;
use std::cmp::Ordering;

const MEMORY_RANGES: [AddressRange; 13] = [
    AddressRange {
        start: 0x0000,
        end: 0x3FFF,
        memory_type: MemoryType::RomBank0,
    },
    AddressRange {
        start: 0x4000,
        end: 0x7FFF,
        memory_type: MemoryType::RomBank1,
    },
    AddressRange {
        start: 0x8000,
        end: 0x97FF,
        memory_type: MemoryType::ChrRam,
    },
    AddressRange {
        start: 0x9800,
        end: 0x9BFF,
        memory_type: MemoryType::BgMap1,
    },
    AddressRange {
        start: 0x9C00,
        end: 0x9FFF,
        memory_type: MemoryType::BgMap2,
    },
    AddressRange {
        start: 0xA000,
        end: 0xBFFF,
        memory_type: MemoryType::CartridgeRam,
    },
    AddressRange {
        start: 0xC000,
        end: 0xCFFF,
        memory_type: MemoryType::RamBank0,
    },
    AddressRange {
        start: 0xD000,
        end: 0xDFFF,
        memory_type: MemoryType::RamBank1To7,
    },
    AddressRange {
        start: 0xE000,
        end: 0xFDFF,
        memory_type: MemoryType::EchoRam,
    },
    AddressRange {
        start: 0xFE00,
        end: 0xFE9F,
        memory_type: MemoryType::ObjectAttributeMemory,
    },
    AddressRange {
        start: 0xFEA0,
        end: 0xFEFF,
        memory_type: MemoryType::Unusable,
    },
    AddressRange {
        start: 0xFF00,
        end: 0xFF7F,
        memory_type: MemoryType::IoRegisters,
    },
    AddressRange {
        start: 0xFF80,
        end: 0xFFFE,
        memory_type: MemoryType::ZeroPage,
    },
];

#[derive(Debug, Clone)]
pub struct Bus {
    cart: Cart,
}

impl Bus {
    pub fn new(cart: Cart) -> Self {
        Self { cart }
    }

    pub fn read(&self, address: u16) -> Result<u8, String> {
        let memory_type = get_memory_type(address);

        let Some(memory_type) = memory_type else {
            return Err(format!("Can't read: invalid memory address {address}"));
        };

        match memory_type {
            MemoryType::RomBank0 | MemoryType::RomBank1 | MemoryType::ChrRam => {
                return Ok(self.cart.read(address))
            }
            MemoryType::BgMap1 => {}
            MemoryType::BgMap2 => {}
            MemoryType::CartridgeRam => {}
            MemoryType::RamBank0 => {}
            MemoryType::RamBank1To7 => {}
            MemoryType::EchoRam => {}
            MemoryType::ObjectAttributeMemory => {}
            MemoryType::Unusable => {}
            MemoryType::IoRegisters => {}
            MemoryType::ZeroPage => {}
        }

        unimplemented!("Bus read not implemented yet");
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), String> {
        let memory_type = get_memory_type(address);

        let Some(memory_type) = memory_type else {
            return Err(format!("Can't read: invalid memory address {address}"));
        };

        match memory_type {
            MemoryType::RomBank0 | MemoryType::RomBank1 | MemoryType::ChrRam => {
                self.cart.write(address, value);
                return Ok(());
            }
            MemoryType::BgMap1 => {}
            MemoryType::BgMap2 => {}
            MemoryType::CartridgeRam => {}
            MemoryType::RamBank0 => {}
            MemoryType::RamBank1To7 => {}
            MemoryType::EchoRam => {}
            MemoryType::ObjectAttributeMemory => {}
            MemoryType::Unusable => {}
            MemoryType::IoRegisters => {}
            MemoryType::ZeroPage => {}
        }

        unimplemented!("Bus read not implemented yet");
    }

    pub fn read16(&self, address: u16) -> Result<u16, String> {
        let lo = self.read(address)? as u16;
        let hi = self.read(address + 1)? as u16;

        Ok(lo | (hi << 8))
    }

    pub fn write16(&mut self, address: u16, value: u16) -> Result<(), String> {
        self.write(address + 1, ((value >> 8) & 0xFF) as u8)?;
        self.write(address, (value & 0xFF) as u8)?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressRange {
    pub start: u16,
    pub end: u16,
    pub memory_type: MemoryType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    RomBank0,
    RomBank1,
    ChrRam,
    BgMap1,
    BgMap2,
    CartridgeRam,
    RamBank0,
    RamBank1To7,
    EchoRam,
    ObjectAttributeMemory,
    Unusable,
    IoRegisters,
    ZeroPage,
}

impl Ord for AddressRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for AddressRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_memory_type(address: u16) -> Option<MemoryType> {
    match MEMORY_RANGES.binary_search_by(|range| {
        if address < range.start {
            Ordering::Less
        } else if address > range.end {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }) {
        Ok(index) => Some(MEMORY_RANGES[index].memory_type),
        Err(_) => None,
    }
}
