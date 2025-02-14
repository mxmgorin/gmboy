use crate::bus::Bus;
use crate::ppu::lcd::Lcd;
use crate::ppu::oam::OamItem;
use crate::ppu::tile::{get_color_index, Pixel, PixelColor, TILE_BITS_COUNT, TILE_BIT_SIZE, TILE_HEIGHT, TILE_LINE_BYTES_COUNT, TILE_WIDTH};
use crate::ppu::{LCD_X_RES, LCD_Y_RES};
use std::collections::VecDeque;

pub const MAX_FIFO_BG_SIZE: usize = 8;
pub const MAX_FIFO_SPRITES_SIZE: usize = 10;

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub state: PipelineState,
    pub fifo: VecDeque<Pixel>,
    pub line_x: u8,
    pub pushed_x: u8,
    pub fetch_x: u8,
    pub bgw_fetch_data: [u8; 3],
    pub fetch_entry_data: [u8; 6],
    pub map_y: u8,
    pub map_x: u8,
    pub tile_y: u8,
    pub fifo_x: u8,

    pub line_ticks: usize,
    pub buffer: Vec<Pixel>,
    pub line_sprites: VecDeque<OamItem>,

    pub fetched_sprites_count: usize,
    pub fetched_sprites: [OamItem; 3], //entries fetched during pipeline.
}

impl Default for Pipeline {
    fn default() -> Pipeline {
        Self {
            state: PipelineState::Tile,
            fifo: Default::default(),
            line_x: 0,
            pushed_x: 0,
            fetch_x: 0,
            bgw_fetch_data: [0; 3],
            fetch_entry_data: [0; 6],
            map_y: 0,
            map_x: 0,
            tile_y: 0,
            fifo_x: 0,
            line_ticks: 0,
            buffer: vec![Pixel::default(); LCD_Y_RES as usize * LCD_X_RES as usize],
            line_sprites: Default::default(),
            fetched_sprites_count: 0,
            fetched_sprites: [OamItem::default(), OamItem::default(), OamItem::default()],
        }
    }
}

impl Pipeline {
    pub fn reset(&mut self) {
        self.fifo.clear();
    }

    pub fn process(&mut self, bus: &Bus) {
        self.map_y = bus.io.lcd.ly.wrapping_add(bus.io.lcd.scroll_y);
        self.map_x = self.fetch_x.wrapping_add(bus.io.lcd.scroll_x);
        self.tile_y = ((self.map_y % TILE_HEIGHT as u8) % 8) * 2;

        if self.line_ticks % 2 != 0 {
            self.fetch(bus);
        }

        self.buffer_pixel(bus);
    }

    fn buffer_pixel(&mut self, bus: &Bus) {
        if self.fifo.len() > MAX_FIFO_BG_SIZE {
            let pixel = self.fifo.pop_front().unwrap();

            if self.line_x >= bus.io.lcd.scroll_x % TILE_WIDTH as u8 {
                let index = (self.pushed_x as usize)
                    .wrapping_add(bus.io.lcd.ly as usize * LCD_X_RES as usize);
                self.buffer[index] = pixel;
                self.pushed_x += 1;
            }

            self.line_x += 1;
        }
    }

    fn fetch(&mut self, bus: &Bus) {
        match self.state {
            PipelineState::Tile => {
                self.fetched_sprites_count = 0;

                if bus.io.lcd.control.bgw_enabled() {
                    let addr = bus.io.lcd.control.bg_map_area()
                        + (self.map_x as u16 / TILE_WIDTH)
                        + ((self.map_y as u16 / TILE_HEIGHT) * 32);
                    self.bgw_fetch_data[0] = bus.read(addr);

                    if bus.io.lcd.control.bgw_data_area() == 0x8800 {
                        self.bgw_fetch_data[0] = self.bgw_fetch_data[0].wrapping_add(128);
                    }
                }

                if bus.io.lcd.control.obj_enabled() && !self.line_sprites.is_empty() {
                    self.load_sprite_tile(bus);
                }

                self.state = PipelineState::Data0;
                self.fetch_x = self.fetch_x.wrapping_add(TILE_WIDTH as u8);
            }
            PipelineState::Data0 => {
                self.bgw_fetch_data[1] = bus.read(self.get_bgw_data_addr(&bus.io.lcd));
                self.load_sprite_data(bus, 0);
                self.state = PipelineState::Data1;
            }
            PipelineState::Data1 => {
                self.bgw_fetch_data[2] = bus.read(self.get_bgw_data_addr(&bus.io.lcd) + 1);
                self.load_sprite_data(bus, 1);
                self.state = PipelineState::Idle;
            }
            PipelineState::Idle => self.state = PipelineState::Push,
            PipelineState::Push => {
                if self.try_fifo_add(bus) {
                    self.state = PipelineState::Tile;
                }
            }
        }
    }

