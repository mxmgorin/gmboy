use crate::ppu::vram::VRAM_SIZE;
use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

pub const W_RAM_SIZE: usize = 0x2000;
const H_RAM_SIZE: usize = 0x80;
const W_RAM_ADDR_START: usize = 0xC000;
const H_RAM_ADDR_START: usize = 0xFF80;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ram {
    #[serde(
        serialize_with = "serialize_array_wram",
        deserialize_with = "deserialize_array_wram"
    )]
    working_ram: [u8; W_RAM_SIZE],
    #[serde(
        serialize_with = "serialize_array_hram",
        deserialize_with = "deserialize_array_hram"
    )]
    high_ram: [u8; H_RAM_SIZE],
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            working_ram: [0; W_RAM_SIZE],
            high_ram: [0; H_RAM_SIZE],
        }
    }
}

impl Ram {
    pub fn working_ram_read(&self, addr: u16) -> u8 {
        self.working_ram[normalize_w_addr(addr)]
    }

    pub fn working_ram_write(&mut self, addr: u16, val: u8) {
        self.working_ram[normalize_w_addr(addr)] = val;
    }

    pub fn high_ram_read(&self, addr: u16) -> u8 {
        self.high_ram[normalize_h_addr(addr)]
    }

    pub fn high_ram_write(&mut self, addr: u16, val: u8) {
        self.high_ram[normalize_h_addr(addr)] = val;
    }
}

fn normalize_w_addr(addr: u16) -> usize {
    addr as usize - W_RAM_ADDR_START
}

fn normalize_h_addr(addr: u16) -> usize {
    addr as usize - H_RAM_ADDR_START
}

pub fn serialize_array_wram<S>(arr: &[u8; W_RAM_SIZE], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(VRAM_SIZE))?;
    for elem in arr.iter() {
        seq.serialize_element(elem)?;
    }
    seq.end()
}

pub fn deserialize_array_wram<'de, D>(deserializer: D) -> Result<[u8; W_RAM_SIZE], D::Error>
where
    D: Deserializer<'de>,
{
    struct ArrayVisitor;

    impl<'de> Visitor<'de> for ArrayVisitor {
        type Value = [u8; W_RAM_SIZE];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an array of {} u8", W_RAM_SIZE)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = Vec::with_capacity(W_RAM_SIZE);
            for i in 0..W_RAM_SIZE {
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

pub fn serialize_array_hram<S>(arr: &[u8; H_RAM_SIZE], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(VRAM_SIZE))?;
    for elem in arr.iter() {
        seq.serialize_element(elem)?;
    }
    seq.end()
}

pub fn deserialize_array_hram<'de, D>(deserializer: D) -> Result<[u8; H_RAM_SIZE], D::Error>
where
    D: Deserializer<'de>,
{
    struct ArrayVisitor;

    impl<'de> Visitor<'de> for ArrayVisitor {
        type Value = [u8; H_RAM_SIZE];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an array of {} u8", H_RAM_SIZE)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut vec = Vec::with_capacity(H_RAM_SIZE);
            for i in 0..H_RAM_SIZE {
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
