use crate::ppu::vram::{VRAM_ADDR_START};

pub const TILES_COUNT: usize = TILE_ROWS as usize * TILE_COLS as usize;
pub const TILE_TABLE_START: u16 = VRAM_ADDR_START;
pub const TILE_TABLE_END: u16 = 0x97FF;
pub const TILE_LINE_BYTE_SIZE: u16 = 2;
pub const TILE_BYTE_SIZE: u16 = 16;
pub const TILE_WIDTH: u16 = 8;
pub const TILE_HEIGHT: u16 = 8;
pub const TILE_BYTES_COUNT: usize = 16;
pub const TILE_ROWS: i32 = 24;
pub const TILE_COLS: i32 = 16;

pub const TILE_BITS_COUNT: i32 = 8;

#[derive(Copy, Clone, Debug, Default)]
pub struct TilePixel {
    pub byte_one: u8,
    pub byte_two: u8,
}

impl TilePixel {
    pub fn new(byte_one: u8, byte_two: u8) -> TilePixel {
        Self { byte_one, byte_two }
    }

    pub fn get_color_index(&self, bit: i32) -> usize {
        let bit1 = (self.byte_one >> (7 - bit)) & 0x01;
        let bit2 = (self.byte_two >> (7 - bit)) & 0x01;

        (bit2 << 1 | bit1) as usize
    }
}
