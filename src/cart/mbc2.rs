use crate::mbc::{BatterySave, Mbc, MbcData};
use crate::{CartData};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mbc2 {
    data: MbcData,
}

impl Mbc2 {
    pub fn new(data: MbcData) -> Self {
        Self { data }
    }
}

impl Mbc for Mbc2 {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        self.data.read_rom(cart_data, address)
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // RAM enable
            0x0000..=0x1FFF => {
                if address & 0x100 != 0 {
                    return;
                }

                self.data.write_ram_enabled(value);
            }
            // ROM bank number
            0x2000..=0x3FFF => {
                if address & 0x100 != 0x100 {
                    return;
                }

                let bank_number = if value == 0 { 1 } else { value };
                self.data.rom_bank_number = (bank_number & 0xF) as u16;
            }
            0x4000..=0x7FFF => {}
            _ => eprintln!(
                "Unknown address: {:#X}. Can't write byte: {:#X}.",
                address, value
            ),
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