    fn try_fifo_add(&mut self, bus: &Bus) -> bool {
        if self.fifo.len() > MAX_FIFO_BG_SIZE {
            return false;
        }

        let x: i32 = self.fetch_x.wrapping_sub(8 - (bus.io.lcd.scroll_x % 8)) as i32;

        for bit in 0..TILE_BITS_COUNT {
            let bg_color_index =
                get_color_index(self.bgw_fetch_data[1], self.bgw_fetch_data[2], bit);

            let bg_color = if bus.io.lcd.control.bgw_enabled() {
                bus.io.lcd.bg_colors[bg_color_index]
            } else {
                bus.io.lcd.bg_colors[0]
            };

            let color = if bus.io.lcd.control.obj_enabled() {
                self.fetch_sprite_pixels(&bus.io.lcd, bg_color, bg_color_index)
            } else {
                bg_color
            };

            if x >= 0 {
                self.fifo.push_back(Pixel::new(color, bg_color_index.into()));
                self.fifo_x += 1;
            }
        }

        true
    }

    fn get_bgw_data_addr(&self, lcd: &Lcd) -> u16 {
        lcd.control
            .bgw_data_area()
            .wrapping_add(self.bgw_fetch_data[0] as u16 * 16)
            .wrapping_add(self.tile_y as u16)
    }

    fn fetch_sprite_pixels(&self, lcd: &Lcd, bg_color: PixelColor, bg_color_index: usize) -> PixelColor {
        let mut bg_color = bg_color;

        for i in 0..self.fetched_sprites_count {
            let sprite_x = self.fetched_sprites[i].x - 8 + (lcd.scroll_x % 8);

            if sprite_x + 8 < self.fifo_x {
                // past pixel point already
                continue;
            }

            let offset: i32 = self.fifo_x as i32 - sprite_x as i32;

            if !(0..=7).contains(&offset) {
                // out of bounds
                continue;
            }

            let mut bit = 7 - offset;

            if self.fetched_sprites[i].f_x_flip() {
                bit = offset
            }

            let byte1 = self.fetch_entry_data[i * 2];
            let byte2 = self.fetch_entry_data[i * 2 + 1];
            let color_index = get_color_index(byte1, byte2, bit as u8);

            if color_index == 0 {
                // transparent
                continue;
            }

            if !self.fetched_sprites[i].f_bgp() || bg_color_index == 0 {
                bg_color = if self.fetched_sprites[i].f_pn() {
                    lcd.sp2_colors[color_index]
                } else {
                    lcd.sp1_colors[color_index]
                };

                if color_index != 0 {
                    break;
                }
            }
        }

        bg_color
    }

    fn load_sprite_data(&mut self, bus: &Bus, offset: u8) {
        let cur_y = bus.io.lcd.ly;
        let sprite_height = bus.io.lcd.control.obj_height();

        for (i, sprite) in self.fetched_sprites.iter().enumerate() {
            let mut tile_y = (cur_y + TILE_BIT_SIZE as u8) - sprite.y * TILE_LINE_BYTES_COUNT as u8;

            if sprite.f_y_flip() {
                tile_y = ((sprite_height * 2) - 2) - tile_y;
            }

            let tile_index = if sprite_height == 16 {
                // remove last bit
                sprite.tile_index & !1
            } else {
                sprite.tile_index
            };

            let addr = 0x8000 + (tile_index as u16 * 16) + tile_y as u16 + offset as u16;
            self.fetch_entry_data[(i * 2) + offset as usize] = bus.read(addr);
        }
    }

    fn load_sprite_tile(&mut self, bus: &Bus) {
        for sprite in self.line_sprites.iter() {
            let sp_x = (sprite.x - 8) + (bus.io.lcd.scroll_x % 8);

            if (sp_x >= self.fetch_x && sp_x < self.fetch_x + 8)
                || (sp_x + 8 >= self.fetch_x && sp_x + 8 < self.fetch_x + 8)
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
}

#[derive(Debug, Clone)]
pub enum PipelineState {
    Tile,
    Data0,
    Data1,
    Idle,
    Push,
}
