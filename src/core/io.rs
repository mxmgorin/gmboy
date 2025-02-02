use crate::core::timer::{Timer, TimerAddress};

impl TryFrom<u16> for IoAddress {
    type Error = ();

    fn try_from(address: u16) -> Result<Self, Self::Error> {
        const TIMER_START: u16 = TimerAddress::get_start();
        const TIMER_END: u16 = TimerAddress::get_end();

        match address {
            0xFF01 => Ok(Self::SB),
            0xFF02 => Ok(Self::SC),
            TIMER_START..=TIMER_END => Ok(Self::Timer(address.try_into()?)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Io {
    pub serial: Serial,
    pub timer: Timer,
}

impl Io {
    pub fn new() -> Io {
        Io {
            serial: Serial::new(),
            timer: Timer::new(),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let location = IoAddress::try_from(address)
            .unwrap_or_else(|_| panic!("invalid IO address {:X}", address));

        match location {
            IoAddress::SB => self.serial.sb,
            IoAddress::SC => self.serial.sc,
            IoAddress::Timer(address) => self.timer.read(address)
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let location = IoAddress::try_from(address)
            .unwrap_or_else(|_| panic!("invalid IO address {:X}", address));

        match location {
            IoAddress::SB => self.serial.sb = value,
            IoAddress::SC => self.serial.sc = value,
            IoAddress::Timer(address) => self.timer.write(address, value),
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
    /// FF01 — SB: Serial transfer data
    SB,
    /// FF02 — SC: Serial transfer control
    SC,
    Timer(TimerAddress),
}
