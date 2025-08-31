use crate::ppu::fetcher::MAX_FIFO_SPRITES_SIZE;
use crate::ppu::lcd::{Lcd, PixelColor};
use crate::ppu::oam::{OamEntry, OamRam};
use crate::ppu::tile::{
    get_color_index, TileLineData, TILE_BIT_SIZE, TILE_LINE_BYTES_COUNT, TILE_SET_DATA_1_START,
};
use crate::ppu::vram::VideoRam;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize)]
pub struct SpriteFetchedData {
    pub tile_line: TileLineData,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpriteFetcher {
    pub line_sprites_count: usize,
    pub line_sprites: [OamEntry; MAX_FIFO_SPRITES_SIZE],
    pub fetched_sprites_count: usize,
    pub fetched_sprites: [OamEntry; 3], // entries fetched during pipeline.
    pub fetched_sprite_data: [SpriteFetchedData; 3],
}

impl SpriteFetcher {
    #[inline(always)]
    pub fn load_line_sprites(&mut self, lcd: &Lcd, oam_ram: &OamRam) {
        self.line_sprites_count = 0;
        let cur_y = lcd.ly.wrapping_add(16);
        let sprite_height = lcd.control.get_obj_height();

        for ram_entry in oam_ram.entries.iter() {
            if ram_entry.x == 0 {
                // Not visible (X = 0 means hidden on real hardware)
                continue;
            }

            if self.line_sprites_count >= MAX_FIFO_SPRITES_SIZE {
                // Already reached max sprites per scanline (Game Boy limit = 10)
                break;
            }

            // Check if the sprite is on the current scanline
            if ram_entry.y <= cur_y && ram_entry.y + sprite_height > cur_y {
                let mut inserted = false;

                // Iterate through sorted list to insert at correct position
                for i in 0..self.line_sprites_count {
                    let current_entry = unsafe { self.line_sprites.get_unchecked(i) };

                    if ram_entry.x < current_entry.x && ram_entry.x != current_entry.x {
                        // Sort by X first, then by OAM index if X is the same
                        for j in (i..self.line_sprites_count).rev() {
                            unsafe {
                                *self.line_sprites.get_unchecked_mut(j + 1) =
                                    *self.line_sprites.get_unchecked(j)
                            }
                        }

                        unsafe { *self.line_sprites.get_unchecked_mut(i) = *ram_entry };
                        self.line_sprites_count += 1;
                        inserted = true;
                        break;
                    }
                }

                if !inserted {
                    // If no earlier insertion, push to the back
                    unsafe {
                        *self.line_sprites.get_unchecked_mut(self.line_sprites_count) = *ram_entry;
                    }
                    self.line_sprites_count += 1;
                }
            }
        }
    }

    #[inline(always)]
    pub fn fetch_sprite_tiles(&mut self, scroll_x: u8, fetch_x: u8) {
        self.fetched_sprites_count = 0;

        for idx in 0..self.line_sprites_count {
            let sprite = unsafe { self.line_sprites.get_unchecked(idx) };
            let sp_x = self.calc_sprite_x(sprite.x, scroll_x);

            if (sp_x >= fetch_x && sp_x < fetch_x.wrapping_add(8))
                || (sp_x.wrapping_add(8) >= fetch_x
                    && sp_x.wrapping_add(8) < fetch_x.wrapping_add(8))
            {
                // need to add
                unsafe {
                    *self
                        .fetched_sprites
                        .get_unchecked_mut(self.fetched_sprites_count) = *sprite;
                };
                self.fetched_sprites_count += 1;
            }

            if self.fetched_sprites_count >= self.fetched_sprites.len() {
                // max 3 sprites on pixels
                break;
            }
        }
    }

    #[inline(always)]
    fn calc_sprite_x(&self, sprite_x: u8, scroll_x: u8) -> u8 {
        sprite_x.wrapping_sub(8).wrapping_add(scroll_x % 8)
    }

    #[inline(always)]
    pub fn fetch_sprite_data(&mut self, lcd: &Lcd, vram: &VideoRam, byte_offset: u16) {
        let cur_y = lcd.ly.wrapping_add(TILE_BIT_SIZE as u8);
        let sprite_height = lcd.control.get_obj_height();

        for i in 0..self.fetched_sprites_count {
            let sprite = unsafe { self.fetched_sprites.get_unchecked(i) };

            let mut tile_y = cur_y
                .wrapping_sub(sprite.y)
                .wrapping_mul(TILE_LINE_BYTES_COUNT as u8);

            if sprite.f_y_flip() {
                tile_y = sprite_height
                    .wrapping_mul(2)
                    .wrapping_sub(2)
                    .wrapping_sub(tile_y);
            }

            let tile_index = if sprite_height == 16 {
                // remove last bit
                sprite.tile_index & !1
            } else {
                sprite.tile_index
            };

            let addr = TILE_SET_DATA_1_START
                .wrapping_add(tile_index as u16 * TILE_BIT_SIZE)
                .wrapping_add(tile_y as u16)
                .wrapping_add(byte_offset);

            let data = unsafe { self.fetched_sprite_data.get_unchecked_mut(i) };
            let value = vram.read(addr);

            unsafe {
                *data
                    .tile_line
                    .as_bytes_mut()
                    .get_unchecked_mut(byte_offset as usize) = value;
            }
        }
    }

    #[inline(always)]
    pub fn fetch_sprite_pixel(
        &self,
        lcd: &Lcd,
        fifo_x: u8,
        bg_color_index: usize,
    ) -> Option<PixelColor> {
        for i in 0..self.fetched_sprites_count {
            let sprite = unsafe { self.fetched_sprites.get_unchecked(i) };
            let sprite_x = self.calc_sprite_x(sprite.x, lcd.scroll_x);

            if sprite_x.wrapping_add(8) < fifo_x {
                continue; // Skip past sprites
            }

            let offset = fifo_x.wrapping_sub(sprite_x);
            if !(0..=7).contains(&offset) {
                continue; // Out of sprite range
            }

            let bit = if sprite.f_x_flip() {
                7 - offset
            } else {
                offset
            };

            let data = unsafe { self.fetched_sprite_data.get_unchecked(i) };
            let color_index = get_color_index(data.tile_line.byte1, data.tile_line.byte2, bit);

            if color_index == 0 {
                continue; // Transparent
            }

            if !sprite.f_bgp() || bg_color_index == 0 {
                let color = unsafe {
                    if sprite.f_pn() {
                        lcd.sp2_colors.get_unchecked(color_index)
                    } else {
                        lcd.sp1_colors.get_unchecked(color_index)
                    }
                };

                return Some(*color);
            }
        }

        None
    }
}
