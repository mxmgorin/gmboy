use crate::ppu::vram::{VRAM_ADDR_START};

pub const TILE_TABLE_START: u16 = VRAM_ADDR_START;
pub const TILE_TABLE_END: u16 = 0x97FF;
pub const TILE_LINE_BYTE_SIZE: usize = 2;
pub const TILE_BYTE_SIZE: u16 = 16;
pub const TILE_WIDTH: u16 = 8;
pub const TILE_HEIGHT: u16 = 8;
pub const TILE_BITS_COUNT: i32 = 8;

#[derive(Copy, Clone, Debug, Default)]
pub struct Tile {
    pub lines: [TileLine; TILE_HEIGHT as usize],
}

#[derive(Copy, Clone, Debug, Default)]
pub struct TileLine {
    pub byte_one: u8,
    pub byte_two: u8,
}

impl TileLine {
    pub fn new(byte_one: u8, byte_two: u8) -> TileLine {
        Self { byte_one, byte_two }
    }

    pub fn get_color_id(&self, bit: i32) -> usize {
        let bit1 = (self.byte_one >> (7 - bit)) & 0x01;
        let bit2 = (self.byte_two >> (7 - bit)) & 0x01;

        (bit2 << 1 | bit1) as usize
    }

    pub fn iter_color_ids(&self) -> impl Iterator<Item = usize> {
        TileLineIterator {
            bit: 0,
            line: *self,
        }
    }
}

pub struct  TileLineIterator {
    pub bit: u8,
    pub line: TileLine,
}

impl Iterator for  TileLineIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bit < TILE_BITS_COUNT as u8 {
            let bit = self.bit;
            self.bit += 1;

            Some(self.line.get_color_id(bit as i32))
        } else {
            None
        }
    }

}