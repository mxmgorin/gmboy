use crate::cart::header::{CartHeader, CartType, RamSize, RomSize};
use crate::cart::mbc::{Mbc, MbcVariant};
use crate::mbc::{
    MbcData, RAM_EXTERNAL_END_ADDR, RAM_EXTERNAL_START_ADDR, ROM_BANK_NON_ZERO_END_ADDR,
    ROM_BANK_ZERO_START_ADDR,
};
use crate::mbc1::Mbc1;
use crate::mbc2::Mbc2;
use crate::mbc3::Mbc3;
use crate::mbc5::Mbc5;
use serde::{Deserialize, Serialize};

pub const RAM_ADDRESS_START: usize = 0xA000;
pub const RAM_SIZE: usize = 0x4000;
pub const ROM_BANK_SIZE: usize = 16 * 1024;
pub const RAM_BANK_SIZE: usize = 8 * 1024;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cart {
    pub data: CartData,
    pub has_battery: bool,
    pub mbc: MbcVariant,
}

impl Cart {
    pub fn new(rom_bytes: Vec<u8>) -> Result<Cart, String> {
        let data = CartData::new(rom_bytes);
        let cart_type = data.get_cart_type()?;
        let ram_size = data.get_ram_size()?;
        let ram_bytes = vec![0; ram_size.bytes_size()];

        let mbc = match cart_type {
            CartType::RomOnly => MbcVariant::NoMbc,
            CartType::RomRam | CartType::RomRamBattery => MbcVariant::NoMbcRam(ram_bytes),
            CartType::Mbc1 | CartType::Mbc1Ram | CartType::Mbc1RamBattery => {
                MbcVariant::Mbc1(Mbc1::new(MbcData::new(ram_bytes)))
            }
            CartType::Mbc2 | CartType::Mbc2Battery => {
                MbcVariant::Mbc2(Mbc2::new(MbcData::new(ram_bytes)))
            }
            CartType::Mbc5
            | CartType::Mbc5Ram
            | CartType::Mbc5Rumble
            | CartType::Mbc5RumbleRam
            | CartType::Mbc3
            | CartType::Mbc3Ram
            | CartType::Mbc5RamBattery
            | CartType::Mbc5RumbleRamBattery => {
                MbcVariant::Mbc5(Mbc5::new(MbcData::new(ram_bytes)))
            }
            CartType::Mbc3RamBattery
            | CartType::Mbc3TimerBattery
            | CartType::Mbc3TimerRamBattery => MbcVariant::Mbc3(Mbc3::new(MbcData::new(ram_bytes))),
            CartType::Mmm01
            | CartType::Mmm01Ram
            | CartType::Mmm01RamBattery
            | CartType::PocketCamera
            | CartType::BandaiTama5
            | CartType::HuC3
            | CartType::HuC1RamBattery => unimplemented!("Cart type {:?}", cart_type),
        };

        Ok(Self {
            data,
            has_battery: cart_type.has_battery(),
            mbc,
        })
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            ROM_BANK_ZERO_START_ADDR..=ROM_BANK_NON_ZERO_END_ADDR => {
                self.mbc.read_rom(&self.data, address)
            }
            RAM_EXTERNAL_START_ADDR..=RAM_EXTERNAL_END_ADDR => self.mbc.read_ram(address),
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            ROM_BANK_ZERO_START_ADDR..=ROM_BANK_NON_ZERO_END_ADDR => {
                self.mbc.write_rom(address, value)
            }
            RAM_EXTERNAL_START_ADDR..=RAM_EXTERNAL_END_ADDR => self.mbc.write_ram(address, value),
            _ => (),
        }
    }

    pub fn load_ram(&mut self, bytes: Vec<u8>) {
        if self.has_battery {
            self.mbc.load_ram(bytes);
        }
    }

    pub fn dump_ram(&self) -> Option<Vec<u8>> {
        if self.has_battery {
            return self.mbc.dump_ram();
        }

        None
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CartData {
    pub bytes: Vec<u8>,
}

impl CartData {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    pub fn get_title(&self) -> String {
        CartHeader::parse_title(&self.bytes)
    }

    pub fn get_cart_type(&self) -> Result<CartType, String> {
        CartHeader::parse_cart_type(&self.bytes)
    }

    pub fn get_rom_size(&self) -> Result<RomSize, String> {
        CartHeader::parse_rom_size(&self.bytes)
    }

    pub fn get_ram_size(&self) -> Result<RamSize, String> {
        CartHeader::parse_ram_size(&self.bytes)
    }

    pub fn get_rom_version(&self) -> u8 {
        CartHeader::get_rom_version(&self.bytes)
    }

    pub fn checksum_valid(&self) -> bool {
        let checksum = self.calc_checksum();

        CartHeader::get_header_checksum(self.bytes.as_slice()) == checksum
    }

    pub fn calc_checksum(&self) -> u8 {
        const END: usize = 0x014C;

        if self.bytes.len() < END {
            return 0;
        }

        const START: usize = 0x0134;
        let mut checksum: u8 = 0;

        for &byte in &self.bytes[START..=END] {
            checksum = checksum.wrapping_sub(byte).wrapping_sub(1);
        }

        checksum
    }
}
