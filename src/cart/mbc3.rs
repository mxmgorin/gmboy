use crate::mbc::{Mbc, MbcData};
use crate::CartData;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use crate::header::{RamSize, RomSize};
use crate::mbc1::BankingMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mbc3 {
    data: MbcData,
    rtc: Rtc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rtc {
    selected_register: u8,
    latch_state_register: u8,
    registers: RtcRegisters,
    registers_latched: RtcRegisters,
    start: SystemTime,
}

impl Default for Rtc {
    fn default() -> Self {
        Self {
            selected_register: 0,
            start: SystemTime::UNIX_EPOCH,
            registers: Default::default(),
            registers_latched: Default::default(),
            latch_state_register: 0,
        }
    }
}

impl Rtc {
    pub fn is_selected(&self) -> bool {
        (0x08..=0x0C).contains(&self.selected_register)
    }

    pub fn reset_selected(&mut self) {
        self.selected_register = 0;
    }

    pub fn latch_clock(&mut self) {
        let now = SystemTime::now();
        let elapsed = now.duration_since(self.start).unwrap_or(Duration::ZERO);
        let elapsed_seconds = elapsed.as_secs();
        let days = (elapsed_seconds / 86400) as u16;

        self.registers.seconds = (elapsed_seconds % 60) as u8;
        self.registers.minutes = (elapsed_seconds / 60 % 60) as u8;
        self.registers.hours = (elapsed_seconds / 3600 % 24) as u8;
        self.registers.days_lower = (days & 0xFF) as u8; // Bit 0 = day high

        let mut flags = ((days >> 8) & 0x01) as u8;

        if self.registers.halted {
            flags |= 0x40;
        }

        if days > 511 {
            flags |= 0x80;
            self.registers.overflow = true;
        }

        self.registers.days_upper = flags;

        self.registers_latched = self.registers.clone();
    }
}

// TODO:
// When accessing the RTC Registers, it is recommended to wait 4 µs (4 M-cycles in Single Speed Mode) between any separate accesses.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RtcRegisters {
    pub seconds: u8,
    pub minutes: u8,
    pub hours: u8,
    // Upper 1 bit of Day Counter, Carry Bit, Halt Flag.
    // Bit 0: Most significant bit (Bit 8) of Day Counter
    // Bit 6: Halt (0=Active, 1=Stop Timer)
    // Bit 7: Day Counter Carry Bit (1=Counter Overflow)
    pub days_lower: u8,
    pub days_upper: u8,

    halted: bool,   // Bit 6 of RTC Control
    overflow: bool, // Bit 7 of RTC Control
}

impl RtcRegisters {
    pub fn read(&self, address: u8) -> u8 {
        match address {
            0x08 => self.seconds,
            0x09 => self.minutes,
            0x0A => self.hours,
            0x0B => self.days_lower,
            0x0C => {
                let mut val = self.days_upper & 0b0000_0001;

                if (self.days_upper & 0x10) != 0 {
                    val |= 0x01; // Bit 0: Day high
                }

                if self.halted {
                    val |= 0x40; // Bit 6: Halt flag
                }

                if self.overflow {
                    val |= 0x80; // Bit 7: Overflow flag
                }

                val
            }
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u8, value: u8) {
        match address {
            0x08 => self.seconds = value % 60,
            0x09 => self.minutes = value % 60,
            0x0A => self.hours = value % 24,
            0x0B => {
                self.days_lower = value;
            }
            0x0C => {
                self.days_upper = value & 0b1100_0001;

                self.halted = (value & 0x40) != 0;
                self.overflow = (value & 0x80) != 0;
            }
            _ => {}
        }
    }
}

impl Mbc3 {
    pub fn new(ram_size: RamSize, rom_size: RomSize) -> Self {
        Self {
            data: MbcData::new(vec![0; ram_size.bytes_size()], rom_size),
            rtc: Default::default(),
        }
    }
}

impl Mbc for Mbc3 {
    fn read_rom(&self, cart_data: &CartData, address: u16) -> u8 {
        self.data.read_rom(cart_data, address)
    }

    fn write_rom(&mut self, _cart_data: &CartData, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.data.write_ram_enabled(value),
            0x2000..=0x3FFF => {
                let bank_number = if value == 0 { 1 } else { value };
                self.data.rom_bank_number = bank_number as u16;
                self.data.clamp_rom_bank_number();
            }
            0x4000..=0x5FFF => {
                match value {
                    0x00..=0x03 => {
                        self.data.ram_bank_number = value & 0x03;
                        self.rtc.reset_selected();
                    },
                    0x08..=0x0C => self.rtc.selected_register = value,
                    _ => {}
                };
            }
            0x6000..=0x7FFF => {
                // Latch sequence: 0 -> 1 triggers latch
                if self.rtc.latch_state_register == 0 && value == 1 {
                    self.rtc.latch_clock();
                }

                self.rtc.latch_state_register = value;
            }
            _ => {}
        }
    }

    fn read_ram(&self, address: u16) -> u8 {
        if self.rtc.is_selected() {
            return self.rtc.registers_latched.read(self.rtc.selected_register);
        }

        self.data.read_ram(address, BankingMode::RamBanking)
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if self.rtc.is_selected() {
            return self.rtc.registers_latched.write(self.rtc.selected_register, value);
        }

        self.data.write_ram(address, value, BankingMode::RamBanking);
    }

    fn load_ram(&mut self, bytes: Vec<u8>) {
        self.data.load_ram(bytes);
    }

    fn dump_ram(&self) -> Option<Vec<u8>> {
        self.data.dump_ram()
    }
}
