use crate::apu::channels::wave_channel::{CH3_WAVE_RAM_END, CH3_WAVE_RAM_START};
use crate::apu::Apu;
use crate::apu::{AUDIO_END_ADDRESS, AUDIO_START_ADDRESS};
use crate::auxiliary::joypad::Joypad;
use crate::auxiliary::timer::{Timer, TIMER_DIV_ADDRESS, TIMER_TAC_ADDRESS};
use crate::cpu::interrupts::Interrupts;
use crate::ppu::lcd::{Lcd, LCD_ADDRESS_END, LCD_ADDRESS_START};
use serde::{Deserialize, Serialize};

const IO_IF_UNUSED_MASK: u8 = 0b1110_0000;

// All unreadable bits of I/O registers return 1. In general, all unused bits in I/O registers are
// unreadable so they return 1. Some exceptions are:
// - Unknown purpose (if any) registers. Some bits of them can be read and written.
// - The IE register (only the 5 lower bits are used, but the upper 3 can hold any value).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Io {
    pub serial: Serial,
    pub timer: Timer,
    pub interrupts: Interrupts,
    pub lcd: Lcd,
    pub joypad: Joypad,
    pub apu: Apu,
}

impl Io {
    pub fn new(lcd: Lcd, apu: Apu) -> Self {
        Io {
            serial: Serial::default(),
            timer: Timer::default(),
            interrupts: Interrupts::default(),
            lcd,
            joypad: Default::default(),
            apu,
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.joypad.get_byte(),
            0xFF01 => self.serial.sb,
            0xFF02 => self.serial.sc | SERIAL_SC_UNUSED_MASK,
            TIMER_DIV_ADDRESS..=TIMER_TAC_ADDRESS => self.timer.read(address),
            AUDIO_START_ADDRESS..=AUDIO_END_ADDRESS |
            CH3_WAVE_RAM_START..=CH3_WAVE_RAM_END => self.apu.read(address),
            LCD_ADDRESS_START..=LCD_ADDRESS_END => self.lcd.read(address),
            0xFF4F | 0xFF50 | 0xFF51..=0xFF55 | 0xFF68..=0xFF6B | 0xFF70 => 0xFF,
            0xFF0F => self.interrupts.int_flags | IO_IF_UNUSED_MASK,
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => self.joypad.set_byte(value),
            0xFF01 => self.serial.sb = value,
            0xFF02 => self.serial.sc = value,
            TIMER_DIV_ADDRESS..=TIMER_TAC_ADDRESS => self.timer.write(address, value),
            AUDIO_START_ADDRESS..=AUDIO_END_ADDRESS |
            CH3_WAVE_RAM_START..=CH3_WAVE_RAM_END => self.apu.write(address, value),
            LCD_ADDRESS_START..=LCD_ADDRESS_END => self.lcd.write(address, value),
            0xFF4F |
            0xFF50 |
            0xFF51..=0xFF55 |
            0xFF68..=0xFF6B |
            0xFF70 => { },
            0xFF0F => self.interrupts.int_flags = value,
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Serial {
    /// FF01 — SB: Serial transfer data
    sb: u8,
    /// FF02 — SC: Serial transfer control
    sc: u8,
}

const SERIAL_SC_UNUSED_MASK: u8 = 0b01111110;

impl Serial {
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
