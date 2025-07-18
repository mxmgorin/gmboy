use crate::cart::mbc::{Mbc, MbcData};
use crate::header::{RamSize, RomSize};
use crate::{CartData, ROM_BANK_SIZE};
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
    is_multicart: bool,
}

impl Mbc1 {
    pub fn new(ram_size: RamSize, rom_size: RomSize, rom_bytes: &[u8]) -> Self {
        Self {
            data: MbcData::new(vec![0; ram_size.bytes_size()].into_boxed_slice(), rom_size),
            banking_mode: BankingMode::RomBanking,
            is_multicart: Mbc1::is_multicart(rom_bytes),
        }
    }

    fn is_multicart(rom_bytes: &[u8]) -> bool {
        // Only 8 Mbit MBC1 multicarts exist
        if rom_bytes.len() != 1_048_576 {
            return false;
        }

        let nintendo_logo_count = (0..4)
            .map(|page| {
                let start = page * 0x40000 + 0x0104;
                let end = start + 0x30;

                crc::crc32::checksum_ieee(&rom_bytes[start..end])
            })
            .filter(|&checksum| checksum == 0x4619_5417)
            .count();

        // A multicart should have at least two games + a menu with valid logo data
        nintendo_logo_count >= 3
    }

    pub fn get_effective_rom_bank_number(&self, address: u16) -> u8 {
        let bank = if address < 0x4000 {
            match self.banking_mode {
                BankingMode::RomBanking => 0,
                BankingMode::RamBanking => {
                    if self.is_multicart {
                        self.data.ram_bank_number << 4
                    } else {
                        self.data.ram_bank_number << 5
                    }
                }
            }
        } else {
            if self.is_multicart {
                (self.data.rom_bank_number as u8 & 0b1111) | self.data.ram_bank_number << 4
            } else {
                self.data.rom_bank_number as u8 | self.data.ram_bank_number << 5
            }
        };

        bank
    }

    pub fn get_rom_bank_mask(&self) -> u8 {
        let required_bits = (usize::BITS - (self.data.rom_banks_count - 1).leading_zeros()) as u8;
        let mask = (1 << required_bits) - 1;

        mask as u8
    }

    pub fn get_rom_address(address: u16, bank_number: u8) -> usize {
        (address as usize & 0x3FFF) | (bank_number as usize * ROM_BANK_SIZE)
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        let bank = self.get_effective_rom_bank_number(address);
        let address = Mbc1::get_rom_address(address, bank) & (cart_data.bytes.len() - 1);

        cart_data.bytes[address]
    }

    fn write_rom(&mut self, _cart_data: &CartData, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.data.write_ram_enabled(value),
            0x2000..=0x3FFF => {
                let value = value & 0b0001_1111;
                let value = if value == 0 { 1 } else { value };
                self.data.rom_bank_number = value as u16;
            }
            0x4000..=0x5FFF => self.data.ram_bank_number = value & 0b0000_0011,
            0x6000..=0x7FFF => match value & 0x0000_0001 {
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

    fn load_ram(&mut self, bytes: Box<[u8]>) {
        self.data.load_ram(bytes);
    }

    fn dump_ram(&self) -> Option<Box<[u8]>> {
        self.data.dump_ram()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::header::{RamSize, RomSize};
    use crate::mbc1::{BankingMode, Mbc1};

    #[test]
    pub fn test_get_rom_bank_mask_256kib() {
        let mbc = Mbc1::new(RamSize::Ram32KiB, RomSize::Rom256KiB, &vec![]);
        let mask = mbc.get_rom_bank_mask();

        assert_eq!(mask, 0b00001111);
    }

    #[test]
    pub fn test_get_rom_bank_mask_512kib() {
        let mbc = Mbc1::new(RamSize::Ram32KiB, RomSize::Rom512KiB, &vec![]);
        let mask = mbc.get_rom_bank_mask();

        assert_eq!(mask, 0b00011111);
    }

    #[test]
    pub fn test_effective_rom_bank_number_0x40000() {
        let rom_size = RomSize::Rom1MiB;
        let ram_size = RamSize::Ram8KiB;
        let mut mbc = Mbc1::new(ram_size, rom_size, &vec![]);
        mbc.data.rom_bank_number = 0b10010;
        mbc.data.ram_bank_number = 0b01;

        let effective_rom_bank_number = mbc.get_effective_rom_bank_number(0x4000);

        assert_eq!(effective_rom_bank_number, 0b0110010);
    }

    #[test]
    pub fn test_effective_rom_bank_number_0x00000() {
        // ROM banking example 1
        // Let’s assume we have previously written 0x12 to the BANK1 register and 0b01 to the BANK2
        // register. The effective bank number during ROM reads depends on which address range we
        // read and on the value of the MODE register:
        // Value of the BANK 1 register 0b10010
        // Value of the BANK 2 register 0b01
        // Effective ROM bank number
        // (reading 0x4000-0x7FFF) 0b0110010 (= 50 = 0x32)
        // Effective ROM bank number
        // (reading 0x0000-0x3FFF, MODE = 0b0) 0b0000000 (= 0 = 0x00)
        // Effective ROM bank number
        // (reading 0x0000-0x3FFF, MODE = 0b1) 0b
        let rom_size = RomSize::Rom1MiB;
        let ram_size = RamSize::Ram8KiB;
        let mut mbc = Mbc1::new(ram_size, rom_size, &vec![]);
        mbc.data.rom_bank_number = 0b10010;
        mbc.data.ram_bank_number = 0b01;
        mbc.banking_mode = BankingMode::RomBanking;

        let effective_rom_bank_number = mbc.get_effective_rom_bank_number(0x0000);

        assert_eq!(effective_rom_bank_number, 0b0000000);

        mbc.banking_mode = BankingMode::RamBanking;

        let effective_rom_bank_number = mbc.get_effective_rom_bank_number(0x0000);

        assert_eq!(effective_rom_bank_number, 0b0100000);
    }

    #[test]
    pub fn test_effective_rom_bank_number_0x72a7_rom_banking_mode() {
        // ROM banking example 2
        // Let’s assume we have previously requested ROM bank number 68, MBC1 mode is 0b0, and we
        // are now reading a byte from 0x72A7. The actual physical ROM address that will be read is going
        // to be 0x1132A7 and is constructed in the following way:
        // Value of the BANK 1 register 0b00100
        // Value of the BANK 2 register 0b10
        // ROM bank number 0b1000100 (= 68 = 0x44)
        // Address being read 0b0111 0010 1010 0111 (= 0x72A7)
        // Actual physical ROM address 0b1 0001 0011 0010 1010 0111 (= 0x1132A7)
        let rom_size = RomSize::Rom4MiB;
        let ram_size = RamSize::Ram8KiB;
        let mut mbc = Mbc1::new(ram_size, rom_size, &vec![]);
        mbc.data.rom_bank_number = 0b00100;
        mbc.data.ram_bank_number = 0b10;
        mbc.banking_mode = BankingMode::RamBanking;
        let address = 0x72A7;

        let effective_rom_bank_number = mbc.get_effective_rom_bank_number(address);
        let address = Mbc1::get_rom_address(address, effective_rom_bank_number);

        assert_eq!(effective_rom_bank_number, 0b1000100);
        assert_eq!(address, 0x1132A7)
    }
}
