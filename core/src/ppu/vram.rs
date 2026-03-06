use crate::ppu::tile::{
    TileData, TileFlags, TileLineData, TILE_BIT_SIZE, TILE_HEIGHT, TILE_LINE_BYTES_COUNT,
    TILE_SET_2_END, TILE_SET_DATA_1_START,
};
use serde::{Deserialize, Serialize};

pub const VRAM_CGB_BANKS_COUNT: usize = 2;
pub const VRAM_BANK_SIZE: usize = 0x2000;
pub const VRAM_ADDR_START: u16 = 0x8000;
pub const VRAM_ADDR_END: u16 = 0x9FFF;
pub const VRAM_BANK_NUMBER_ADDR: u16 = 0xFF4F;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoRam {
    banks: [Box<[u8]>; VRAM_CGB_BANKS_COUNT],
    bank_number: u8,
}

impl Default for VideoRam {
    fn default() -> Self {
        Self {
            banks: std::array::from_fn(|_| vec![0u8; VRAM_BANK_SIZE].into_boxed_slice()),
            bank_number: 0,
        }
    }
}

impl VideoRam {
    #[inline(always)]
    pub fn write_bank_number(&mut self, val: u8) {
        self.bank_number = val & 0x01;
    }

    #[inline(always)]
    pub fn read_bank_number(&self) -> u8 {
        self.bank_number | 0b1110
    }

    #[inline(always)]
    pub fn read(&self, addr: u16) -> u8 {
        let addr = (addr - VRAM_ADDR_START) as usize;
        let bank = self.get_bank();
        unsafe { *bank.get_unchecked(addr) }
    }

    #[inline(always)]
    pub fn write(&mut self, addr: u16, val: u8) {
        let addr = (addr - VRAM_ADDR_START) as usize;
        let bank = self.get_bank_mut();
        unsafe {
            *bank.get_unchecked_mut(addr) = val;
        }
    }

    #[inline(always)]
    pub fn read_tile_line(&self, addr: u16) -> TileLineData {
        let addr = (addr - VRAM_ADDR_START) as usize;
        let bank = self.get_bank();

        unsafe {
            TileLineData::new(
                *bank.get_unchecked(addr),
                *bank.get_unchecked(addr.wrapping_add(1)),
            )
        }
    }

    #[inline(always)]
    pub fn read_tile(&self, addr: u16) -> TileData {
        let mut tile = TileData::default();

        for line_y in 0..TILE_HEIGHT as usize {
            tile.lines[line_y] =
                self.read_tile_line(addr + (line_y * TILE_LINE_BYTES_COUNT) as u16);
        }

        tile
    }

    #[inline(always)]
    pub fn iter_tiles(&self) -> impl Iterator<Item = TileData> + '_ {
        (0..384).map(move |i| {
            let addr = TILE_SET_DATA_1_START + (i as u16 * TILE_BIT_SIZE);
            self.read_tile(addr)
        })
    }

    #[inline(always)]
    fn get_bank(&self) -> &[u8] {
        unsafe { self.banks.get_unchecked(self.bank_number as usize) }
    }

    #[inline(always)]
    fn get_bank_mut(&mut self) -> &mut [u8] {
        unsafe { self.banks.get_unchecked_mut(self.bank_number as usize) }
    }

    #[inline(always)]
    pub fn read_from_bank(&self, bank: u8, addr: u16) -> u8 {
        let addr = (addr - VRAM_ADDR_START) as usize;
        unsafe { *self.banks.get_unchecked(bank as usize).get_unchecked(addr) }
    }

    #[inline(always)]
    pub fn read_tile_flags(&self, addr: u16) -> TileFlags {
        self.read_from_bank(1, addr).into()
    }

    #[inline(always)]
    pub fn read_tile_line_from_bank(&self, bank: u8, addr: u16) -> TileLineData {
        let addr = (addr - VRAM_ADDR_START) as usize;
        let bank_ref = unsafe { self.banks.get_unchecked(bank as usize) };

        unsafe {
            TileLineData::new(
                *bank_ref.get_unchecked(addr),
                *bank_ref.get_unchecked(addr.wrapping_add(1)),
            )
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
            let tile = self.video_ram.read_tile(self.current_address);
            self.current_address += TILE_BIT_SIZE;

            return Some(tile);
        }

        None
    }
}
