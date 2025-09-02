use crate::ppu::fifo::PixelFifo;
use crate::ppu::framebuffer::FrameBuffer;
use crate::ppu::lcd::{Lcd, PixelColor};
use crate::ppu::sprite::SpriteFetcher;
use crate::ppu::tile::{get_color_index, TileLineData, TILE_BITS_COUNT, TILE_HEIGHT, TILE_WIDTH};
use crate::ppu::vram::VideoRam;
use crate::ppu::{LCD_X_RES, PPU_BUFFER_LEN, PPU_BYTES_PER_PIXEL};
use serde::{Deserialize, Serialize};
use std::ptr;

type FetchFn = fn(&mut PixelFetcher, &Lcd, &VideoRam);

const FETCH_FNS: [FetchFn; 5] = [
    PixelFetcher::fetch_tile,
    PixelFetcher::fetch_data0,
    PixelFetcher::fetch_data1,
    PixelFetcher::fetch_idle,
    PixelFetcher::fetch_push,
];

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BgwFetchedData {
    pub tile_line: TileLineData,
    pub is_window: bool,
}

#[inline(always)]
pub fn get_bgw_tile_addr(tile_idx: u8, map_y: u8, data_area: u16) -> u16 {
    let tile_y = (map_y % TILE_HEIGHT as u8) * 2;

    data_area
        .wrapping_add(tile_idx as u16 * 16)
        .wrapping_add(tile_y as u16)
}

