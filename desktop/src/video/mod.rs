use crate::config::VideoConfig;
use crate::video::game_window::VideoTexture;
use crate::video::sdl2_backend::Sdl2Backend;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;
use sdl2::rect::Rect;

mod char;
pub mod draw_text;
mod filter;
pub mod frame_blend;
pub mod game_window;
mod sdl2_backend;
pub mod tiles_window;
mod ui;

const BYTES_PER_PIXEL: usize = 4;

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

pub enum VideoBackend {
    Sdl2(Sdl2Backend),
}

impl VideoBackend {
    pub fn draw_buffer(&mut self, buffer: &[u32], config: &VideoConfig) {
        match self {
            VideoBackend::Sdl2(x) => x.draw_buffer(buffer, config),
        }
    }

    pub fn draw_menu(&mut self, texture: &VideoTexture, config: &VideoConfig) {
        match self {
            VideoBackend::Sdl2(x) => x.draw_menu(texture, config),
        }
    }

    pub fn draw_fps(&mut self, texture: &VideoTexture) {
        match self {
            VideoBackend::Sdl2(x) => x.draw_fps(texture),
        }
    }

    pub fn draw_notif(&mut self, texture: &VideoTexture) {
        match self {
            VideoBackend::Sdl2(x) => x.draw_notif(texture),
        }
    }

    pub fn show(&mut self) {
        match self {
            VideoBackend::Sdl2(x) => x.show(),
        }
    }

    pub fn set_scale(&mut self, scale: u32) -> Result<(), String> {
        match self {
            VideoBackend::Sdl2(x) => x.set_scale(scale),
        }
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        match self {
            VideoBackend::Sdl2(x) => x.set_fullscreen(fullscreen),
        }
    }

    pub fn get_position(&self) -> (i32, i32) {
        match self {
            VideoBackend::Sdl2(x) => x.get_position(),
        }
    }
}
