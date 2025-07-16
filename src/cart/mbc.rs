use crate::cart::mbc1::Mbc1;
use crate::mbc2::Mbc2;
use crate::mbc3::Mbc3;
use crate::mbc5::Mbc5;
use crate::{CartData, RAM_ADDRESS_START, RAM_BANK_SIZE, ROM_BANK_SIZE};
use serde::{Deserialize, Serialize};

pub const ROM_BANK_ZERO_START_ADDR: u16 = 0x0000;
pub const ROM_BANK_ZERO_END_ADDR: u16 = 0x3FFF;
pub const ROM_BANK_NON_ZERO_START_ADDR: u16 = 0x4000;
pub const ROM_BANK_NON_ZERO_END_ADDR: u16 = 0x7FFF;
pub const RAM_EXTERNAL_START_ADDR: u16 = 0xA000;
pub const RAM_EXTERNAL_END_ADDR: u16 = 0xBFFF;

pub trait Mbc {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8;
    fn write_rom(&mut self, _cart_data: &CartData, address: u16, value: u8);
    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
    fn load_ram(&mut self, bytes: Vec<u8>);
    fn dump_ram(&self) -> Option<Vec<u8>>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum MbcVariant {
    #[default]
    NoMbc,
    /// 8 KiB
    NoMbcRam(Vec<u8>),
    Mbc1(Mbc1),
    Mbc2(Mbc2),
    Mbc3(Mbc3),
    Mbc5(Mbc5),
}

impl Mbc for MbcVariant {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        match self {
            MbcVariant::NoMbc => cart_data.bytes[address as usize],
            MbcVariant::NoMbcRam(_c) => cart_data.bytes[address as usize],
            MbcVariant::Mbc1(c) => c.read_rom(cart_data, address),
            MbcVariant::Mbc2(c) => c.read_rom(cart_data, address),
            MbcVariant::Mbc3(c) => c.read_rom(cart_data, address),
            MbcVariant::Mbc5(c) => c.read_rom(cart_data, address),
        }
    }

    fn write_rom(&mut self, cart_data: &CartData, address: u16, value: u8) {
        match self {
            MbcVariant::NoMbc | MbcVariant::NoMbcRam(_) => {}
            MbcVariant::Mbc1(c) => c.write_rom(&cart_data, address, value),
            MbcVariant::Mbc2(c) => c.write_rom(&cart_data, address, value),
            MbcVariant::Mbc3(c) => c.write_rom(&cart_data, address, value),
            MbcVariant::Mbc5(c) => c.write_rom(&cart_data, address, value),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        match self {
            MbcVariant::NoMbc => 0xFF,
            MbcVariant::NoMbcRam(c) => c
                .get(address as usize - RAM_ADDRESS_START)
                .copied()
                .unwrap_or(0xFF),
            MbcVariant::Mbc1(c) => c.read_ram(address),
            MbcVariant::Mbc2(c) => c.read_ram(address),
            MbcVariant::Mbc3(c) => c.read_ram(address),
            MbcVariant::Mbc5(c) => c.read_ram(address),
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        match self {
            MbcVariant::NoMbc => {}
            MbcVariant::NoMbcRam(c) => c[address as usize - RAM_ADDRESS_START] = value,
            MbcVariant::Mbc1(c) => c.write_ram(address, value),
            MbcVariant::Mbc2(c) => c.write_ram(address, value),
            MbcVariant::Mbc3(c) => c.write_ram(address, value),
            MbcVariant::Mbc5(c) => c.write_ram(address, value),
        }
    }

    fn load_ram(&mut self, bytes: Vec<u8>) {
        match self {
            MbcVariant::NoMbc => {}
            MbcVariant::NoMbcRam(c) => *c = bytes,
            MbcVariant::Mbc1(c) => c.load_ram(bytes),
            MbcVariant::Mbc2(c) => c.load_ram(bytes),
            MbcVariant::Mbc3(c) => c.load_ram(bytes),
            MbcVariant::Mbc5(c) => c.load_ram(bytes),
        }
    }

    fn dump_ram(&self) -> Option<Vec<u8>> {
        match self {
            MbcVariant::NoMbc => None,
            MbcVariant::NoMbcRam(c) => Some(c.to_owned()),
            MbcVariant::Mbc1(c) => c.dump_ram(),
            MbcVariant::Mbc2(c) => c.dump_ram(),
            MbcVariant::Mbc3(c) => c.dump_ram(),
            MbcVariant::Mbc5(c) => c.dump_ram(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MbcData {
    pub ram_bytes: Vec<u8>,
    pub rom_bank_number: u16,
    pub ram_bank_number: u8,
    pub ram_enabled: bool,
}

impl MbcData {
    pub fn new(ram_bytes: Vec<u8>) -> Self {
        Self {
            ram_bytes,
            rom_bank_number: 1,
            ram_bank_number: 0,
            ram_enabled: false,
        }
    }

    pub fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        match address {
            ROM_BANK_ZERO_START_ADDR..=ROM_BANK_ZERO_END_ADDR => cart_data.bytes[address as usize],
            ROM_BANK_NON_ZERO_START_ADDR..=ROM_BANK_NON_ZERO_END_ADDR => {
                let offset = ROM_BANK_SIZE * self.rom_bank_number as usize;
                cart_data.bytes[(address as usize - ROM_BANK_SIZE) + offset]
            }
            _ => 0xFF,
        }
    }

    pub fn write_ram_enabled(&mut self, value: u8) {
        self.ram_enabled = value & 0xF == 0xA;
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        if self.ram_bytes.is_empty() {
            return 0xFF;
        }

        let offset = RAM_BANK_SIZE * self.ram_bank_number as usize;
        self.ram_bytes[(address as usize - RAM_ADDRESS_START) + offset]
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }

        if self.ram_bytes.is_empty() {
            return;
        }

        let offset = RAM_BANK_SIZE * self.ram_bank_number as usize;
        self.ram_bytes[(address as usize - RAM_ADDRESS_START) + offset] = value;
    }

    pub fn load_ram(&mut self, bytes: Vec<u8>) {
        self.ram_bytes = bytes;
    }

    pub fn dump_ram(&self) -> Option<Vec<u8>> {
        Some(self.ram_bytes.clone())
    }

    pub fn clamp_rom_bank_number(&mut self, cart_data: &CartData) {
        let max_banks = (cart_data.bytes.len() / ROM_BANK_SIZE).max(1);
        self.rom_bank_number = self.rom_bank_number % max_banks as u16;
    }
}
