use crate::cart::mbc1::Mbc1;
use crate::mbc2::Mbc2;
use crate::mbc3::Mbc3;
use crate::mbc5::Mbc5;
use crate::{CartData, RAM_ADDRESS_START, RAM_BANK_SIZE, ROM_BANK_SIZE};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

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
    fn load_save(&mut self, save: BatterySave);
    fn dump_save(&self) -> Option<BatterySave>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MbcVariant {
    Mbc1(Mbc1),
    Mbc2(Mbc2),
    Mbc3(Mbc3),
    Mbc5(Mbc5),
}

impl Mbc for MbcVariant {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        match self {
            MbcVariant::Mbc1(c) => c.read_rom(cart_data, address),
            MbcVariant::Mbc2(c) => c.read_rom(cart_data, address),
            MbcVariant::Mbc3(c) => c.read_rom(cart_data, address),
            MbcVariant::Mbc5(c) => c.read_rom(cart_data, address),
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match self {
            MbcVariant::Mbc1(c) => c.write_rom(address, value),
            MbcVariant::Mbc2(c) => c.write_rom(address, value),
            MbcVariant::Mbc3(c) => c.write_rom(address, value),
            MbcVariant::Mbc5(c) => c.write_rom(address, value),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        match self {
            MbcVariant::Mbc1(c) => c.read_ram(address),
            MbcVariant::Mbc2(c) => c.read_ram(address),
            MbcVariant::Mbc3(c) => c.read_ram(address),
            MbcVariant::Mbc5(c) => c.read_ram(address),
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        match self {
            MbcVariant::Mbc1(c) => c.write_ram(address, value),
            MbcVariant::Mbc2(c) => c.write_ram(address, value),
            MbcVariant::Mbc3(c) => c.write_ram(address, value),
            MbcVariant::Mbc5(c) => c.write_ram(address, value),
        }
    }

    fn load_save(&mut self, save: BatterySave) {
        match self {
            MbcVariant::Mbc1(c) => c.load_save(save),
            MbcVariant::Mbc2(c) => c.load_save(save),
            MbcVariant::Mbc3(c) => c.load_save(save),
            MbcVariant::Mbc5(c) => c.load_save(save),
        }
    }

    fn dump_save(&self) -> Option<BatterySave> {
        match self {
            MbcVariant::Mbc1(c) => c.dump_save(),
            MbcVariant::Mbc2(c) => c.dump_save(),
            MbcVariant::Mbc3(c) => c.dump_save(),
            MbcVariant::Mbc5(c) => c.dump_save(),
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
        self.ram_enabled = value == 0x0A;
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

    pub fn load_save(&mut self, save: BatterySave) {
        self.ram_bytes = save.ram_bytes;
    }

    pub fn dump_save(&self) -> Option<BatterySave> {
        Some(BatterySave::from_bytes(self.ram_bytes.clone()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatterySave {
    pub ram_bytes: Vec<u8>,
}

impl BatterySave {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { ram_bytes: bytes }
    }
    pub fn save(&self, name: &str) -> std::io::Result<()> {
        let path = Self::generate_path(name);

        if let Some(parent) = Path::new(&path).parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = File::create(path)?;
        file.write_all(&self.ram_bytes)?;

        Ok(())
    }

    pub fn load(name: &str) -> std::io::Result<Self> {
        let path = Self::generate_path(name);
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        Ok(Self { ram_bytes: buffer })
    }

    pub fn generate_path(name: &str) -> PathBuf {
        let exe_path = env::current_exe().expect("Failed to get executable path");
        let exe_dir = exe_path
            .parent()
            .expect("Failed to get executable directory");

        exe_dir.join("saves").join(format!("{name}.sav"))
    }
}
