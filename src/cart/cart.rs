use crate::cart::header::{CartHeader, CartType, RamSize, RomSize};
use crate::cart::mbc::{Mbc, MbcVariant};
use crate::mbc::{
    RAM_EXTERNAL_END_ADDR, RAM_EXTERNAL_START_ADDR, ROM_BANK_NON_ZERO_END_ADDR,
    ROM_BANK_ZERO_START_ADDR,
};
use serde::{Deserialize, Serialize};

pub const RAM_ADDRESS_START: usize = 0xA000;
pub const RAM_SIZE: usize = 0x4000;
pub const ROM_BANK_SIZE: usize = 16 * 1024;
pub const RAM_BANK_SIZE: usize = 8 * 1024;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cart {
    pub data: CartData,
    pub mbc: Option<MbcVariant>,
}

impl Cart {
    pub fn new(rom_bytes: Vec<u8>) -> Result<Cart, String> {
        let data = CartData::new(rom_bytes);

        Ok(Self {
            mbc: MbcVariant::new(&data),
            data,
        })
    }

    pub fn read(&self, address: u16) -> u8 {
        if let Some(mbc) = &self.mbc {
            match address {
                ROM_BANK_ZERO_START_ADDR..=ROM_BANK_NON_ZERO_END_ADDR => {
                    mbc.read_rom(&self.data, address)
                }
                RAM_EXTERNAL_START_ADDR..=RAM_EXTERNAL_END_ADDR => mbc.read_ram(address),
                _ => 0xFF,
            }
        } else {
            self.data.bytes[address as usize]
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if let Some(mbc) = &mut self.mbc {
            match address {
                ROM_BANK_ZERO_START_ADDR..=ROM_BANK_NON_ZERO_END_ADDR => {
                    mbc.write_rom(address, value)
                }
                RAM_EXTERNAL_START_ADDR..=RAM_EXTERNAL_END_ADDR => mbc.write_ram(address, value),
                _ => (),
            }
        }
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

    pub fn get_title(&self) -> Result<String, String> {
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
