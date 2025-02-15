use crate::bus::Bus;
use crate::ppu::lcd::Lcd;
use crate::ppu::sprite::SpriteFetcher;
use crate::ppu::tile::{get_color_index, Pixel, TILE_BITS_COUNT, TILE_HEIGHT, TILE_WIDTH};
use crate::ppu::window::Window;
use crate::ppu::{LCD_X_RES, LCD_Y_RES};
use std::collections::VecDeque;

pub const MAX_FIFO_SIZE: usize = 8;
pub const MAX_FIFO_SPRITES_SIZE: usize = 10;

#[derive(Debug, Clone, Default)]
pub struct BgwFetchedData {
    pub tile_index: u8,
    pub byte1: u8,
    pub byte2: u8,
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub state: PipelineState,
    pub line_ticks: usize,
    pub line_x: u8,
    pub pushed_x: u8,
    pub fetch_x: u8,
    pub map_y: u8,
    pub map_x: u8,
    pub tile_y: u8,
    pub fifo_x: u8,

    pub pixel_fifo: VecDeque<Pixel>,
    pub bgw_fetched_data: BgwFetchedData,
    pub sprite_fetcher: SpriteFetcher,

    pub buffer: Vec<Pixel>,
}

impl Default for Pipeline {
    fn default() -> Pipeline {
        Self {
            state: PipelineState::Tile,
            pixel_fifo: Default::default(),
            line_x: 0,
            pushed_x: 0,
            fetch_x: 0,
            bgw_fetched_data: Default::default(),
            map_y: 0,
            map_x: 0,
            tile_y: 0,
            fifo_x: 0,
            line_ticks: 0,
            buffer: vec![Pixel::default(); LCD_Y_RES as usize * LCD_X_RES as usize],
            sprite_fetcher: Default::default(),
        }
    }
}

impl Pipeline {
    pub fn process(&mut self, bus: &Bus) {
        self.map_y = bus.io.lcd.ly.wrapping_add(bus.io.lcd.scroll_y);
        self.map_x = self.fetch_x.wrapping_add(bus.io.lcd.scroll_x);
        self.tile_y = ((self.map_y % TILE_HEIGHT as u8) % 8) * 2;

        if self.line_ticks & 1 != 0 {
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
        match self.state {
            PipelineState::Tile => {
                if bus.io.lcd.control.bgw_enabled() {
                    let addr = bus.io.lcd.control.bg_map_area()
                        + (self.map_x as u16 / TILE_WIDTH)
                        + ((self.map_y as u16 / TILE_HEIGHT) * 32);
                    self.bgw_fetched_data.tile_index = bus.read(addr);

                    if let Some(tile) = bus.io.lcd.window.get_tile_index(self.fetch_x as u16, bus) {
                        self.bgw_fetched_data.tile_index = tile;
                    }

                    if bus.io.lcd.control.bgw_data_area() == 0x8800 {
                        self.bgw_fetched_data.tile_index =
                            self.bgw_fetched_data.tile_index.wrapping_add(128);
                    }
                }

                if bus.io.lcd.control.obj_enabled() {
                    self.sprite_fetcher
                        .fetch_sprite_tiles(bus.io.lcd.scroll_x, self.fetch_x);
                }

                self.state = PipelineState::Data0;
                self.fetch_x = self.fetch_x.wrapping_add(TILE_WIDTH as u8);
            }
            PipelineState::Data0 => {
                self.bgw_fetched_data.byte1 = bus.read(self.get_bgw_data_addr(&bus.io.lcd));
                self.sprite_fetcher.fetch_sprite_data(bus, 0);
                self.state = PipelineState::Data1;
            }
            PipelineState::Data1 => {
                self.bgw_fetched_data.byte2 = bus.read(self.get_bgw_data_addr(&bus.io.lcd) + 1);
                self.sprite_fetcher.fetch_sprite_data(bus, 1);
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
        if self.pixel_fifo.len() > MAX_FIFO_SIZE {
            return false;
        }

        let x: i32 = self.fetch_x.wrapping_sub(8 - (bus.io.lcd.scroll_x % 8)) as i32;

        for bit in 0..TILE_BITS_COUNT {
            let bg_color_index = get_color_index(
                self.bgw_fetched_data.byte1,
                self.bgw_fetched_data.byte2,
                bit,
            );

            let bg_pixel = if bus.io.lcd.control.bgw_enabled() {
                Pixel::new(bus.io.lcd.bg_colors[bg_color_index], bg_color_index.into())
            } else {
                Pixel::new(bus.io.lcd.bg_colors[0], 0.into())
            };

            let pixel = if bus.io.lcd.control.obj_enabled() {
                let pixel = self.sprite_fetcher.fetch_sprite_pixel(
                    &bus.io.lcd,
                    self.fifo_x,
                    bg_color_index,
                );

                pixel.unwrap_or(bg_pixel)
            } else {
                bg_pixel
            };

            if x >= 0 {
                self.pixel_fifo.push_back(pixel);
                self.fifo_x += 1;
            }
        }

        true
    }

    fn get_bgw_data_addr(&self, lcd: &Lcd) -> u16 {
        lcd.control
            .bgw_data_area()
            .wrapping_add(self.bgw_fetched_data.tile_index as u16 * 16)
            .wrapping_add(self.tile_y as u16)
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
