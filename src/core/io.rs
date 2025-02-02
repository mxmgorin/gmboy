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
    serial: [u8; 2],
}

impl Io {
    pub fn new() -> Io {
        Io { serial: [0; 2] }
    }

    pub fn read(&self, address: u16) -> u8 {
        let location = IoAddrLocation::try_from(address).expect("invalid IO address");

        match location {
            IoAddrLocation::SB => self.serial[0],
            IoAddrLocation::SC => self.serial[1],
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        let location = IoAddrLocation::try_from(address).expect("invalid IO address");

        match location {
            IoAddrLocation::SB => self.serial[0] = value,
            IoAddrLocation::SC => self.serial[1] = value,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IoAddrLocation {
    /// FF01 — SB: Serial transfer data
    SB,
    /// FF02 — SC: Serial transfer control
    SC,
}
