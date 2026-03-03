use crate::apu::channels::wave_channel::{CH3_WAVE_RAM_END, CH3_WAVE_RAM_START};
use crate::apu::Apu;
use crate::apu::{AUDIO_END_ADDRESS, AUDIO_START_ADDRESS};
use crate::auxiliary::joypad::Joypad;
use crate::auxiliary::ram::{Ram, WRAM_BANK_NUMBER_ADDR};
use crate::auxiliary::timer::{Timer, TIMER_DIV_ADDRESS, TIMER_TAC_ADDRESS};
use crate::cpu::interrupts::Interrupts;
use crate::emu::config::GbModel;
use crate::ppu::lcd::{
    CGB_OBJ_PRIORITY_MODE_ADDR, CGB_PALLETE_END_ADDR, CGB_PALLETE_START_ADDR, LCD_ADDRESS_END,
    LCD_ADDRESS_START,
};
use crate::ppu::vram::VRAM_BANK_NUMBER_ADDR;
use crate::ppu::Ppu;
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
    pub joypad: Joypad,
    pub apu: Apu,
    pub ppu: Ppu,
    pub ram: Ram,
}

impl Io {
    pub fn new(ppu: Ppu, apu: Apu) -> Self {
        Io {
            serial: Serial::default(),
            timer: Timer::default(),
            interrupts: Interrupts::default(),
            joypad: Default::default(),
            ram: Ram::default(),
            ppu,
            apu,
        }
    }

    #[inline(always)]
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => self.joypad.get_byte(),
            0xFF01 => self.serial.sb,
            0xFF02 => self.serial.sc | SERIAL_SC_UNUSED_MASK,
            TIMER_DIV_ADDRESS..=TIMER_TAC_ADDRESS => self.timer.read(addr),
            AUDIO_START_ADDRESS..=AUDIO_END_ADDRESS | CH3_WAVE_RAM_START..=CH3_WAVE_RAM_END => {
                self.apu.read(addr)
            }
            LCD_ADDRESS_START..=LCD_ADDRESS_END => self.ppu.lcd.read(addr),
            CGB_OBJ_PRIORITY_MODE_ADDR => match self.ppu.lcd.model {
                GbModel::Cgb => self.ppu.lcd.read_obj_priority_mode(),
                GbModel::Dmg => 0xFF,
            },
            VRAM_BANK_NUMBER_ADDR
            | 0xFF50
            | 0xFF51..=0xFF55
            | CGB_PALLETE_START_ADDR..=CGB_PALLETE_END_ADDR
            | WRAM_BANK_NUMBER_ADDR => match self.ppu.lcd.model {
                GbModel::Dmg => 0xFF,
                GbModel::Cgb => match addr {
                    VRAM_BANK_NUMBER_ADDR => self.ppu.video_ram.read_bank_number(),
                    WRAM_BANK_NUMBER_ADDR => self.ram.read_wram_bank(),
                    CGB_PALLETE_START_ADDR..=CGB_PALLETE_END_ADDR => {
                        self.ppu.lcd.cgb_palette.read(addr)
                    }
                    _ => 0xFF,
                },
            },
            0xFF0F => self.interrupts.int_flags | IO_IF_UNUSED_MASK,
            _ => 0xFF,
        }
    }

    #[inline(always)]
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF00 => self.joypad.set_byte(value),
            0xFF01 => self.serial.sb = value,
            0xFF02 => self.serial.sc = value,
            TIMER_DIV_ADDRESS..=TIMER_TAC_ADDRESS => self.timer.write(addr, value),
            AUDIO_START_ADDRESS..=AUDIO_END_ADDRESS | CH3_WAVE_RAM_START..=CH3_WAVE_RAM_END => {
                self.apu.write(addr, value)
            }
            LCD_ADDRESS_START..=LCD_ADDRESS_END => self.ppu.lcd.write(addr, value),
            CGB_OBJ_PRIORITY_MODE_ADDR => self.ppu.lcd.write_obj_priority_mode(value),
            VRAM_BANK_NUMBER_ADDR
            | 0xFF50
            | 0xFF51..=0xFF55
            | CGB_PALLETE_START_ADDR..=CGB_PALLETE_END_ADDR
            | WRAM_BANK_NUMBER_ADDR => match self.ppu.lcd.model {
                GbModel::Cgb => match addr {
                    VRAM_BANK_NUMBER_ADDR => self.ppu.video_ram.write_bank_number(value),
                    WRAM_BANK_NUMBER_ADDR => self.ram.write_wram_bank(value),
                    CGB_PALLETE_START_ADDR..=CGB_PALLETE_END_ADDR => {
                        self.ppu.lcd.cgb_palette.write(addr, value)
                    }
                    _ => {}
                },
                _ => {}
            },
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
    #[inline(always)]
    pub fn has_data(&self) -> bool {
        self.sc == 0x81
    }

    #[inline(always)]
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
