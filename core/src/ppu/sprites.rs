use crate::ppu::lcd::{Lcd, PixelColor};
use crate::ppu::oam::{OamEntry, OamRam};
use crate::ppu::tile::{
    get_color_idx, TileLineData, TILE_BIT_SIZE, TILE_LINE_BYTES_COUNT, TILE_SET_DATA_1_START,
};
use crate::ppu::vram::VideoRam;
use serde::{Deserialize, Serialize};

const MAX_LINE_SPRITES_COUNT: usize = 10;
const MAX_FETCHED_SPRITES_COUNT: usize = 3;

#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize)]
pub struct SpriteFetchedData {
    pub tile_line: TileLineData,
    pub oam: OamEntry,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpriteFetcher {
    line_sprites_count: usize,
    line_sprites: [OamEntry; MAX_LINE_SPRITES_COUNT],
    fetched_sprites_count: usize,
    fetched_sprites: [SpriteFetchedData; MAX_FETCHED_SPRITES_COUNT],
}

impl SpriteFetcher {
    #[inline(always)]
    pub fn scan_oam(&mut self, lcd: &Lcd, oam_ram: &OamRam) {
        let mut line_sprites_count = 0;
        let cur_y = lcd.ly.wrapping_add(16);
        let sprite_height = lcd.control.get_obj_height();

        for ram_entry in oam_ram.entries.iter() {
            if ram_entry.x == 0 {
                // Not visible (X = 0 means hidden on real hardware)
                continue;
            }

            // Check if the sprite is on the current scanline
            if ram_entry.y <= cur_y && ram_entry.y + sprite_height > cur_y {
                let mut inserted = false;

                // Iterate through sorted list to insert at correct position
                for i in 0..line_sprites_count {
                    let current_entry = unsafe { self.line_sprites.get_unchecked(i) };

                    if ram_entry.x < current_entry.x && ram_entry.x != current_entry.x {
                        // Sort by X first, then by OAM index if X is the same
                        for j in (i..line_sprites_count).rev() {
                            unsafe {
                                *self.line_sprites.get_unchecked_mut(j + 1) =
                                    *self.line_sprites.get_unchecked(j)
                            }
                        }

                        unsafe { *self.line_sprites.get_unchecked_mut(i) = *ram_entry };
                        inserted = true;
                        break;
                    }
                }

                if !inserted {
                    // If no earlier insertion, push to the back
                    unsafe {
                        *self.line_sprites.get_unchecked_mut(line_sprites_count) = *ram_entry;
                    }
                }

                line_sprites_count += 1;
            }

            if line_sprites_count >= MAX_LINE_SPRITES_COUNT {
                break;
            }
        }

        self.line_sprites_count = line_sprites_count;
    }

    #[inline(always)]
    pub fn fetch_sprites(&mut self, lcd: &Lcd, vram: &VideoRam, scroll_x: u8, fetch_x: u8) {
        let mut fetched_sprites_count = 0;
        let cur_y = lcd.ly.wrapping_add(TILE_BIT_SIZE as u8);
        let sprite_height = lcd.control.get_obj_height();

        for idx in 0..self.line_sprites_count {
            let oam = unsafe { *self.line_sprites.get_unchecked(idx) };
            let sp_x = self.calc_sprite_x(oam.x, scroll_x);

            if (sp_x >= fetch_x && sp_x < fetch_x.wrapping_add(8))
                || (sp_x.wrapping_add(8) >= fetch_x
                && sp_x.wrapping_add(8) < fetch_x.wrapping_add(8))
            {
                // need to add
                let mut tile_y = cur_y
                    .wrapping_sub(oam.y)
                    .wrapping_mul(TILE_LINE_BYTES_COUNT as u8);

                if oam.f_y_flip() {
                    tile_y = sprite_height
                        .wrapping_mul(2)
                        .wrapping_sub(2)
                        .wrapping_sub(tile_y);
                }

                let tile_index = if sprite_height == 16 {
                    // remove last bit
                    oam.tile_index & !1
                } else {
                    oam.tile_index
                };

                let addr = TILE_SET_DATA_1_START
                    .wrapping_add(tile_index as u16 * TILE_BIT_SIZE)
                    .wrapping_add(tile_y as u16);
                let tile_line = vram.read_tile_line(addr);

                unsafe {
                    let sprite = self
                        .fetched_sprites
                        .get_unchecked_mut(fetched_sprites_count);
                    sprite.oam = oam;
                    sprite.tile_line = tile_line;
                };
                fetched_sprites_count += 1;
            }

            if fetched_sprites_count >= MAX_FETCHED_SPRITES_COUNT {
                // max 3 sprites on pixels
                break;
            }
        }

        self.fetched_sprites_count = fetched_sprites_count;
    }

    #[inline(always)]
    fn calc_sprite_x(&self, sprite_x: u8, scroll_x: u8) -> u8 {
        sprite_x.wrapping_sub(8).wrapping_add(scroll_x % 8)
    }

    #[inline(always)]
    pub fn get_sprite_color(
        &self,
        lcd: &Lcd,
        fifo_x: u8,
        bg_color_index: usize,
    ) -> Option<PixelColor> {
        let scroll_x = lcd.scroll_x;

        for i in 0..self.fetched_sprites_count {
            let sprite = unsafe { self.fetched_sprites.get_unchecked(i) };
            let sprite_x = self.calc_sprite_x(sprite.oam.x, scroll_x);

            if sprite_x.wrapping_add(8) < fifo_x {
                continue; // Skip past sprites
            }

            let offset = fifo_x.wrapping_sub(sprite_x);
            if !(0..=7).contains(&offset) {
                continue; // Out of sprite range
            }

            let bit = if sprite.oam.f_x_flip() {
                7 - offset
            } else {
                offset
            };

            let color_idx = get_color_idx(sprite.tile_line.byte1, sprite.tile_line.byte2, bit);

            if color_idx == 0 {
                continue; // Transparent
            }

            if !sprite.oam.f_bgp() || bg_color_index == 0 {
                let color = unsafe {
                    if sprite.oam.f_pn() {
                        lcd.sp2_colors.get_unchecked(color_idx)
                    } else {
                        lcd.sp1_colors.get_unchecked(color_idx)
                    }
                };

                return Some(*color);
            }
        }

        None
    }
}
