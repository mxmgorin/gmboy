use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub const WRAM_BANK_SIZE: usize = 0x2000;
pub const WRAM_CGB_BANKS: usize = 7;
pub const WRAM_START_ADDR: u16 = 0xC000;
pub const WRAM_BANK_NUMBER_ADDR: u16 = 0xFF70;
pub const WRAM_CGB_BANK_START_ADDR: u16 = 0xD000;
pub const WRAM_CGB_BANK_END_ADDR: u16 = 0xDFFF;

const HRAM_SIZE: usize = 0x80;
const HRAM_ADDR_START: usize = 0xFF80;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ram {
    wram_bank_number: u8,
    #[serde(
        serialize_with = "serialize_array_wram",
        deserialize_with = "deserialize_array_wram"
    )]
    wram_bank_0: [u8; WRAM_BANK_SIZE],
    wram_cgb_banks: Box<[u8]>,
    #[serde(
        serialize_with = "serialize_array_hram",
        deserialize_with = "deserialize_array_hram"
    )]
    hram: [u8; HRAM_SIZE],
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            wram_bank_number: 0,
            wram_bank_0: [0; WRAM_BANK_SIZE],
            wram_cgb_banks: vec![0; WRAM_BANK_SIZE * WRAM_CGB_BANKS].into_boxed_slice(),
            hram: [0; HRAM_SIZE],
        }
    }
}

impl Ram {
    #[inline(always)]
    pub fn read_wram_bank(&self) -> u8 {
        self.wram_bank_number | 0xF8 // upper bits typically read as 1
    }

    #[inline(always)]
    pub fn write_wram_bank(&mut self, val: u8) {
        self.wram_bank_number = val & 0b0000_0111;
    }

    #[inline]
    pub fn read_wram_read(&self, addr: u16) -> u8 {
        match addr {
            WRAM_CGB_BANK_START_ADDR..=WRAM_CGB_BANK_END_ADDR => {
                let addr = self.wram_cgb_bank_addr(addr);
                unsafe { *self.wram_cgb_banks.get_unchecked(addr) }
            }
            _ => {
                // SAFETY: address is matched in bus
                unsafe { *self.wram_bank_0.get_unchecked(normalize_w_addr(addr)) }
            }
        }
    }

    #[inline]
    pub fn write_wram_write(&mut self, addr: u16, val: u8) {
        match addr {
            WRAM_CGB_BANK_START_ADDR..=WRAM_CGB_BANK_END_ADDR => {
                let addr = self.wram_cgb_bank_addr(addr);
                unsafe { *self.wram_cgb_banks.get_unchecked_mut(addr) = val }
            }
            _ => {
                // SAFETY: address is matched in bus
                unsafe {
                    *self.wram_bank_0.get_unchecked_mut(normalize_w_addr(addr)) = val;
                }
            }
        }
    }

    #[inline]
    pub fn read_hram_read(&self, addr: u16) -> u8 {
        // SAFETY: address is matched in bus
        unsafe { *self.hram.get_unchecked(normalize_h_addr(addr)) }
    }

    #[inline]
    pub fn write_hram_write(&mut self, addr: u16, val: u8) {
        // SAFETY: address is matched in bus
        unsafe {
            *self.hram.get_unchecked_mut(normalize_h_addr(addr)) = val;
        }
    }

    #[inline(always)]
    fn wram_cgb_bank_addr(&self, addr: u16) -> usize {
        let bank = if self.wram_bank_number == 0 {
            1
        } else {
            self.wram_bank_number as usize
        };
        let base = (bank - 1) * WRAM_BANK_SIZE;
        let offset = (addr - WRAM_CGB_BANK_START_ADDR) as usize;

        base + offset
    }
}

#[inline]
fn normalize_w_addr(addr: u16) -> usize {
    addr as usize - WRAM_START_ADDR as usize
}

#[inline]
fn normalize_h_addr(addr: u16) -> usize {
    addr as usize - HRAM_ADDR_START
}

pub fn serialize_array_wram<S>(arr: &[u8; WRAM_BANK_SIZE], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(WRAM_BANK_SIZE))?;
    for elem in arr.iter() {
        seq.serialize_element(elem)?;
    }
    seq.end()
}

pub fn deserialize_array_wram<'de, D>(deserializer: D) -> Result<[u8; WRAM_BANK_SIZE], D::Error>
where
    D: Deserializer<'de>,
{
    struct ArrayVisitor;

    impl<'de> Visitor<'de> for ArrayVisitor {
        type Value = [u8; WRAM_BANK_SIZE];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an array of {WRAM_BANK_SIZE} u8")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = Vec::with_capacity(WRAM_BANK_SIZE);
            for i in 0..WRAM_BANK_SIZE {
                let value = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(i, &self))?;
                vec.push(value);
            }
            vec.try_into()
                .map_err(|_| Error::custom("Failed to convert Vec to array"))
        }
    }

    deserializer.deserialize_seq(ArrayVisitor)
}

pub fn serialize_array_hram<S>(arr: &[u8; HRAM_SIZE], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(HRAM_SIZE))?;
    for elem in arr.iter() {
        seq.serialize_element(elem)?;
    }
    seq.end()
}

pub fn deserialize_array_hram<'de, D>(deserializer: D) -> Result<[u8; HRAM_SIZE], D::Error>
where
    D: Deserializer<'de>,
{
    struct ArrayVisitor;

    impl<'de> Visitor<'de> for ArrayVisitor {
        type Value = [u8; HRAM_SIZE];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an array of {HRAM_SIZE} u8")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = Vec::with_capacity(HRAM_SIZE);
            for i in 0..HRAM_SIZE {
                let value = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(i, &self))?;
                vec.push(value);
            }
            vec.try_into()
                .map_err(|_| Error::custom("Failed to convert Vec to array"))
        }
    }

    deserializer.deserialize_seq(ArrayVisitor)
}
