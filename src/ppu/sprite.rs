use crate::bus::Bus;
use crate::ppu::lcd::Lcd;
use crate::ppu::oam::OamEntry;
use crate::ppu::fetcher::MAX_FIFO_SPRITES_SIZE;
use crate::ppu::tile::{
    get_color_index, Pixel, TileLineData, TILE_BIT_SIZE, TILE_LINE_BYTES_COUNT,
    TILE_SET_DATA_1_START,
};
use std::collections::VecDeque;

#[derive(Debug, Clone, Default, Copy)]
pub struct SpriteFetchedData {
    pub tile_line: TileLineData,
}

#[derive(Debug, Clone, Default)]
pub struct SpriteFetcher {
    pub line_sprites: VecDeque<OamEntry>,
    pub fetched_sprites_count: usize,
    pub fetched_sprites: [OamEntry; 3], //entries fetched during pipeline.
    pub fetched_sprite_data: [SpriteFetchedData; 3],
}

impl SpriteFetcher {
    pub fn load_line_sprites(&mut self, bus: &mut Bus) {
        self.line_sprites.clear();
        let cur_y: i32 = bus.io.lcd.ly as i32;
        let sprite_height = bus.io.lcd.control.obj_height() as i32;

        for ram_entry in bus.oam_ram.entries.iter() {
            if ram_entry.x == 0 {
                // Not visible (X = 0 means hidden on real hardware)
                continue;
            }

            if self.line_sprites.len() >= MAX_FIFO_SPRITES_SIZE {
                // Already reached max sprites per scanline (Game Boy limit = 10)
                break;
            }

            // Check if the sprite is on the current scanline
            if ram_entry.y as i32 <= cur_y + 16 && ram_entry.y as i32 + sprite_height > cur_y + 16 {
                let mut inserted = false;

                // Iterate through sorted list to insert at correct position
                for i in 0..self.line_sprites.len() {
                    let current_entry = &self.line_sprites[i];

                    if ram_entry.x < current_entry.x && ram_entry.x != current_entry.x {
                        // Sort by X first, then by OAM index if X is the same
                        self.line_sprites.insert(i, ram_entry.to_owned());
                        inserted = true;
                        break;
                    }
                }

                if !inserted {
                    // If no earlier insertion, push to the back
                    self.line_sprites.push_back(ram_entry.to_owned());
                }
            }
        }
    }

    pub fn fetch_sprite_tiles(&mut self, _scroll_x: u8, fetch_x: u8) {
        self.fetched_sprites_count = 0;
        let fetch_x = fetch_x as i32;

        for sprite in self.line_sprites.iter() {
            let sp_x: i32 = (sprite.x as i32).wrapping_sub(8);
            // todo: is it needed?
            // scroll_x doesn't used for sprites—it only applies to the background
            //.wrapping_add(scroll_x as i32 % 8);

            if (sp_x >= fetch_x && sp_x < fetch_x + 8)
                || (sp_x + 8 >= fetch_x && sp_x + 8 < fetch_x + 8)
            {
                // need to add
                self.fetched_sprites[self.fetched_sprites_count] = sprite.to_owned();
                self.fetched_sprites_count += 1;
            }

            if self.fetched_sprites_count >= self.fetched_sprites.len() {
                // max 3 sprites on pixels
                break;
            }
        }
    }

    pub fn fetch_sprite_data(&mut self, bus: &Bus, byte_offset: u16) {
        let cur_y: i32 = bus.io.lcd.ly as i32;
        let sprite_height: u8 = bus.io.lcd.control.obj_height();

        for i in 0..self.fetched_sprites_count {
            let sprite = self.fetched_sprites[i];
            let mut tile_y: u8 = cur_y
                .wrapping_add(TILE_BIT_SIZE as i32)
                .wrapping_sub(sprite.y as i32)
                .wrapping_mul(TILE_LINE_BYTES_COUNT as i32) as u8;

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

            match byte_offset {
                0 => self.fetched_sprite_data[i].tile_line.byte1 = bus.read(addr),
                1 => self.fetched_sprite_data[i].tile_line.byte2 = bus.read(addr),
                _ => unreachable!(),
            }
        }
    }

    pub fn fetch_sprite_pixel(
        &self,
        lcd: &Lcd,
        fifo_x: u8,
        bg_color_index: usize,
    ) -> Option<Pixel> {
        for i in 0..self.fetched_sprites_count {
            let sprite_x = self.fetched_sprites[i].x as i32 - 8;

            if sprite_x + 8 < fifo_x as i32 {
                continue; // Skip past sprites
            }

            let offset = fifo_x as i32 - sprite_x;
            if !(0..=7).contains(&offset) {
                continue; // Out of sprite range
            }

            let bit = if self.fetched_sprites[i].f_x_flip() {
                7 - offset
            } else {
                offset
            };

            let data = self.fetched_sprite_data[i];
            let color_index =
                get_color_index(data.tile_line.byte1, data.tile_line.byte2, bit as u8);

            if color_index == 0 {
                continue; // Transparent
            }

            if !self.fetched_sprites[i].f_bgp() || bg_color_index == 0 {
                let color = if self.fetched_sprites[i].f_pn() {
                    lcd.sp2_colors[color_index]
                } else {
                    lcd.sp1_colors[color_index]
                };

                return Some(Pixel::new(color, color_index.into()));
            }
        }

        None
    }
}
