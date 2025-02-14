use crate::bus::Bus;
use crate::ppu::lcd::Lcd;
use crate::ppu::oam::OamItem;
use crate::ppu::pipeline::MAX_FIFO_SPRITES_SIZE;
use crate::ppu::tile::{
    get_color_index, Pixel, TILE_BIT_SIZE, TILE_LINE_BYTES_COUNT, TILE_TABLE_START,
};
use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub struct SpriteFetcher {
    pub line_sprites: VecDeque<OamItem>,
    pub fetched_sprites_count: usize,
    pub fetched_sprites: [OamItem; 3], //entries fetched during pipeline.
    pub fetched_sprite_data: [u8; 6],
}

impl SpriteFetcher {
    pub fn load_line_sprites(&mut self, bus: &mut Bus) {
        self.line_sprites.clear();
        let cur_y = bus.io.lcd.ly;
        let sprite_height = bus.io.lcd.control.obj_height();

        for ram_sprite in bus.oam_ram.items.iter() {
            if ram_sprite.x == 0 {
                // not visible
                continue;
            }

            if self.line_sprites.len() >= MAX_FIFO_SPRITES_SIZE {
                // max sprites per line
                break;
            }

            if ram_sprite.y <= cur_y + 16 && ram_sprite.y + sprite_height >= cur_y + 16 {
                // this sprite is on the current line
                if let Some(line_sprite) = self.line_sprites.front() {
                    if line_sprite.x > ram_sprite.x {
                        self.line_sprites.push_front(ram_sprite.to_owned());
                        continue;
                    }
                }

                self.line_sprites.push_back(ram_sprite.to_owned());

                // do sorting
                for i in 0..self.line_sprites.len() {
                    if self.line_sprites[i].x > ram_sprite.x {
                        self.line_sprites.insert(i, ram_sprite.to_owned());
                        break;
                    }
                }
            }
        }
    }

    pub fn fetch_sprite_tiles(&mut self, scroll_x: u8, fetch_x: u8) {
        self.fetched_sprites_count = 0;

        for sprite in self.line_sprites.iter() {
            let sp_x = (sprite.x - 8) + (scroll_x % 8);

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
        let cur_y = bus.io.lcd.ly as usize;
        let sprite_height = bus.io.lcd.control.obj_height() as usize;

        for i in 0..self.fetched_sprites_count {
            let sprite = self.fetched_sprites[i];
            let mut tile_y = cur_y
                .wrapping_add(TILE_BIT_SIZE as usize)
                .wrapping_sub(sprite.y as usize)
                .wrapping_mul(TILE_LINE_BYTES_COUNT);

            if sprite.f_y_flip() {
                tile_y = ((sprite_height * 2) - 2) - tile_y;
            }

            let tile_index = if sprite_height == 16 {
                // remove last bit
                sprite.tile_index & !1
            } else {
                sprite.tile_index
            };

            let addr = TILE_TABLE_START
                .wrapping_add(tile_index as u16 * TILE_BIT_SIZE)
                .wrapping_add(tile_y as u16)
                .wrapping_add(byte_offset);

            self.fetched_sprite_data[(i * 2) + byte_offset as usize] = bus.read(addr);
        }
    }

    pub fn fetch_sprite_pixel(
        &self,
        lcd: &Lcd,
        fifo_x: u8,
        bg_color_index: usize,
    ) -> Option<Pixel> {
        for i in 0..self.fetched_sprites_count {
            let sprite_x = self.fetched_sprites[i].x - 8 + (lcd.scroll_x % 8);

            if sprite_x + 8 < fifo_x {
                // past pixel point already
                continue;
            }

            let offset: i32 = fifo_x as i32 - sprite_x as i32;

            if !(0..=7).contains(&offset) {
                // out of bounds
                continue;
            }

            let bit = if self.fetched_sprites[i].f_x_flip() {
                7 - offset
            } else {
                offset
            };

            let byte1 = self.fetched_sprite_data[i * 2];
            let byte2 = self.fetched_sprite_data[(i * 2) + 1];
            let color_index = get_color_index(byte1, byte2, bit as u8);

            if color_index == 0 {
                // transparent
                continue;
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
