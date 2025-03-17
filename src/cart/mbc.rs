use crate::cart::header::{CartType, RamSize};
use crate::cart::mbc1::Mbc1;
use crate::{CartData, RAM_BANK_SIZE, ROM_BANK_SIZE};

pub trait Mbc {
    fn read_rom(&self, rom_bytes: &[u8], address: u16) -> u8;
    fn write_rom(&mut self, rom_bytes: &mut Vec<u8>, address: u16, value: u8);
    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
    fn load_ram(&mut self, ram_bytes: Vec<u8>);
}

#[derive(Debug, Clone)]
pub enum MbcVariant {
    Mbc1(Mbc1),
}

impl MbcVariant {
    pub fn new(cart_data: &CartData) -> Option<MbcVariant> {
        let cart_type = cart_data.get_cart_type().unwrap();
        match cart_type {
            CartType::RomOnly => None,
            CartType::Mbc1 | CartType::Mbc1Ram | CartType::Mbc1RamBattery => Some(
                MbcVariant::Mbc1(Mbc1::new(MbcData::new(cart_data.get_ram_size().unwrap()))),
            ),
            CartType::Mbc2
            | CartType::Mbc2Battery
            | CartType::RomRam
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
        }
    }
}

impl Mbc for MbcVariant {
    fn read_rom(&self, rom_bytes: &[u8], address: u16) -> u8 {
        match self {
            MbcVariant::Mbc1(c) => c.read_rom(rom_bytes, address),
        }
    }

    fn write_rom(&mut self, rom_bytes: &mut Vec<u8>, address: u16, value: u8) {
        match self {
            MbcVariant::Mbc1(c) => c.write_rom(rom_bytes, address, value),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        match self {
            MbcVariant::Mbc1(c) => c.read_ram(address),
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        match self {
            MbcVariant::Mbc1(c) => c.write_ram(address, value),
        }
    }

    fn load_ram(&mut self, ram_bytes: Vec<u8>) {
        match self {
            MbcVariant::Mbc1(c) => c.load_ram(ram_bytes),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MbcData {
    pub ram_bytes: Vec<u8>,
    pub rom_bank: u16,
    pub ram_bank: u8,
    pub rom_offset: usize,
    pub ram_offset: usize,
    pub ram_enabled: bool,
}

impl MbcData {
    pub fn new(ram_size: RamSize) -> Self {
        Self {
            ram_bytes: vec![0; ram_size.bytes_size()],
            rom_bank: 1,
            ram_bank: 0,
            rom_offset: ROM_BANK_SIZE,
            ram_offset: RAM_BANK_SIZE,
            ram_enabled: false,
        }
    }
}
