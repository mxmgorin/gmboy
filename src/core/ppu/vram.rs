// Tile data is stored in VRAM in the memory area at $8000-$97FF;
// with each tile taking 16 bytes, this area defines data for 384 tiles.

use sdl2::pixels::Color;

pub const TILE_TABLE_START: u16 = VRAM_ADDR_START;
pub const TILE_TABLE_END: u16 = 0x97FF;
pub const TILE_LINE_BYTE_SIZE: u16 = 2;
pub const TILE_BYTE_SIZE: u16 = 16;
pub const TILE_WIDTH: u16 = 8;
pub const TILE_HEIGHT: i32 = 16;
pub const TILE_ROWS: i32 = 24;
pub const TILE_COLS: i32 = 16;
pub const TILES_COUNT: usize = TILE_ROWS as usize * TILE_COLS as usize;
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

#[derive(Debug, Clone)]
pub struct Tile {
    pub data: [[Color; TILE_WIDTH as usize]; TILE_HEIGHT as usize],
}

impl Tile {
    pub fn new(bytes: &[u8], colors: Colors) -> Self {
        let mut data = [[colors.white; TILE_WIDTH as usize]; TILE_HEIGHT as usize];

        for row in 0..(TILE_HEIGHT as usize) {
            let first_byte = bytes[row * 2];
            let second_byte = bytes[row * 2 + 1];

            for col in 0..(TILE_WIDTH as usize) {
                let bit1 = (first_byte >> (7 - col)) & 0x01;
                let bit2 = (second_byte >> (7 - col)) & 0x01;

                data[row][col] = match (bit2, bit1) {
                    (0, 0) => colors.white,
                    (0, 1) => colors.light,
                    (1, 0) => colors.dark,
                    (1, 1) => colors.black,
                    _ => unreachable!(),
                };
            }
        }

        Self { data }
    }
}

pub struct Colors {
    pub black: Color,
    pub dark: Color,
    pub light: Color,
    pub white: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self::new()
    }
}

impl Colors {
    pub fn new() -> Self {
        Self {
            black: Color::RGB(8, 24, 32),
            dark: Color::RGB(52, 104, 86),
            light: Color::RGB(136, 192, 112),
            white: Color::RGB(224, 248, 208),
        }
    }
}
