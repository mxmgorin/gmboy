const VRAM_SIZE: usize = 0x2000;
const VRAM_ADDR_OFFSET: usize = 0x8000;

pub enum VRamAddressLocation {
    /// 0x8000 - 0x97FF
    ChrRam,
    /// 0x9800 - 0x9BFF
    BgMap1,
    /// 0x9C00 - 0x9FFF
    BgMap2,
}

impl From<u16> for VRamAddressLocation {
    fn from(address: u16) -> Self {
        match address {
            0x8000..=0x97FF => VRamAddressLocation::ChrRam,
            0x9800..=0x9BFF => VRamAddressLocation::BgMap1,
            0x9C00..=0x9FFF => VRamAddressLocation::BgMap2,
            _ => panic!("Invalid VRAM address: {:X}", address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VRam {
    pub bytes: [u8; VRAM_SIZE],
}

impl VRam {
    pub fn new() -> Self {
        Self {
            bytes: [0; VRAM_SIZE],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.bytes[addr as usize - VRAM_ADDR_OFFSET]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.bytes[addr as usize - VRAM_ADDR_OFFSET] = val;
    }
}