#[inline(always)]
pub fn normalize_bgw_tile_idx(tile_idx: u8, data_area: u16) -> u8 {
    if data_area == 0x8800 {
        return tile_idx.wrapping_add(128);
    }

    tile_idx
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelFetcher {
    pub buffer: FrameBuffer,
    pub sprite_fetcher: SpriteFetcher,
    fetch_step: FetchStep,
    line_x: u8,
    fetch_x: u8,
    fifo_x: u8,
    pixel_fifo: PixelFifo,
    bgw_fetched_data: BgwFetchedData,
    pub pushed_x: usize,
}

impl Default for PixelFetcher {
    fn default() -> PixelFetcher {
        Self {
            fetch_step: FetchStep::Tile,
            pixel_fifo: Default::default(),
            line_x: 0,
            pushed_x: 0,
            fetch_x: 0,
            bgw_fetched_data: Default::default(),
            fifo_x: 0,
            buffer: FrameBuffer::new(vec![0; PPU_BUFFER_LEN].into_boxed_slice()),
            sprite_fetcher: Default::default(),
        }
    }
}

impl PixelFetcher {
    #[inline(always)]
    pub fn process(&mut self, lcd: &Lcd, vram: &VideoRam, line_ticks: usize) {
        // fetch on odd lines
        if line_ticks & 1 != 0 {
            // SAFETY: we control FETCH_FNS and FetchStep
            unsafe {
                FETCH_FNS.get_unchecked(self.fetch_step as usize)(self, lcd, vram);
            }
        }

        self.try_fifo_pop(lcd);
    }

    #[inline(always)]
    fn try_fifo_pop(&mut self, lcd: &Lcd) {
        if let Some(pixel) = self.pixel_fifo.pop() {
            // Check if we are in the window or background layer
            // For the window layer, bypass scroll_x to avoid horizontal scrolling
            if self.bgw_fetched_data.is_window {
                // No horizontal scroll for window, only adjust based on `line_x` and `pushed_x`
                let index = self
                    .pushed_x
                    .wrapping_add(lcd.ly as usize * LCD_X_RES as usize);
                self.push_buffer(index, pixel);
            } else if self.line_x >= lcd.scroll_x % TILE_WIDTH as u8 {
                // For the background layer, apply scroll_x for horizontal scrolling
                let index = self
                    .pushed_x
                    .wrapping_add(lcd.ly as usize * LCD_X_RES as usize);
                self.push_buffer(index, pixel);
            }

            self.line_x += 1;
        }
    }

    #[inline(always)]
    fn try_fifo_push(&mut self, lcd: &Lcd) -> bool {
        if self.pixel_fifo.is_full() {
            return false;
        }

        let control = lcd.control;
        let obj_enabled = control.is_obj_enabled();
        let bgw_enabled = control.is_bgw_enabled();
        let bg_colors = &lcd.bg_colors;
        let x: i32 = self.fetch_x.wrapping_sub(8 - (lcd.scroll_x % 8)) as i32;

        if x < 0 {
            return true; // nothing to push
        }

        for bit in 0..TILE_BITS_COUNT {
            let bgw_color_index = get_color_index(
                self.bgw_fetched_data.tile_line.byte1,
                self.bgw_fetched_data.tile_line.byte2,
                bit,
            );

            let pixel = if obj_enabled {
                if let Some(sprite_pixel) =
                    self.sprite_fetcher
                        .get_sprite_color(lcd, self.fifo_x, bgw_color_index)
                {
                    sprite_pixel
                } else {
                    Self::get_gbw_color(bg_colors, bgw_color_index, bgw_enabled)
                }
            } else {
                Self::get_gbw_color(bg_colors, bgw_color_index, bgw_enabled)
            };

            self.pixel_fifo.push(pixel);
            self.fifo_x += 1;
        }

        true
    }

    #[inline(always)]
    fn push_buffer(&mut self, index: usize, pixel: PixelColor) {
        let base = index * PPU_BYTES_PER_PIXEL;
        let bytes = pixel.as_rgb565_bytes();

        unsafe {
            let dst = self.buffer.as_mut_ptr().add(base);
            ptr::copy_nonoverlapping(bytes.as_ptr(), dst, PPU_BYTES_PER_PIXEL);
        }

        self.pushed_x += 1;
    }

    #[inline(always)]
    fn fetch_tile(&mut self, lcd: &Lcd, vram: &VideoRam) {
        let control = lcd.control;

        if control.is_bgw_enabled() {
            let (map_y, tile_idx) =
                if let Some(tile_idx) = lcd.window.get_tile_idx(self.fetch_x as u16, lcd, vram) {
                    self.bgw_fetched_data.is_window = true;

                    (lcd.ly.wrapping_add(lcd.window.y), tile_idx)
                } else {
                    let map_y = lcd.ly.wrapping_add(lcd.scroll_y);
                    let map_x = self.fetch_x.wrapping_add(lcd.scroll_x);
                    let addr = control.get_bg_map_area()
                        + (map_x as u16 / TILE_WIDTH)
                        + ((map_y as u16 / TILE_HEIGHT) * 32);
                    self.bgw_fetched_data.is_window = false;

                    (map_y, vram.read(addr))
                };

            let data_area = control.get_bgw_data_area();
            let tile_idx = normalize_bgw_tile_idx(tile_idx, data_area);
            let addr = get_bgw_tile_addr(tile_idx, map_y, data_area);
            self.bgw_fetched_data.tile_line = vram.read_tile_line(addr);
        }

        if control.is_obj_enabled() {
            self.sprite_fetcher
                .fetch_sprites(lcd, vram, lcd.scroll_x, self.fetch_x);
        }

        self.fetch_step = FetchStep::Data0;
        self.fetch_x = self.fetch_x.wrapping_add(TILE_WIDTH as u8);
    }

    #[inline(always)]
    fn fetch_data0(&mut self, _: &Lcd, _: &VideoRam) {
        self.fetch_step = FetchStep::Data1;
    }

    #[inline(always)]
    fn fetch_data1(&mut self, _: &Lcd, _: &VideoRam) {
        self.fetch_step = FetchStep::Idle;
    }

    #[inline(always)]
    fn fetch_idle(&mut self, _: &Lcd, _: &VideoRam) {
        self.fetch_step = FetchStep::Push;
    }

    #[inline(always)]
    fn fetch_push(&mut self, lcd: &Lcd, _: &VideoRam) {
        if self.try_fifo_push(lcd) {
            self.fetch_step = FetchStep::Tile;
        }
    }

    #[inline(always)]
    fn get_gbw_color(colors: &[PixelColor; 4], index: usize, enabled: bool) -> PixelColor {
        if enabled {
            // SAFETY: always index 0..=3
            unsafe { *colors.get_unchecked(index) }
        } else {
            // SAFETY: there is always 4 colors
            unsafe { *colors.get_unchecked(0) }
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.fetch_step = FetchStep::Tile;
        self.pixel_fifo.clear();
        self.line_x = 0;
        self.fetch_x = 0;
        self.pushed_x = 0;
        self.fifo_x = 0;
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum FetchStep {
    Tile,
    Data0,
    Data1,
    Idle,
    Push,
}
