use crate::config::VideoConfig;
use crate::video::gl_backend::GlBackend;
use crate::video::sdl2_backend::Sdl2Backend;
use core::ppu::tile::PixelColor;
use core::ppu::tile::TileData;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;
use sdl2::rect::Rect;

pub mod text;
mod font;
pub mod frame_blend;
mod sdl2_filters;
mod video;
pub use video::*;

mod gl_backend;
mod overlay;
mod sdl2_backend;
pub mod sdl2_tiles;
pub mod shader;

pub fn calc_win_height(scale: u32) -> u32 {
    LCD_Y_RES as u32 * scale
}

pub fn calc_win_width(scale: u32) -> u32 {
    LCD_X_RES as u32 * scale
}

pub fn new_scaled_rect(window_width: u32, window_height: u32) -> Rect {
    let screen_aspect = window_width as f32 / window_height as f32;
    let game_aspect = LCD_X_RES as f32 / LCD_Y_RES as f32;

    let (new_width, new_height) = if screen_aspect > game_aspect {
        // Screen is wider than game: Fit height, adjust width
        let height = window_height;
        let width = ((height as f32) * game_aspect) as u32;
        (width, height)
    } else {
        // Screen is taller than game: Fit width, adjust height
        let width = window_width;
        let height = ((width as f32) / game_aspect) as u32;
        (width, height)
    };

    // Center the image in the screen
    let x = ((window_width - new_width) / 2) as i32;
    let y = ((window_height - new_height) / 2) as i32;

    Rect::new(x, y, new_width, new_height)
}

pub struct VideoTexture {
    pub pitch: usize,
    pub buffer: Box<[u8]>,
    pub rect: Rect,
    pub bytes_per_pixel: usize,
}

impl VideoTexture {
    pub fn new(rect: Rect, bytes_per_pixel: usize) -> Self {
        let pitch = rect.w as usize * bytes_per_pixel;

        Self {
            pitch,
            buffer: vec![0; pitch * rect.h as usize].into_boxed_slice(),
            rect,
            bytes_per_pixel,
        }
    }

    pub fn fill(&mut self, color: PixelColor) {
        fill_buffer(&mut self.buffer, color, self.bytes_per_pixel)
    }

    pub fn clear(&mut self) {
        for i in (0..self.buffer.len()).step_by(self.bytes_per_pixel) {
            self.buffer[i] = 0;
            self.buffer[i + 1] = 0;
            self.buffer[i + 2] = 0;

            if self.bytes_per_pixel == 4 {
                self.buffer[i + 3] = 0;
            }
        }
    }
}

#[inline]
pub fn fill_buffer(buffer: &mut [u8], color: PixelColor, bytes_per_pixel: usize) {
    for i in (0..buffer.len()).step_by(bytes_per_pixel) {
        draw_color(buffer, i, color, bytes_per_pixel);
    }
}

#[inline]
pub fn draw_color(buffer: &mut [u8], index: usize, color: PixelColor, bytes_per_pixel: usize) {
    if bytes_per_pixel == 2 {
        let bytes = color.as_rgb565_bytes();
        buffer[index] = bytes[0];
        buffer[index + 1] = bytes[1];
    } else {
        let bytes = color.as_rgb_bytes();
        buffer[index] = bytes[0];
        buffer[index + 1] = bytes[1];
        buffer[index + 2] = bytes[2];

        if bytes_per_pixel == 4 {
            buffer[index + 3] = 255;
        }
    }
}

pub struct FrameBuffer<'a> {
    pub buffer: &'a mut [u8],
    pub pitch: usize,
    pub bytes_per_pixel: usize,
}

impl<'a> FrameBuffer<'a> {
    pub fn new_ppu(buffer: &'a mut [u8]) -> Self {
        FrameBuffer {
            buffer,
            pitch: core::ppu::PPU_PITCH,
            bytes_per_pixel: core::ppu::PPU_BYTES_PER_PIXEL,
        }
    }
}

pub fn truncate_text(s: &str, max_chars: usize) -> String {
    let max_len = s.len().min(max_chars + 2);
    let mut truncated = String::with_capacity(max_len);

    for (i, ch) in s.chars().enumerate() {
        if i == max_chars {
            let ends_with_paren = s.ends_with(')');
            let total_chars = s.chars().count();

            if total_chars > max_chars + 1 || !ends_with_paren {
                truncated.push('…');
            }

            if ends_with_paren {
                truncated.push(')');
            }

            break;
        }

        truncated.push(ch);
    }

    truncated
}

pub enum VideoBackend {
    Sdl2(Box<Sdl2Backend>),
    Gl(GlBackend),
}

impl VideoBackend {
    pub fn draw_buffer(&mut self, buffer: &[u8], config: &VideoConfig) {
        match self {
            VideoBackend::Sdl2(x) => x.draw_buffer(buffer, config),
            VideoBackend::Gl(x) => x.draw_buffer(buffer),
        }
    }

    pub fn draw_menu(&mut self, buffer: &[u8], config: &VideoConfig) {
        match self {
            VideoBackend::Sdl2(x) => x.draw_menu(buffer, config),
            VideoBackend::Gl(x) => x.draw_menu(buffer),
        }
    }

    pub fn show(&mut self) {
        match self {
            VideoBackend::Sdl2(x) => x.show(),
            VideoBackend::Gl(x) => x.show(),
        }
    }

    pub fn set_scale(&mut self, scale: u32) -> Result<(), String> {
        match self {
            VideoBackend::Sdl2(x) => x.set_scale(scale),
            VideoBackend::Gl(x) => x.set_scale(scale),
        }
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        match self {
            VideoBackend::Sdl2(x) => x.set_fullscreen(fullscreen),
            VideoBackend::Gl(x) => x.set_fullscreen(fullscreen),
        }
    }

    pub fn update_config(&mut self, config: &VideoConfig) {
        match self {
            VideoBackend::Sdl2(x) => x.update_config(config),
            VideoBackend::Gl(x) => x.update_config(&config.render),
        }
    }
    pub fn draw_tiles(&mut self, tiles: impl Iterator<Item = TileData>) {
        match self {
            VideoBackend::Sdl2(x) => x.draw_tiles(tiles),
            VideoBackend::Gl(_) => {}
        }
    }

    pub fn close_window(&mut self, id: u32) -> bool {
        match self {
            VideoBackend::Sdl2(x) => x.close_window(id),
            VideoBackend::Gl(x) => x.close_window(id),
        }
    }
}
