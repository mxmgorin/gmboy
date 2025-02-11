use crate::ppu::tile::{
    Tile, TileLine, TILE_BYTE_SIZE, TILE_HEIGHT, TILE_LINE_BYTE_SIZE, TILE_TABLE_END,
};

// Tile data is stored in VRAM in the memory area at $8000-$97FF;

pub const VRAM_SIZE: usize = 0x2000;
pub const VRAM_ADDR_START: u16 = 0x8000;
pub const VRAM_ADDR_END: u16 = 0x9FFF;

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
            VRAM_ADDR_START..=TILE_TABLE_END => VRamAddressLocation::ChrRam,
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

    pub fn get_tile_line(&self, addr: u16) -> TileLine {
        let byte_one = self.read(addr);
        let byte_two = self.read(addr + 1);

        TileLine::new(byte_one, byte_two)
    }

    pub fn get_tile(&self, addr: u16) -> Tile {
        let mut tile = Tile::default();

        for line_y in 0..TILE_HEIGHT as usize {
            let tile_line = self.get_tile_line(addr + (line_y * TILE_LINE_BYTE_SIZE) as u16);
            tile.lines[line_y] = tile_line;
        }

        tile
    }
}

pub struct TilesIterator<'a> {
    pub vram: &'a VideoRam,
    pub current_address: u16,
}

impl Iterator for TilesIterator<'_> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_address < TILE_TABLE_END {
            let tile = self.vram.get_tile(self.current_address);
            self.current_address += TILE_BYTE_SIZE;

            return Some(tile);
        }

        None
    }
}
