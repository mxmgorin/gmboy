use crate::mbc::{Mbc, MbcData};
use crate::{CartData};
use serde::{Deserialize, Serialize};
use crate::header::{RomSize};
use crate::mbc1::BankingMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mbc2 {
    data: MbcData,
}

impl Mbc2 {
    pub fn new(rom_size: RomSize) -> Self {
        Self {
            data: MbcData::new(vec![0; 512 * 4], rom_size),
        }
    }
}

impl Mbc for Mbc2 {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        self.data.read_rom(cart_data, address)
    }

    fn write_rom(&mut self, _cart_data: &CartData, address: u16, value: u8) {
        match address {
            0x0000..=0x3FFF => {
                if address & 0x100 == 0 {
                    self.data.ram_enabled = value & 0xF == 0xA;
                } else {
                    self.data.rom_bank_number = match (value as u16) & 0x0F {
                        0 => 1,
                        n => n,
                    };
                    self.data.clamp_rom_bank_number();
                }
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.data.ram_enabled {
            return 0xFF;
        }

        let address = (address as usize) & 0x1FF; // wrap every 512 bytes
        self.data.ram_bytes[address] | 0xF0
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.data.write_ram(address, value, BankingMode::RamBanking);
    }

    fn load_ram(&mut self, bytes: Vec<u8>) {
        self.data.load_ram(bytes);
    }

    fn dump_ram(&self) -> Option<Vec<u8>> {
        self.data.dump_ram()
    }
}
