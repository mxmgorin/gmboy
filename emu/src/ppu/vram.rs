use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::ser::SerializeSeq;
use serde::de::{Visitor, SeqAccess, Error};
use std::fmt;
use crate::ppu::tile::{
    TileData, TileLineData, BG_TILE_MAP_1_ADDR_END, BG_TILE_MAP_1_ADDR_START,
    BG_TILE_MAP_2_ADDR_END, BG_TILE_MAP_2_ADDR_START, TILE_BIT_SIZE, TILE_HEIGHT,
    TILE_LINE_BYTES_COUNT, TILE_SET_2_END, TILE_SET_DATA_1_START,
};

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
            TILE_SET_DATA_1_START..=TILE_SET_2_END => VRamAddressLocation::ChrRam,
            BG_TILE_MAP_1_ADDR_START..=BG_TILE_MAP_1_ADDR_END => VRamAddressLocation::BgMap1,
            BG_TILE_MAP_2_ADDR_START..=BG_TILE_MAP_2_ADDR_END => VRamAddressLocation::BgMap2,
            _ => panic!("Invalid VRAM address: {:X}", address),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoRam {
    #[serde(
        serialize_with = "serialize_array_vram",
        deserialize_with = "deserialize_array_vram"
    )]
    pub bytes: [u8; VRAM_SIZE],
}

pub fn serialize_array_vram<S>(
    arr: &[u8; VRAM_SIZE],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(VRAM_SIZE))?;
    for elem in arr.iter() {
        seq.serialize_element(elem)?;
    }
    seq.end()
}

pub fn deserialize_array_vram<'de, D>(
    deserializer: D,
) -> Result<[u8; VRAM_SIZE], D::Error>
where
    D: Deserializer<'de>,
{
    struct ArrayVisitor;

    impl<'de> Visitor<'de> for ArrayVisitor {
        type Value = [u8; VRAM_SIZE];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an array of {} u8", VRAM_SIZE)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = Vec::with_capacity(VRAM_SIZE);
            for i in 0..VRAM_SIZE {
                let value = seq.next_element()?.ok_or_else(|| Error::invalid_length(i, &self))?;
                vec.push(value);
            }
            vec.try_into().map_err(|_| Error::custom("Failed to convert Vec to array"))
        }
    }

    deserializer.deserialize_seq(ArrayVisitor)
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

    pub fn get_tile_line(&self, addr: u16) -> TileLineData {
        TileLineData::new(self.read(addr), self.read(addr + 1))
    }

    pub fn get_tile(&self, addr: u16) -> TileData {
        let mut tile = TileData::default();

        for line_y in 0..TILE_HEIGHT as usize {
            tile.lines[line_y] = self.get_tile_line(addr + (line_y * TILE_LINE_BYTES_COUNT) as u16);
        }

        tile
    }

    pub fn fill_tiles(&self, tiles: &mut [TileData; 384]) {
        for (i, tile) in tiles.iter_mut().enumerate() {
            let addr = TILE_SET_DATA_1_START + (i as u16 * TILE_BIT_SIZE);
            *tile = self.get_tile(addr);
        }
    }
}

pub struct TilesIterator<'a> {
    pub video_ram: &'a VideoRam,
    pub current_address: u16,
}

impl Iterator for TilesIterator<'_> {
    type Item = TileData;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_address < TILE_SET_2_END {
            let tile = self.video_ram.get_tile(self.current_address);
            self.current_address += TILE_BIT_SIZE;

            return Some(tile);
        }

        None
    }
}
