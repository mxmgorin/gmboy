use crate::cart::mbc::{Mbc, MbcData};
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
}

impl Mbc1 {
    pub fn new(inner: MbcData) -> Self {
        Self {
            data: inner,
            banking_mode: BankingMode::RomBanking,
        }
    }

    pub fn get_effective_rom_bank_number(&self, cart_data: &CartData, address: u16) -> u8 {
        if address < 0x4000 {
            // --- Fixed bank area ---
            match self.banking_mode {
                BankingMode::RomBanking => 0,
                BankingMode::RamBanking => {
                    // In RAM banking mode, upper bits (bits 5–6) affect the 0x0000 area
                    if cart_data.bytes.len() >= 1024 * 1024 {
                        (self.data.ram_bank_number & 0b0000_0011) << 5
                    } else {
                        0
                    }
                }
            }
        } else {
            // --- Switchable bank area ---
            let mut bank = self.data.rom_bank_number as u8;

            if bank & 0b0001_1111 == 0 {
                bank = 1; // Bank 0 is never mapped here
            }

            // Combine with upper bits (bits 5–6 from 0x4000–0x5FFF writes)
            bank |= self.data.ram_bank_number << 5;

            bank & Mbc1::get_rom_bank_mask(cart_data)
        }
    }

    pub fn get_rom_bank_mask(cart_data: &CartData) -> u8 {
        let rom_bank_count = (cart_data.bytes.len() / ROM_BANK_SIZE).max(1);
        let required_bits = (usize::BITS - (rom_bank_count - 1).leading_zeros()) as u8;
        let mask = (1 << required_bits) - 1;

        mask as u8
    }

    pub fn get_rom_address(address: u16, bank_number: u8) -> usize {
        (address as usize & 0x3FFF) + (bank_number as usize * ROM_BANK_SIZE)
    }
}

impl Mbc for Mbc1 {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        let bank = self.get_effective_rom_bank_number(cart_data, address);
        let address = Mbc1::get_rom_address(address, bank);

        cart_data.bytes.get(address).copied().unwrap_or(0xFF)
    }

    fn write_rom(&mut self, _cart_data: &CartData, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.data.write_ram_enabled(value),
            0x2000..=0x3FFF => {
                let value = if value == 0 { 1 } else { value };
                self.data.rom_bank_number = value as u16 & 0b0001_1111;
            }
            0x4000..=0x5FFF => match self.banking_mode {
                BankingMode::RamBanking => self.data.ram_bank_number = value & 0b0000_0011,
                BankingMode::RomBanking => {
                    self.data.rom_bank_number |= (value as u16 & 0b0000_0011) << 5;
                }
            },
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

    fn load_ram(&mut self, bytes: Vec<u8>) {
        self.data.load_ram(bytes);
    }

    fn dump_ram(&self) -> Option<Vec<u8>> {
        self.data.dump_ram()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::mbc::MbcData;
    use crate::mbc1::{BankingMode, Mbc1};
    use crate::{CartData, ROM_BANK_SIZE};

    #[test]
    pub fn test_get_rom_bank_mask_256kib() {
        let cart_date = CartData::new(vec![0; 1024 * 256]);
        let mask = Mbc1::get_rom_bank_mask(&cart_date);

        assert_eq!(mask, 0b00001111);
    }

    #[test]
    pub fn test_get_rom_bank_mask_512kib() {
        let cart_date = CartData::new(vec![0; 1024 * 512]); // 4 Mbit
        let mask = Mbc1::get_rom_bank_mask(&cart_date);

        assert_eq!(mask, 0b00011111);
    }

    #[test]
    pub fn test_effective_rom_bank_number_0x40000() {
        let cart_date = CartData::new(vec![0; ROM_BANK_SIZE * 50]);
        let mut mbc = Mbc1::new(MbcData::new(vec![0; ROM_BANK_SIZE]));
        mbc.data.rom_bank_number = 0b10010;
        mbc.data.ram_bank_number = 0b01;

        let effective_rom_bank_number = mbc.get_effective_rom_bank_number(&cart_date, 0x4000);

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
        let cart_date = CartData::new(vec![0; ROM_BANK_SIZE * 1024]);
        let mut mbc = Mbc1::new(MbcData::new(vec![0; ROM_BANK_SIZE]));
        mbc.data.rom_bank_number = 0b10010;
        mbc.data.ram_bank_number = 0b01;
        mbc.banking_mode = BankingMode::RomBanking;

        let effective_rom_bank_number = mbc.get_effective_rom_bank_number(&cart_date, 0x0000);

        assert_eq!(effective_rom_bank_number, 0b0000000);

        mbc.banking_mode = BankingMode::RamBanking;

        let effective_rom_bank_number = mbc.get_effective_rom_bank_number(&cart_date, 0x0000);

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
        let cart_date = CartData::new(vec![0; ROM_BANK_SIZE * 1024]);
        let mut mbc = Mbc1::new(MbcData::new(vec![0; ROM_BANK_SIZE]));
        mbc.data.rom_bank_number = 0b00100;
        mbc.data.ram_bank_number = 0b10;
        mbc.banking_mode = BankingMode::RamBanking;
        let address = 0x72A7;

        let effective_rom_bank_number = mbc.get_effective_rom_bank_number(&cart_date, address);
        let address = Mbc1::get_rom_address(address, effective_rom_bank_number);

        assert_eq!(effective_rom_bank_number, 0b1000100);
        assert_eq!(address, 0x1132A7)
    }
}
