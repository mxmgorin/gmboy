use crate::cart::mbc::{Mbc, MbcData};
use crate::mbc::BatterySave;
use crate::CartData;
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
        self.data.read_rom(cart_data, address)
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // RAM enable
            0x0000..=0x1FFF => self.data.write_ram_enabled(value),
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
        self.data.read_ram(address)
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.data.write_ram(address, value);
    }

    fn load_save(&mut self, save: BatterySave) {
        self.data.load_save(save);
    }

    fn dump_save(&self) -> Option<BatterySave> {
        self.data.dump_save()
    }
}
