use crate::mbc::{Mbc, MbcData};
use crate::CartData;
use serde::{Deserialize, Serialize};
use crate::mbc1::BankingMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mbc5 {
    data: MbcData,
}

impl Mbc5 {
    pub fn new(data: MbcData) -> Self {
        Self { data }
    }
}

impl Mbc for Mbc5 {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        self.data.read_rom(cart_data, address)
    }

    fn write_rom(&mut self, _cart_data: &CartData, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.data.write_ram_enabled(value),
            0x2000..=0x2FFF => {
                // Set lower 8 bits of the ROM bank
                self.data.rom_bank_number = (self.data.rom_bank_number & 0x100) | value as u16;
                self.data.clamp_rom_bank_number();
            }
            0x3000..=0x3FFF => {
                // Set the 9th bit (bit 8)
                self.data.rom_bank_number =
                    (self.data.rom_bank_number & 0xFF) | ((value as u16 & 0x01) << 8);
                self.data.clamp_rom_bank_number();
            }
            0x4000..=0x5FFF => {
                // RAM bank select (only lower 4 bits used)
                self.data.ram_bank_number = value & 0x0F;
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.data.read_ram(address, BankingMode::RamBanking)
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
