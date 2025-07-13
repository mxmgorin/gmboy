// Object attributes reside in the object attribute memory (OAM) at $FE00-FE9F.
// Has 40 movable objects.
use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub const OAM_ENTRIES_COUNT: usize = 40;
pub const OAM_ADDR_START: u16 = 0xFE00;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OamRam {
    #[serde(
        serialize_with = "serialize_array_oam",
        deserialize_with = "deserialize_array_oam"
    )]
    pub entries: [OamEntry; OAM_ENTRIES_COUNT],
}

impl Default for OamRam {
    fn default() -> Self {
        Self {
            entries: [OamEntry::default(); OAM_ENTRIES_COUNT],
        }
    }
}

impl OamRam {
    pub fn read(&self, addr: u16) -> u8 {
        let (item_index, byte_offset) = self.get_index_and_offset(addr);

        match byte_offset {
            0 => self.entries[item_index].y,
            1 => self.entries[item_index].x,
            2 => self.entries[item_index].tile_index,
            3 => self.entries[item_index].flags,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        let (item_index, byte_offset) = self.get_index_and_offset(addr);

        match byte_offset {
            0 => self.entries[item_index].y = value,
            1 => self.entries[item_index].x = value,
            2 => self.entries[item_index].tile_index = value,
            3 => self.entries[item_index].flags = value,
            _ => unreachable!(),
        };
    }

    /// Determine the index in the oam_ram array and the specific byte to update
    fn get_index_and_offset(&self, addr: u16) -> (usize, usize) {
        let addr = addr - OAM_ADDR_START;
        let item_index = (addr / 4) as usize; // Each `OamItem` is 4 bytes
        let byte_offset = (addr % 4) as usize;

        (item_index, byte_offset)
    }
}

//  Bit7   BG and Window over OBJ (0=No, 1=BG and Window colors 1-3 over the OBJ)
//  Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
//  Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
//  Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
//  Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
//  Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct OamEntry {
    pub y: u8,
    pub x: u8,
    pub tile_index: u8,
    pub flags: u8,
}

impl OamEntry {
    pub fn f_cgb_pn(&self) -> u8 {
        self.flags & 0b0000_0111 // Extract bits 0-2
    }

    pub fn f_cgb_vram_bank(&self) -> bool {
        (self.flags & 0b0000_1000) != 0 // Bit 3
    }

    pub fn f_pn(&self) -> bool {
        (self.flags & 0b0001_0000) != 0 // Bit 4
    }

    pub fn f_x_flip(&self) -> bool {
        (self.flags & 0b0010_0000) != 0 // Bit 5
    }

    pub fn f_y_flip(&self) -> bool {
        (self.flags & 0b0100_0000) != 0 // Bit 6
    }

    pub fn f_bgp(&self) -> bool {
        (self.flags & 0b1000_0000) != 0 // Bit 7
    }
}

pub fn serialize_array_oam<S>(
    arr: &[OamEntry; OAM_ENTRIES_COUNT],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(OAM_ENTRIES_COUNT))?;
    for elem in arr.iter() {
        seq.serialize_element(elem)?;
    }
    seq.end()
}

// Visitor for deserialization
struct OamArrayVisitor;

impl<'de> Visitor<'de> for OamArrayVisitor {
    type Value = [OamEntry; OAM_ENTRIES_COUNT];

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "an array of {} OamEntry items",
            OAM_ENTRIES_COUNT
        )
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<[OamEntry; OAM_ENTRIES_COUNT], A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut vec = Vec::with_capacity(OAM_ENTRIES_COUNT);
        for i in 0..OAM_ENTRIES_COUNT {
            let value = seq
                .next_element()?
                .ok_or_else(|| Error::invalid_length(i, &self))?;
            vec.push(value);
        }
        vec.try_into()
            .map_err(|_| Error::custom("Failed to convert Vec to array"))
    }
}

// Deserializer function
pub fn deserialize_array_oam<'de, D>(
    deserializer: D,
) -> Result<[OamEntry; OAM_ENTRIES_COUNT], D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_seq(OamArrayVisitor)
}
