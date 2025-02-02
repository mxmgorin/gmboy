use crate::core::timer::{Timer, TimerAddress};
use crate::core::Interrupts;

impl TryFrom<u16> for IoAddress {
    type Error = ();

    fn try_from(address: u16) -> Result<Self, Self::Error> {
        const TIMER_START: u16 = TimerAddress::get_start();
        const TIMER_END: u16 = TimerAddress::get_end();

        match address {
            0xFF00 => Ok(Self::Joypad),
            0xFF01 => Ok(Self::SerialSb),
            0xFF02 => Ok(Self::SerialSc),
            TIMER_START..=TIMER_END => Ok(Self::Timer(address.try_into()?)),
            0xFF10..=0xFF26 => Ok(Self::Audio),
            0xFF30..=0xFF3F => Ok(Self::WavePattern),
            0xFF40..=0xFF4B => Ok(Self::Display),
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
}

impl Io {
    pub fn new() -> Io {
        Io {
            serial: Serial::new(),
            timer: Timer::new(),
            interrupts: Interrupts::new(),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let address_type = IoAddress::try_from(address)
            .unwrap_or_else(|_| panic!("invalid IO address {:X}", address));

        match address_type {
            IoAddress::SerialSb => self.serial.sb,
            IoAddress::SerialSc => self.serial.sc,
            IoAddress::Timer(address) => self.timer.read(address),
            IoAddress::InterruptFlags => self.interrupts.int_flags,
            IoAddress::Joypad
            | IoAddress::Audio
            | IoAddress::WavePattern
            | IoAddress::Display
            | IoAddress::VRAMBankSelect
            | IoAddress::DisableBootROM
            | IoAddress::VRAMdma
            | IoAddress::Background
            | IoAddress::WRAMBankSelect => {
                // TODO: Impl
                eprintln!("Can't IO read address {:?} {}", address_type, address);

                0
            }
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let address_type = IoAddress::try_from(address)
            .unwrap_or_else(|_| panic!("invalid IO address {:X}", address));

        match address_type {
            IoAddress::SerialSb => self.serial.sb = value,
            IoAddress::SerialSc => self.serial.sc = value,
            IoAddress::Timer(address) => self.timer.write(address, value),
            IoAddress::InterruptFlags => self.interrupts.int_flags = value,
            IoAddress::Joypad
            | IoAddress::Audio
            | IoAddress::WavePattern
            | IoAddress::Display
            | IoAddress::VRAMBankSelect
            | IoAddress::DisableBootROM
            | IoAddress::VRAMdma
            | IoAddress::Background
            | IoAddress::WRAMBankSelect => {
                // TODO: Impl
                eprintln!("Can't IO write address {:?} {}", address_type, address);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Serial {
    sb: u8,
    sc: u8,
}

impl Serial {
    pub fn new() -> Serial {
        Self { sb: 0, sc: 0 }
    }

    pub fn has_data(&self) -> bool {
        if self.sc == 0x81 {
            return true;
        }

        false
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
    Timer(TimerAddress),
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
