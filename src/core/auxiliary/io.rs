use crate::auxiliary::timer::{Timer, TIMER_DIV_ADDRESS, TIMER_TAC_ADDRESS};
use crate::cpu::interrupts::Interrupts;
use crate::ppu::lcd::{Lcd, LCD_ADDRESS_END, LCD_ADDRESS_START};

impl TryFrom<u16> for IoAddressLocation {
    type Error = ();

    fn try_from(address: u16) -> Result<Self, Self::Error> {
        match address {
            0xFF00 => Ok(Self::Joypad),
            0xFF01 => Ok(Self::SerialSb),
            0xFF02 => Ok(Self::SerialSc),
            TIMER_DIV_ADDRESS..=TIMER_TAC_ADDRESS => Ok(Self::Timer),
            0xFF10..=0xFF26 => Ok(Self::Audio),
            0xFF30..=0xFF3F => Ok(Self::WavePattern),
            LCD_ADDRESS_START..=LCD_ADDRESS_END => Ok(Self::Display),
            0xFF4F => Ok(Self::VRAMBankSelect),
            0xFF50 => Ok(Self::DisableBootROM),
            0xFF51..=0xFF55 => Ok(Self::VRAMdma),
            0xFF68..=0xFF6B => Ok(Self::Background),
            0xFF70 => Ok(Self::WRAMBankSelect),
            0xFF0F => Ok(Self::InterruptFlags),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Io {
    pub serial: Serial,
    pub timer: Timer,
    pub interrupts: Interrupts,
    pub lcd: Lcd,
}

impl Default for Io {
    fn default() -> Self {
        Io {
            serial: Serial::new(),
            timer: Timer::default(),
            interrupts: Interrupts::new(),
            lcd: Lcd::default(),
        }
    }
}

impl Io {
    pub fn read(&self, address: u16) -> u8 {
        let location = IoAddressLocation::try_from(address);

        let Ok(location) = location else {
            //#[cfg(not(test))]
            //eprintln!("Can't IO read address {:X}", address);
            return 0;
        };

        match location {
            IoAddressLocation::SerialSb => self.serial.sb,
            IoAddressLocation::SerialSc => self.serial.sc,
            IoAddressLocation::Timer => self.timer.read(address),
            IoAddressLocation::InterruptFlags => self.interrupts.int_flags,
            IoAddressLocation::Display => self.lcd.read(address),
            IoAddressLocation::Joypad
            | IoAddressLocation::Audio
            | IoAddressLocation::WavePattern
            | IoAddressLocation::VRAMBankSelect
            | IoAddressLocation::DisableBootROM
            | IoAddressLocation::VRAMdma
            | IoAddressLocation::Background
            | IoAddressLocation::WRAMBankSelect => {
                // TODO: Impl
                //#[cfg(not(test))]
                //eprintln!("Can't IO read address {:?} {:X}", location, address);

                0
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let location = IoAddressLocation::try_from(address);

        let Ok(location) = location else {
            //#[cfg(not(test))]
            //eprintln!("Can't IO write address {:X}", address);
            return;
        };

        match location {
            IoAddressLocation::SerialSb => self.serial.sb = value,
            IoAddressLocation::SerialSc => self.serial.sc = value,
            IoAddressLocation::Timer => self.timer.write(address, value),
            IoAddressLocation::InterruptFlags => self.interrupts.int_flags = value,
            IoAddressLocation::Display => self.lcd.write(address, value),
            IoAddressLocation::Joypad
            | IoAddressLocation::Audio
            | IoAddressLocation::WavePattern
            | IoAddressLocation::VRAMBankSelect
            | IoAddressLocation::DisableBootROM
            | IoAddressLocation::VRAMdma
            | IoAddressLocation::Background
            | IoAddressLocation::WRAMBankSelect => {
                // TODO: Impl
                //#[cfg(not(test))]
                //eprintln!("Can't IO write {:?} address {:X}", location, address);
            }
        }
    }
}

#[derive(Debug, Clone)]
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
pub enum IoAddressLocation {
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
}
