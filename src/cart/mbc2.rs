use crate::mbc::{
    Mbc, MbcData, ROM_BANK_NON_ZERO_END_ADDR, ROM_BANK_NON_ZERO_START_ADDR, ROM_BANK_ZERO_END_ADDR,
    ROM_BANK_ZERO_START_ADDR,
};
use crate::{CartData, RAM_ADDRESS_START, ROM_BANK_SIZE};
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
        match address {
            ROM_BANK_ZERO_START_ADDR..=ROM_BANK_ZERO_END_ADDR => cart_data.bytes[address as usize],
            ROM_BANK_NON_ZERO_START_ADDR..=ROM_BANK_NON_ZERO_END_ADDR => {
                let offset = ROM_BANK_SIZE * self.data.rom_bank as usize;
                cart_data.bytes[(address as usize - ROM_BANK_SIZE) + offset]
            }
            _ => {
                eprintln!("Unknown address: {:#X}. Can't read byte.", address);

                0xFF
            }
        }
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        match address {
            // RAM enable
            0x0000..=0x1FFF => {
                if address & 0x100 != 0 {
                    return;
                }

                self.data.ram_enabled = value == 0x0A
            }
            // ROM bank number
            0x2000..=0x3FFF => {
                if address & 0x100 != 0x100 {
                    return;
                }

                let bank_number = if value == 0 { 1 } else { value };
                self.data.rom_bank = (bank_number & 0xF) as u16;
            }
            0x4000..=0x7FFF => {}
            _ => eprintln!(
                "Unknown address: {:#X}. Can't write byte: {:#X}.",
                address, value
            ),
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.data.ram_enabled {
            return 0xFF;
        }

        if self.data.ram_bytes.is_empty() {
            return 0xFF;
        }

        self.data.ram_bytes[address as usize - RAM_ADDRESS_START] & 0xF
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.data.ram_enabled {
            return;
        }

        if self.data.ram_bytes.is_empty() {
            return;
        }

        self.data.ram_bytes[address as usize - RAM_ADDRESS_START] = value & 0xF;
    }

    fn load_ram(&mut self, ram_data: Vec<u8>) {
        self.data.ram_bytes = ram_data;
    }
}
