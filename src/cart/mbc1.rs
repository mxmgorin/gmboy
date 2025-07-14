use crate::cart::mbc::{Mbc, MbcData};
use crate::mbc::{
    ROM_BANK_NON_ZERO_END_ADDR, ROM_BANK_NON_ZERO_START_ADDR, ROM_BANK_ZERO_END_ADDR,
    ROM_BANK_ZERO_START_ADDR,
};
use crate::{CartData, RAM_ADDRESS_START, RAM_BANK_SIZE, ROM_BANK_SIZE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Mode {
    RomBanking,
    RamBanking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mbc1 {
    data: MbcData,
    mode: Mode,
}

impl Mbc1 {
    pub fn new(inner: MbcData) -> Self {
        Self {
            data: inner,
            mode: Mode::RomBanking,
        }
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        match address {
            ROM_BANK_ZERO_START_ADDR..=ROM_BANK_ZERO_END_ADDR => cart_data.bytes[address as usize],
            ROM_BANK_NON_ZERO_START_ADDR..=ROM_BANK_NON_ZERO_END_ADDR => {
                let offset = ROM_BANK_SIZE * self.data.rom_bank as usize;
                cart_data.bytes[(address as usize - ROM_BANK_SIZE) + offset]
            }
            _ => 0xFF,
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // RAM enable
            0x0000..=0x1FFF => self.data.ram_enabled = value == 0x0A,
            // ROM bank number
            0x2000..=0x3FFF => {
                // Specify the lower 5 bits
                let bank_number = if value == 0 { 1 } else { value };
                self.data.rom_bank =
                    (self.data.rom_bank & 0b0110_0000) | (bank_number & 0b0001_1111) as u16;
            }
            // RAM bank number — or — upper bits of ROM bank number
            0x4000..=0x5FFF => match self.mode {
                Mode::RamBanking => self.data.ram_bank = value,
                Mode::RomBanking => self.data.rom_bank |= ((value & 0b0000_0011) << 5) as u16,
            },
            // Banking mode select
            0x6000..=0x7FFF => match value {
                0 => self.mode = Mode::RomBanking,
                1 => self.mode = Mode::RamBanking,
                _ => {}
            },
            _ => (),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.data.ram_enabled {
            return 0xFF;
        }

        let offset = RAM_BANK_SIZE * self.data.ram_bank as usize;

        self.data.ram_bytes[(address as usize - RAM_ADDRESS_START) + offset]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.data.ram_enabled {
            return;
        }

        let ram_offset = RAM_BANK_SIZE;
        let ram_bank = self.data.ram_bank;
        let offset = ram_offset * ram_bank as usize;

        self.data.ram_bytes[(address as usize - RAM_ADDRESS_START) + offset] = value;
    }

    fn load_ram(&mut self, ram_data: Vec<u8>) {
        self.data.ram_bytes = ram_data;
    }
}
