use crate::hex_to_rgba;
use crate::ppu::vram::VRAM_ADDR_START;

pub const TILE_TABLE_START: u16 = VRAM_ADDR_START;
pub const TILE_TABLE_END: u16 = 0x97FF;
pub const TILE_LINE_BYTES_COUNT: usize = 2;
pub const TILE_BIT_SIZE: u16 = 16;
pub const TILE_WIDTH: u16 = 8;
pub const TILE_HEIGHT: u16 = 8;
pub const TILE_BITS_COUNT: u8 = 8;
pub const TILES_COUNT: usize = 384;

#[derive(Copy, Clone, Debug, Default)]
pub struct TileData {
    pub lines: [TileLineData; TILE_HEIGHT as usize],
}

#[derive(Copy, Clone, Debug, Default)]
pub struct TileLineData {
    pub byte1: u8,
    pub byte2: u8,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct PixelData {
    pub byte1: u8,
    pub byte2: u8,
    pub bit: u8,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Pixel {
    pub color: PixelColor,
    pub color_id: ColorId,
}

impl Pixel {
    pub fn new(color: PixelColor, color_id: ColorId) -> Pixel {
        Pixel { color, color_id }
    }
}

impl PixelData {
    pub fn new(byte1: u8, byte2: u8, bit: u8) -> PixelData {
        Self { byte1, byte2, bit }
    }

    pub fn into_color_index(self) -> usize {
        get_color_index(self.byte1, self.byte2, self.bit)
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ColorId {
    #[default]
    Lightest,
    Light,
    Dark,
    Darkest,
}

impl From<usize> for ColorId {
    fn from(value: usize) -> Self {
        match value {
            0 => ColorId::Lightest,
            1 => ColorId::Light,
            2 => ColorId::Dark,
            3 => ColorId::Darkest,
            _ => panic!("Invalid value for ColorId {}", value),
        }
    }
}

impl ColorId {
    pub fn new(byte1: u8, byte2: u8, bit: u8) -> ColorId {
        get_color_index(byte1, byte2, bit).into()
    }
}

pub fn get_color_index(byte1: u8, byte2: u8, bit: u8) -> usize {
    let bit1 = (byte1 >> (7 - bit)) & 0x01;
    let bit2 = (byte2 >> (7 - bit)) & 0x01;

    (bit2 << 1 | bit1) as usize
}

impl TileLineData {
    pub fn new(byte_one: u8, byte_two: u8) -> TileLineData {
        Self {
            byte1: byte_one,
            byte2: byte_two,
        }
    }

    pub fn get_color_id(&self, bit: u8) -> ColorId {
        ColorId::new(self.byte1, self.byte2, bit)
    }

    pub fn iter_color_ids(&self) -> impl Iterator<Item = ColorId> {
        TileLineIterator {
            bit: 0,
            line: *self,
        }
    }
}

pub struct TileLineIterator {
    pub bit: u8,
    pub line: TileLineData,
}

impl Iterator for TileLineIterator {
    type Item = ColorId;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bit < TILE_BITS_COUNT {
            let bit = self.bit;
            self.bit += 1;

            Some(self.line.get_color_id(bit))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PixelColor {
    hex: u32,
}

impl PixelColor {
    pub const fn from_hex(hex: u32) -> PixelColor {
        PixelColor { hex }
    }

    pub fn as_hex(&self) -> u32 {
        self.hex
    }

    pub fn as_rgba(&self) -> (u8, u8, u8, u8) {
        hex_to_rgba(self.hex)
    }
}
