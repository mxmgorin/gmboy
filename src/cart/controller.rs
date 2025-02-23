use crate::cart::header::{CartHeader, CartType};
use crate::cart::mbc1::Mbc1;
use crate::{RAM_BANK_SIZE, ROM_BANK_SIZE};

pub trait CartController {
    fn read_rom(&self, rom_data: &[u8], address: u16) -> u8;
    fn write_rom(&mut self, rom_data: &mut Vec<u8>, address: u16, value: u8);
    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
    fn load(&mut self, ram_data: Vec<u8>);
}

pub enum CartControllerType {
    Mbc1(Mbc1),
}

impl CartController for CartControllerType {
    fn read_rom(&self, rom_bytes: &[u8], address: u16) -> u8 {
        match self {
            CartControllerType::Mbc1(c) => c.read_rom(rom_bytes, address),
        }
    }

    fn write_rom(&mut self, rom_bytes: &mut Vec<u8>, address: u16, value: u8) {
        match self {
            CartControllerType::Mbc1(c) => c.write_rom(rom_bytes, address, value),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        match self {
            CartControllerType::Mbc1(c) => c.read_ram(address),
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        match self {
            CartControllerType::Mbc1(c) => c.write_ram(address, value),
        }
    }

    fn load(&mut self, ram_bytes: Vec<u8>) {
        match self {
            CartControllerType::Mbc1(c) => c.load(ram_bytes),
        }
    }
}

pub fn new_controller(header: &CartHeader) -> Option<CartControllerType> {
    match header.cart_type {
        CartType::RomOnly => None,
        CartType::Mbc1 | CartType::Mbc1Ram | CartType::Mbc1RamBattery => Some(
            CartControllerType::Mbc1(Mbc1::new(ControllerData::new(header))),
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
        | CartType::HuC1RamBattery => unimplemented!(),
    }
}

pub struct ControllerData {
    pub ram_bytes: Vec<u8>,
    pub rom_bank: u16,
    pub ram_bank: u8,
    pub rom_offset: usize,
    pub ram_offset: usize,
    pub ram_enabled: bool,
}

impl ControllerData {
    pub fn new(header: &CartHeader) -> Self {
        let rom_bank = 1;
        let ram_bank = 0;
        let rom_offset: usize = ROM_BANK_SIZE;
        let ram_offset: usize = RAM_BANK_SIZE;
        let ram_enabled = false;

        Self {
            ram_bytes: vec![0; header.ram_size.bytes_size()],
            rom_bank,
            ram_bank,
            rom_offset,
            ram_offset,
            ram_enabled,
        }
    }
}
