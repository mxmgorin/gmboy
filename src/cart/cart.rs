use crate::cart::controller::{new_controller, CartController, CartControllerType};
use crate::cart::header::CartHeader;
use std::fmt::Display;

pub const RAM_ADDRESS_START: usize = 0xA000;
pub const RAM_SIZE: usize = 0x4000;
pub const ROM_BANK_SIZE: usize = 16 * 1024;
pub const RAM_BANK_SIZE: usize = 8 * 1024;
pub const MASK_MSB: u16 = 0xF000;

#[derive(Debug, Clone)]
pub struct Cart {
    pub header: CartHeader,
    pub checksum_valid: bool,
    pub rom_bytes: Vec<u8>,
    pub controller: Option<CartControllerType>,
}

impl Cart {
    pub fn new(rom_bytes: Vec<u8>) -> Result<Cart, String> {
        let checksum = calc_checksum(&rom_bytes);
        let header = CartHeader::new(&rom_bytes)?;

        Ok(Self {
            checksum_valid: checksum == header.header_checksum,
            controller: new_controller(&header),
            header,
            rom_bytes,
        })
    }

    pub fn read(&self, address: u16) -> u8 {
        if let Some(controller) = &self.controller {
            match (address & MASK_MSB) >> 12 {
                0x0..=0x7 => controller.read_rom(&self.rom_bytes, address),
                0xA | 0xB => controller.read_ram(address),
                _ => 0xFF,
            }
        } else {
            self.rom_bytes[address as usize]
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        if let Some(controller) = &mut self.controller {
            match (address & MASK_MSB) >> 12 {
                0x0..=0x7 => controller.write_rom(&mut self.rom_bytes, address, value),
                0xA | 0xB => controller.write_ram(address, value),
                _ => (),
            }
        } else {
            self.rom_bytes[address as usize] = value;
        }
    }
}

fn calc_checksum(bytes: &[u8]) -> u8 {
    let end = 0x014C;

    if bytes.len() < end {
        return 0;
    }

    let start = 0x0134;
    let mut checksum: u8 = 0;

    for &byte in &bytes[start..=end] {
        checksum = checksum.wrapping_sub(byte).wrapping_sub(1);
    }

    checksum
}

impl Display for Cart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!(
            "Cart {{ header: {:?}, checksum_valid: {:?} }}",
            self.header, self.checksum_valid
        );
        write!(f, "{}", str)
    }
}
