use crate::bus::Bus;
use crate::ppu::lcd::Lcd;
use crate::ppu::tile::{get_color_id, TILE_BITS_COUNT, TILE_HEIGHT, TILE_WIDTH};
use crate::ppu::{X_RES, Y_RES};
use std::collections::VecDeque;

pub const FIFO_MAX_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub state: PipelineState,
    pub fifo: VecDeque<u32>,
    pub line_x: u8,
    pub pushed_x: u8,
    pub fetch_x: u8,
    pub bgw_fetch_data: [u8; 3],
    pub _fetch_entry_data: [u8; 6],
    pub map_y: u8,
    pub map_x: u8,
    pub tile_y: u8,
    pub fifo_x: u8,

    pub line_ticks: usize,
    pub buffer: Vec<u32>,
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
            _fetch_entry_data: [0; 6],
            map_y: 0,
            map_x: 0,
            tile_y: 0,
            fifo_x: 0,
            line_ticks: 0,
            buffer: vec![0; Y_RES as usize * X_RES as usize],
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

        self.push_pixel(bus);
    }

    fn push_pixel(&mut self, bus: &Bus) {
        if self.fifo.len() > FIFO_MAX_SIZE {
            let pixel = self.fifo.pop_front().unwrap();

            if self.line_x >= bus.io.lcd.scroll_x % TILE_WIDTH as u8 {
                let index =
                    (self.pushed_x as usize).wrapping_add(bus.io.lcd.ly as usize * X_RES as usize);
                self.buffer[index] = pixel;
                self.pushed_x += 1;
            }

            self.line_x += 1;
        }
    }

    fn fetch(&mut self, bus: &Bus) {
        match self.state {
            PipelineState::Tile => {
                if bus.io.lcd.control.lcd_enable() {
                    let addr = bus.io.lcd.control.bg_map_area()
                        + (self.map_x as u16 / TILE_WIDTH)
                        + ((self.map_y as u16 / TILE_HEIGHT) * 32);
                    self.bgw_fetch_data[0] = bus.read(addr);

                    if bus.io.lcd.control.bgw_data_area() == 0x8800 {
                        self.bgw_fetch_data[0] = self.bgw_fetch_data[0].wrapping_add(128);
                    }
                }

                self.state = PipelineState::Data0;
                self.fetch_x = self.fetch_x.wrapping_add(TILE_WIDTH as u8);
            }
            PipelineState::Data0 => {
                self.bgw_fetch_data[1] = bus.read(self.get_bgw_data_addr(&bus.io.lcd));
                self.state = PipelineState::Data1;
            }
            PipelineState::Data1 => {
                self.bgw_fetch_data[2] = bus.read(self.get_bgw_data_addr(&bus.io.lcd) + 1);
                self.state = PipelineState::Idle;
            }
            PipelineState::Idle => self.state = PipelineState::Push,
            PipelineState::Push => {
                if self.try_fifo_push(bus) {
                    self.state = PipelineState::Tile;
                }
            }
        }
    }

    fn try_fifo_push(&mut self, bus: &Bus) -> bool {
        if self.fifo.len() > FIFO_MAX_SIZE {
            return false;
        }

        let x: i32 = self.fetch_x.wrapping_sub(8 - (bus.io.lcd.scroll_x % 8)) as i32;

        for bit in 0..TILE_BITS_COUNT {
            let color_id = get_color_id(self.bgw_fetch_data[1], self.bgw_fetch_data[2], bit);
            let color = bus.io.lcd.bg_colors[color_id];

            if x >= 0 {
                self.fifo.push_back(color);
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
}

#[derive(Debug, Clone)]
pub enum PipelineState {
    Tile,
    Data0,
    Data1,
    Idle,
    Push,
}
