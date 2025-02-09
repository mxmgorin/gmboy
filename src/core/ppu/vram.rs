// Tile data is stored in VRAM in the memory area at $8000-$97FF;
// with each tile taking 16 bytes, this area defines data for 384 tiles.

pub const VRAM_SIZE: usize = 0x2000;
pub const VRAM_ADDR_START: u16 = 0x8000;

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
            VRAM_ADDR_START..=0x97FF => VRamAddressLocation::ChrRam,
            0x9800..=0x9BFF => VRamAddressLocation::BgMap1,
            0x9C00..=0x9FFF => VRamAddressLocation::BgMap2,
            _ => panic!("Invalid VRAM address: {:X}", address),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VideoRam {
    pub bytes: [u8; VRAM_SIZE],
}

impl Default for VideoRam {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoRam {
    pub fn new() -> Self {
        Self {
            bytes: [0; VRAM_SIZE],
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.bytes[(addr - VRAM_ADDR_START) as usize]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.bytes[(addr - VRAM_ADDR_START) as usize] = val;
    }
}
