use crate::cart::header::{CartHeader, CartType, RamSize, RomSize};
use crate::cart::mbc::{Mbc, MbcVariant};

pub const RAM_ADDRESS_START: usize = 0xA000;
pub const RAM_SIZE: usize = 0x4000;
pub const ROM_BANK_SIZE: usize = 16 * 1024;
pub const RAM_BANK_SIZE: usize = 8 * 1024;
pub const MASK_MSB: u16 = 0xF000;

#[derive(Debug, Clone, Default)]
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
            match (address & MASK_MSB) >> 12 {
                0x0..=0x7 => mbc.read_rom(&self.data.bytes, address),
                0xA | 0xB => mbc.read_ram(address),
                _ => 0xFF,
            }
        } else {
            self.data.bytes[address as usize]
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if let Some(mbc) = &mut self.mbc {
            match (address & MASK_MSB) >> 12 {
                0x0..=0x7 => mbc.write_rom(&mut self.data.bytes, address, value),
                0xA | 0xB => mbc.write_ram(address, value),
                _ => (),
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
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
        let end = 0x014C;

        if self.bytes.len() < end {
            return 0;
        }

        let start = 0x0134;
        let mut checksum: u8 = 0;

        for &byte in &self.bytes[start..=end] {
            checksum = checksum.wrapping_sub(byte).wrapping_sub(1);
        }

        checksum
    }
}
