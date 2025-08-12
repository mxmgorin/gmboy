use crate::cart::header::RomSize;
use crate::cart::mbc1::BankingMode;
use crate::cart::mbc1::Mbc1;
use crate::cart::mbc2::Mbc2;
use crate::cart::mbc3::Mbc3;
use crate::cart::mbc5::Mbc5;
use crate::cart::{CartData, RAM_ADDRESS_START, RAM_BANK_SIZE, ROM_BANK_SIZE};
use serde::{Deserialize, Serialize};

pub const ROM_BANK_ZERO_START_ADDR: u16 = 0x0000;
pub const ROM_BANK_ZERO_END_ADDR: u16 = 0x3FFF;
pub const ROM_BANK_NON_ZERO_START_ADDR: u16 = 0x4000;
pub const ROM_BANK_NON_ZERO_END_ADDR: u16 = 0x7FFF;
pub const RAM_EXTERNAL_START_ADDR: u16 = 0xA000;
pub const RAM_EXTERNAL_END_ADDR: u16 = 0xBFFF;

pub trait Mbc {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);
    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
    fn load_ram(&mut self, bytes: Box<[u8]>);
    fn dump_ram(&self) -> Option<Box<[u8]>>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum MbcVariant {
    #[default]
    NoMbc,
    /// 8 KiB
    NoMbcRam(Box<[u8]>),
    Mbc1(Mbc1),
    Mbc2(Mbc2),
    Mbc3(Mbc3),
    Mbc5(Mbc5),
}

impl Mbc for MbcVariant {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        match self {
            MbcVariant::NoMbc => cart_data.read(address as usize),
            MbcVariant::NoMbcRam(_c) => cart_data.read(address as usize),
            MbcVariant::Mbc1(c) => c.read_rom(cart_data, address),
            MbcVariant::Mbc2(c) => c.read_rom(cart_data, address),
            MbcVariant::Mbc3(c) => c.read_rom(cart_data, address),
            MbcVariant::Mbc5(c) => c.read_rom(cart_data, address),
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match self {
            MbcVariant::NoMbc | MbcVariant::NoMbcRam(_) => {}
            MbcVariant::Mbc1(c) => c.write_rom(address, value),
            MbcVariant::Mbc2(c) => c.write_rom(address, value),
            MbcVariant::Mbc3(c) => c.write_rom(address, value),
            MbcVariant::Mbc5(c) => c.write_rom(address, value),
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
            MbcVariant::NoMbcRam(c) => unsafe {
                // SAFETY: address is matched at the bus
                *c.get_unchecked_mut(address as usize - RAM_ADDRESS_START) = value;
            },
            MbcVariant::Mbc1(c) => c.write_ram(address, value),
            MbcVariant::Mbc2(c) => c.write_ram(address, value),
            MbcVariant::Mbc3(c) => c.write_ram(address, value),
            MbcVariant::Mbc5(c) => c.write_ram(address, value),
        }
    }

    fn load_ram(&mut self, bytes: Box<[u8]>) {
        match self {
            MbcVariant::NoMbc => {}
            MbcVariant::NoMbcRam(c) => *c = bytes,
            MbcVariant::Mbc1(c) => c.load_ram(bytes),
            MbcVariant::Mbc2(c) => c.load_ram(bytes),
            MbcVariant::Mbc3(c) => c.load_ram(bytes),
            MbcVariant::Mbc5(c) => c.load_ram(bytes),
        }
    }

    fn dump_ram(&self) -> Option<Box<[u8]>> {
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
    ram_bytes: Box<[u8]>,
    pub rom_bank_number: u16,
    pub ram_bank_number: u8,
    pub ram_enabled: bool,
    pub rom_banks_count: usize,
}

impl MbcData {
    pub fn new(ram_bytes: Box<[u8]>, rom_size: RomSize) -> Self {
        Self {
            ram_bytes,
            rom_bank_number: 1,
            ram_bank_number: 0,
            ram_enabled: false,
            rom_banks_count: rom_size.banks_count(),
        }
    }

    pub fn read_ram_byte(&self, index: usize) -> u8 {
        unsafe { *self.ram_bytes.get_unchecked(index) }
    }

    pub fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        match address {
            ROM_BANK_ZERO_START_ADDR..=ROM_BANK_ZERO_END_ADDR => cart_data.read(address as usize),
            ROM_BANK_NON_ZERO_START_ADDR..=ROM_BANK_NON_ZERO_END_ADDR => {
                let offset = ROM_BANK_SIZE * self.rom_bank_number as usize;
                let addr = (address as usize - ROM_BANK_SIZE) + offset;
                cart_data.read(addr)
            }
            _ => 0xFF,
        }
    }

    pub fn write_ram_enabled(&mut self, value: u8) {
        self.ram_enabled = value & 0xF == 0xA;
    }

    fn effective_ram_bank(&self, banking_mode: BankingMode) -> usize {
        if self.ram_bytes.len() <= RAM_BANK_SIZE {
            0
        } else if banking_mode == BankingMode::RomBanking {
            // Mode 0: ROM banking mode → always use RAM bank 0
            0
        } else {
            // Mode 1: RAM banking mode → use selected RAM bank, clamped
            (self.ram_bank_number as usize) % (self.ram_bytes.len() / RAM_BANK_SIZE)
        }
    }

    pub fn read_ram(&self, address: u16, banking_mode: BankingMode) -> u8 {
        if !self.ram_enabled || self.ram_bytes.is_empty() {
            return 0xFF;
        }

        let offset = RAM_BANK_SIZE * self.effective_ram_bank(banking_mode);
        let index = (address as usize - RAM_ADDRESS_START) + offset;

        if index < self.ram_bytes.len() {
            self.read_ram_byte(index)
        } else {
            0xFF
        }
    }

    pub fn write_ram(&mut self, address: u16, value: u8, banking_mode: BankingMode) {
        if !self.ram_enabled || self.ram_bytes.is_empty() {
            return;
        }

        let offset = RAM_BANK_SIZE * self.effective_ram_bank(banking_mode);
        let index = (address as usize - RAM_ADDRESS_START) + offset;

        if index < self.ram_bytes.len() {
            unsafe {
                *self.ram_bytes.get_unchecked_mut(index) = value;
            }
        }
    }

    pub fn load_ram(&mut self, bytes: Box<[u8]>) {
        self.ram_bytes = bytes;
    }

    pub fn dump_ram(&self) -> Option<Box<[u8]>> {
        Some(self.ram_bytes.clone())
    }

    pub fn clamp_rom_bank_number(&mut self) {
        self.rom_bank_number %= self.rom_banks_count as u16;
    }
}
