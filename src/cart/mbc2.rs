use crate::mbc::{Mbc, MbcData};
use crate::{MASK_MSB, RAM_ADDRESS_START};

#[derive(Debug, Clone)]
pub struct Mbc2 {
    data: MbcData,
}

impl Mbc2 {
    pub fn new(data: MbcData) -> Self {
        Self { data }
    }
}

impl Mbc for Mbc2 {
    fn read_rom(&self, rom_bytes: &[u8], address: u16) -> u8 {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x3FFF (Bank 00)
            0x0..=0x3 => rom_bytes[address as usize],
            // 0x4000 - 0x7FFF (Bank 01-7F)
            0x4..=0x7 => {
                let offset = self.data.rom_offset * self.data.rom_bank as usize;
                rom_bytes[(address as usize - self.data.rom_offset) + offset]
            }
            _ => {
                eprintln!("Unknown address: {:#X}. Can't read byte.", address);

                0xFF
            }
        }
    }

    fn write_rom(&mut self, rom_bytes: &mut Vec<u8>, address: u16, value: u8) {
        match (address & MASK_MSB) >> 12 {
            // 0x0000 - 0x1FFF (RAM enable)
            0x0 | 0x1 => {
                if address & 0x100 != 0 {
                    return;
                }

                self.data.ram_enabled = value == 0x0A
            }
            // 0x2000 - 0x3FFF (ROM bank number)
            0x2 | 0x3 => {
                if address & 0x100 != 0x100 {
                    return;
                }

                let bank_number = if value == 0 { 1 } else { value };
                self.data.rom_bank = (bank_number & 0xF) as u16;
            }
            0x4..=0x7 => {}
            _ => eprintln!(
                "Unknown address: {:#X}. Can't write byte: {:#X}.",
                address, value
            ),
        }

        self.data.set_rom_bank(rom_bytes.len());
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
