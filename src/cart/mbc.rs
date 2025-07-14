use crate::cart::header::{CartType, RamSize};
use crate::cart::mbc1::Mbc1;
use crate::mbc2::Mbc2;
use crate::CartData;
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
}

impl MbcVariant {
    pub fn new(cart_data: &CartData) -> Option<MbcVariant> {
        let cart_type = cart_data.get_cart_type().unwrap();
        let ram_size = cart_data.get_ram_size().unwrap();

        let mbc_variant = match cart_type {
            CartType::RomOnly => return None,
            CartType::Mbc1 | CartType::Mbc1Ram => {
                MbcVariant::Mbc1(Mbc1::new(MbcData::new(ram_size, false)))
            }
            CartType::Mbc1RamBattery => MbcVariant::Mbc1(Mbc1::new(MbcData::new(ram_size, true))),
            CartType::Mbc2 => MbcVariant::Mbc2(Mbc2::new(MbcData::new(ram_size, false))),
            CartType::Mbc2Battery => MbcVariant::Mbc2(Mbc2::new(MbcData::new(ram_size, true))),
            CartType::RomRam
            | CartType::RomRamBattery
            | CartType::Mmm01
            | CartType::Mmm01Ram
            | CartType::Mmm01RamBattery
            | CartType::Mbc3TimerBattery
            | CartType::Mbc3TimerRamBattery
            | CartType::Mbc3
            | CartType::Mbc3Ram
            | CartType::Mbc3RamBattery
            | CartType::Mbc5
            | CartType::Mbc5Ram
            | CartType::Mbc5RamBattery
            | CartType::Mbc5Rumble
            | CartType::Mbc5RumbleRam
            | CartType::Mbc5RumbleRamBattery
            | CartType::PocketCamera
            | CartType::BandaiTama5
            | CartType::HuC3
            | CartType::HuC1RamBattery => unimplemented!("Cart type {:?}", cart_type),
        };

        Some(mbc_variant)
    }
}

impl Mbc for MbcVariant {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        match self {
            MbcVariant::Mbc1(c) => c.read_rom(cart_data, address),
            MbcVariant::Mbc2(c) => c.read_rom(cart_data, address),
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match self {
            MbcVariant::Mbc1(c) => c.write_rom(address, value),
            MbcVariant::Mbc2(c) => c.write_rom(address, value),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        match self {
            MbcVariant::Mbc1(c) => c.read_ram(address),
            MbcVariant::Mbc2(c) => c.read_ram(address),
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        match self {
            MbcVariant::Mbc1(c) => c.write_ram(address, value),
            MbcVariant::Mbc2(c) => c.write_ram(address, value),
        }
    }

    fn load_save(&mut self, save: BatterySave) {
        match self {
            MbcVariant::Mbc1(c) => c.load_save(save),
            MbcVariant::Mbc2(c) => c.load_save(save),
        }
    }

    fn dump_save(&self) -> Option<BatterySave> {
        match self {
            MbcVariant::Mbc1(c) => c.dump_save(),
            MbcVariant::Mbc2(c) => c.dump_save(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MbcData {
    pub ram_bytes: Vec<u8>,
    pub rom_bank: u16,
    pub ram_bank: u8,
    pub ram_enabled: bool,
    pub has_battery: bool,
}

impl MbcData {
    pub fn new(ram_size: RamSize, has_battery: bool) -> Self {
        Self {
            ram_bytes: vec![0; ram_size.bytes_size()],
            rom_bank: 1,
            ram_bank: 0,
            ram_enabled: false,
            has_battery,
        }
    }

    pub fn load_save(&mut self, save: BatterySave) {
        if self.has_battery {
            self.ram_bytes = save.ram_bytes;
        }
    }

    pub fn dump_save(&self) -> Option<BatterySave> {
        if self.has_battery {
            return Some(BatterySave::from_bytes(self.ram_bytes.clone()));
        }

        None
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

        let encoded: Vec<u8> = bincode::serialize(&self.ram_bytes).expect("Failed to serialize state");
        let mut file = File::create(path)?;
        file.write_all(&encoded)?;

        Ok(())
    }

    pub fn load(name: &str) -> std::io::Result<Self> {
        let path = Self::generate_path(name);
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let decoded: Vec<u8> = bincode::deserialize(&buffer).expect("Failed to deserialize state");

        Ok(Self {
            ram_bytes: decoded,
        })
    }

    pub fn generate_path(name: &str) -> PathBuf {
        let exe_path = env::current_exe().expect("Failed to get executable path");
        let exe_dir = exe_path
            .parent()
            .expect("Failed to get executable directory");

        exe_dir.join("saves").join(format!("{name}.sav"))
    }
}
