use crate::bus::Bus;
use crate::ppu::lcd::Lcd;
use crate::ppu::sprite::SpriteFetcher;
use crate::ppu::tile::{get_color_index, Pixel, TILE_BITS_COUNT, TILE_HEIGHT, TILE_WIDTH};
use crate::ppu::{LCD_X_RES, LCD_Y_RES};
use std::collections::VecDeque;

pub const MAX_FIFO_SIZE: usize = 8;
pub const MAX_FIFO_SPRITES_SIZE: usize = 10;

#[derive(Debug, Clone, Default)]
pub struct BgwFetchedData {
    pub tile_idx: u8,
    pub byte1: u8,
    pub byte2: u8,
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub pushed_x: u8,
    pub sprite_fetcher: SpriteFetcher,
    pub buffer: Vec<Pixel>,

    fetch_step: FetchStep,
    line_x: u8,
    fetch_x: u8,
    map_y: u8,
    map_x: u8,
    tile_y: u8,
    fifo_x: u8,
    pixel_fifo: VecDeque<Pixel>,
    bgw_fetched_data: BgwFetchedData,
}

impl Default for Pipeline {
    fn default() -> Pipeline {
        Self {
            fetch_step: FetchStep::Tile,
            pixel_fifo: Default::default(),
            line_x: 0,
            pushed_x: 0,
            fetch_x: 0,
            bgw_fetched_data: Default::default(),
            map_y: 0,
            map_x: 0,
            tile_y: 0,
            fifo_x: 0,
            buffer: vec![Pixel::default(); LCD_Y_RES as usize * LCD_X_RES as usize],
            sprite_fetcher: Default::default(),
        }
    }
}

impl Pipeline {
    pub fn process(&mut self, bus: &Bus, line_ticks: usize) {
        self.map_y = bus.io.lcd.ly.wrapping_add(bus.io.lcd.scroll_y);
        self.map_x = self.fetch_x.wrapping_add(bus.io.lcd.scroll_x);
        self.tile_y = (self.map_y % TILE_HEIGHT as u8) * 2;

        if line_ticks & 1 != 0 {
            self.fetch(bus);
        }

        self.push_pixel(bus);
    }

    fn push_pixel(&mut self, bus: &Bus) {
        if self.pixel_fifo.len() > MAX_FIFO_SIZE {
            let pixel = self.pixel_fifo.pop_front().unwrap();

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
        match self.fetch_step {
            FetchStep::Tile => {
                if bus.io.lcd.control.bgw_enabled() {
                    let addr = bus.io.lcd.control.bg_map_area()
                        + (self.map_x as u16 / TILE_WIDTH)
                        + ((self.map_y as u16 / TILE_HEIGHT) * 32);
                    self.bgw_fetched_data.tile_idx = bus.read(addr);

                    if let Some(tile_idx) = bus.io.lcd.window.get_tile_idx(self.fetch_x as u16, bus)
                    {
                        self.bgw_fetched_data.tile_idx = tile_idx;
                    }

                    if bus.io.lcd.control.bgw_data_area() == 0x8800 {
                        self.bgw_fetched_data.tile_idx =
                            self.bgw_fetched_data.tile_idx.wrapping_add(128);
                    }
                }

                if bus.io.lcd.control.obj_enabled() {
                    self.sprite_fetcher
                        .fetch_sprite_tiles(bus.io.lcd.scroll_x, self.fetch_x);
                }

                self.fetch_step = FetchStep::Data0;
                self.fetch_x = self.fetch_x.wrapping_add(TILE_WIDTH as u8);
            }
            FetchStep::Data0 => {
                self.bgw_fetched_data.byte1 = bus.read(self.get_bgw_data_addr(&bus.io.lcd));
                self.sprite_fetcher.fetch_sprite_data(bus, 0);
                self.fetch_step = FetchStep::Data1;
            }
            FetchStep::Data1 => {
                self.bgw_fetched_data.byte2 = bus.read(self.get_bgw_data_addr(&bus.io.lcd) + 1);
                self.sprite_fetcher.fetch_sprite_data(bus, 1);
                self.fetch_step = FetchStep::Idle;
            }
            FetchStep::Idle => self.fetch_step = FetchStep::Push,
            FetchStep::Push => {
                if self.try_fifo_add(bus) {
                    self.fetch_step = FetchStep::Tile;
                }
            }
        }
    }

    fn try_fifo_add(&mut self, bus: &Bus) -> bool {
        if self.pixel_fifo.len() > MAX_FIFO_SIZE {
            return false;
        }

        let x: i32 = self.fetch_x.wrapping_sub(8 - (bus.io.lcd.scroll_x % 8)) as i32;

        for bit in 0..TILE_BITS_COUNT {
            let bgw_color_index = get_color_index(
                self.bgw_fetched_data.byte1,
                self.bgw_fetched_data.byte2,
                bit,
            );

            let bgw_pixel = if bus.io.lcd.control.bgw_enabled() {
                Pixel::new(
                    bus.io.lcd.bg_colors[bgw_color_index],
                    bgw_color_index.into(),
                )
            } else {
                Pixel::new(bus.io.lcd.bg_colors[0], 0.into())
            };

            let sprite_pixel = if bus.io.lcd.control.obj_enabled() {
                self.sprite_fetcher
                    .fetch_sprite_pixel(&bus.io.lcd, self.fifo_x, bgw_color_index)
            } else {
                None
            };

            let pixel = sprite_pixel.unwrap_or(bgw_pixel);

            if x >= 0 {
                self.pixel_fifo.push_back(pixel);
                self.fifo_x += 1;
            }
        }

        true
    }

    pub fn reset(&mut self) {
        self.fetch_step = FetchStep::Tile;
        self.line_x = 0;
        self.fetch_x = 0;
        self.pushed_x = 0;
        self.fifo_x = 0;
    }

    pub fn clear(&mut self) {
        self.pixel_fifo.clear();
    }

    fn get_bgw_data_addr(&self, lcd: &Lcd) -> u16 {
        lcd.control
            .bgw_data_area()
            .wrapping_add(self.bgw_fetched_data.tile_idx as u16 * 16)
            .wrapping_add(self.tile_y as u16)
    }
}

#[derive(Debug, Clone)]
pub enum FetchStep {
    Tile,
    Data0,
    Data1,
    Idle,
    Push,
}
