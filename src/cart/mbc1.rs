use crate::cart::mbc::{Mbc, MbcData};
use crate::CartData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BankingMode {
    RomBanking,
    RamBanking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mbc1 {
    data: MbcData,
    banking_mode: BankingMode,
}

impl Mbc1 {
    pub fn new(inner: MbcData) -> Self {
        Self {
            data: inner,
            banking_mode: BankingMode::RomBanking,
        }
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        self.data.read_rom(cart_data, address)
    }

    fn write_rom(&mut self, cart_data: &CartData, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.data.write_ram_enabled(value),
            0x2000..=0x3FFF => {
                let bank_number = if value == 0 { 1 } else { value };
                self.data.rom_bank_number =
                    (self.data.rom_bank_number & 0b0110_0000) | (bank_number & 0b0001_1111) as u16;
                self.data.clamp_rom_bank_number(cart_data);
            }
            // RAM bank number — or — upper bits of ROM bank number
            0x4000..=0x5FFF => match self.banking_mode {
                BankingMode::RamBanking => self.data.ram_bank_number = value,
                BankingMode::RomBanking => {
                    self.data.rom_bank_number |= ((value & 0b0000_0011) << 5) as u16;
                    self.data.clamp_rom_bank_number(cart_data);
                },
            },
            0x6000..=0x7FFF => match value {
                0 => self.banking_mode = BankingMode::RomBanking,
                1 => self.banking_mode = BankingMode::RamBanking,
                _ => {}
            },
            _ => (),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.data.read_ram(address, self.banking_mode)
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.data.write_ram(address, value, self.banking_mode);
    }

    fn load_ram(&mut self, bytes: Vec<u8>) {
        self.data.load_ram(bytes);
    }

    fn dump_ram(&self) -> Option<Vec<u8>> {
        self.data.dump_ram()
    }
}
