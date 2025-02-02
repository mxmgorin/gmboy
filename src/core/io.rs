impl TryFrom<u16> for IoAddrLocation {
    type Error = ();

    fn try_from(address: u16) -> Result<Self, Self::Error> {
        match address {
            0xFF01 => Ok(Self::SB),
            0xFF02 => Ok(Self::SC),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Io {
    pub serial: Serial,
}

impl Io {
    pub fn new() -> Io {
        Io {
            serial: Serial::new(),
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        let location = IoAddrLocation::try_from(address)
            .unwrap_or_else(|_| panic!("invalid IO address {:X}", address));

        match location {
            IoAddrLocation::SB => self.serial.sb,
            IoAddrLocation::SC => self.serial.sc,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let location = IoAddrLocation::try_from(address)
            .unwrap_or_else(|_| panic!("invalid IO address {:X}", address));

        match location {
            IoAddrLocation::SB => self.serial.sb = value,
            IoAddrLocation::SC => self.serial.sc = value,
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
pub enum IoAddrLocation {
    /// FF01 — SB: Serial transfer data
    SB,
    /// FF02 — SC: Serial transfer control
    SC,
}
