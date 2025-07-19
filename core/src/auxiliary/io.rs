use crate::apu::channels::wave_channel::{CH3_WAVE_RAM_END, CH3_WAVE_RAM_START};
use crate::apu::Apu;
use crate::apu::{AUDIO_END_ADDRESS, AUDIO_START_ADDRESS};
use crate::auxiliary::joypad::Joypad;
use crate::auxiliary::timer::{Timer, TIMER_DIV_ADDRESS, TIMER_TAC_ADDRESS};
use crate::cpu::interrupts::Interrupts;
use crate::ppu::lcd::{Lcd, LCD_ADDRESS_END, LCD_ADDRESS_START};
use serde::{Deserialize, Serialize};

const IO_IF_UNUSED_MASK: u8 = 0b1110_0000;

impl From<u16> for IoAddress {
    fn from(address: u16) -> Self {
        match address {
            0xFF00 => Self::Joypad,
            0xFF01 => Self::SerialSb,
            0xFF02 => Self::SerialSc,
            TIMER_DIV_ADDRESS..=TIMER_TAC_ADDRESS => Self::Timer,
            AUDIO_START_ADDRESS..=AUDIO_END_ADDRESS => Self::Audio,
            CH3_WAVE_RAM_START..=CH3_WAVE_RAM_END => Self::WavePattern,
            LCD_ADDRESS_START..=LCD_ADDRESS_END => Self::Display,
            0xFF4F => Self::VRAMBankSelect,
            0xFF50 => Self::DisableBootROM,
            0xFF51..=0xFF55 => Self::VRAMdma,
            0xFF68..=0xFF6B => Self::Background,
            0xFF70 => Self::WRAMBankSelect,
            0xFF0F => Self::InterruptFlags,
            _ => Self::Unused,
        }
    }
}

// All unreadable bits of I/O registers return 1. In general, all unused bits in I/O registers are
// unreadable so they return 1. Some exceptions are:
// - Unknown purpose (if any) registers. Some bits of them can be read and written.
// - The IE register (only the 5 lower bits are used, but the upper 3 can hold any value).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Io {
    pub serial: Serial,
    pub timer: Timer,
    pub interrupts: Interrupts,
    pub lcd: Lcd,
    pub joypad: Joypad,
    pub apu: Apu,
}

impl Default for Io {
    fn default() -> Self {
        Io {
            serial: Serial::new(),
            timer: Timer::default(),
            interrupts: Interrupts::new(),
            lcd: Lcd::default(),
            joypad: Default::default(),
            apu: Default::default(),
        }
    }
}

impl Io {
    pub fn read(&self, address: u16) -> u8 {
        let location = IoAddress::from(address);

        match location {
            IoAddress::SerialSb => self.serial.sb,
            IoAddress::SerialSc => self.serial.sc | SERIAL_SC_UNUSED_MASK,
            IoAddress::Timer => self.timer.read(address),
            IoAddress::InterruptFlags => self.interrupts.int_flags | IO_IF_UNUSED_MASK,
            IoAddress::Display => self.lcd.read(address),
            IoAddress::Joypad => self.joypad.get_byte(),
            IoAddress::Audio | IoAddress::WavePattern => self.apu.read(address),
            IoAddress::Unused
            | IoAddress::VRAMBankSelect
            | IoAddress::DisableBootROM
            | IoAddress::VRAMdma
            | IoAddress::Background
            | IoAddress::WRAMBankSelect => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let location = IoAddress::from(address);

        match location {
            IoAddress::SerialSb => self.serial.sb = value,
            IoAddress::SerialSc => self.serial.sc = value,
            IoAddress::Timer => self.timer.write(address, value),
            IoAddress::InterruptFlags => self.interrupts.int_flags = value,
            IoAddress::Display => self.lcd.write(address, value),
            IoAddress::Joypad => self.joypad.set_byte(value),
            IoAddress::Audio | IoAddress::WavePattern => self.apu.write(address, value),
            IoAddress::Unused
            | IoAddress::VRAMBankSelect
            | IoAddress::DisableBootROM
            | IoAddress::VRAMdma
            | IoAddress::Background
            | IoAddress::WRAMBankSelect => {}
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Serial {
    /// FF01 — SB: Serial transfer data
    sb: u8,
    /// FF02 — SC: Serial transfer control
    sc: u8,
}
impl Default for Serial {
    fn default() -> Self {
        Self::new()
    }
}

const SERIAL_SC_UNUSED_MASK: u8 = 0b01111110;

impl Serial {
    pub fn new() -> Serial {
        Self { sb: 0, sc: 0 }
    }

    pub fn has_data(&self) -> bool {
        self.sc == 0x81
    }

    pub fn take_data(&mut self) -> u8 {
        self.sc = 0;

        self.sb
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IoAddress {
    Joypad,
    /// FF01 — SB: Serial transfer data
    SerialSb,
    /// FF02 — SC: Serial transfer control
    SerialSc,
    Timer,
    InterruptFlags,
    Audio,
    WavePattern,
    Display,
    VRAMBankSelect,
    DisableBootROM,
    VRAMdma,
    Background,
    WRAMBankSelect,
    Unused,
}
