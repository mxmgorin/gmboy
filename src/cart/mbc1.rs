use serde::{Deserialize, Serialize};
use crate::cart::mbc::{Mbc, MbcData};
use crate::{MASK_MSB, RAM_ADDRESS_START};

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
    fn read_rom(&self, rom_bytes: &[u8], address: u16) -> u8 {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x3FFF (Bank 00)
            0x0..=0x3 => rom_bytes[address as usize],
            // 0x4000 - 0x7FFF (Bank 01-7F)
            0x4..=0x7 => {
                let offset = self.data.rom_offset * self.data.rom_bank as usize;
                rom_bytes[(address as usize - self.data.rom_offset) + offset]
            }
            _ => 0xFF,
        }
    }

    fn write_rom(&mut self, rom_data: &mut Vec<u8>, address: u16, value: u8) {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x1FFF (RAM enable)
            0x0 | 0x1 => self.data.ram_enabled = value == 0x0A,
            // 0x2000 - 0x3FFF (ROM bank number)
            0x2 | 0x3 => {
                // Specify the lower 5 bits
                let bank_number = if value == 0 { 1 } else { value };
                self.data.rom_bank =
                    (self.data.rom_bank & 0b0110_0000) | (bank_number & 0b0001_1111) as u16;
            }
            // 0x4000 - 0x5FFF (RAM bank number — or — upper bits of ROM bank number)
            0x4 | 0x5 => match self.mode {
                Mode::RamBanking => self.data.ram_bank = value,
                Mode::RomBanking => self.data.rom_bank |= ((value & 0b0000_0011) << 5) as u16,
            },
            // 0x6000 - 0x7FFF (Banking mode select)
            0x6 | 0x7 => match value {
                0 => self.mode = Mode::RomBanking,
                1 => self.mode = Mode::RamBanking,
                _ => {}
            },
            _ => (),
        }

        self.data.set_rom_bank(rom_data.len());
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.data.ram_enabled {
            return 0xFF;
        }

        let offset = self.data.ram_offset * self.data.ram_bank as usize;

        self.data.ram_bytes[(address as usize - RAM_ADDRESS_START) + offset]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.data.ram_enabled {
            return;
        }

        let ram_offset = self.data.ram_offset;
        let ram_bank = self.data.ram_bank;
        let offset = ram_offset * ram_bank as usize;

        self.data.ram_bytes[(address as usize - RAM_ADDRESS_START) + offset] = value;
    }

    fn load_ram(&mut self, ram_data: Vec<u8>) {
        self.data.ram_bytes = ram_data;
    }
}
