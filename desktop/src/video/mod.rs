use core::ppu::LCD_Y_RES;
use core::ppu::LCD_X_RES;
use sdl2::rect::Rect;
use core::ppu::tile::PixelColor;

mod char;
pub mod draw_text;
pub mod game_window;
pub mod tiles_window;
pub mod frame_blend;
mod filter;
mod ui;
mod sdl2_backend;

const BYTES_PER_PIXEL: usize = 4;

pub fn fill_texture(buffer: &mut [u8], color: PixelColor) {
    let (r, g, b, a) = color.as_rgba();

    for i in (0..buffer.len()).step_by(BYTES_PER_PIXEL) {
        buffer[i] = r;
        buffer[i + 1] = g;
        buffer[i + 2] = b;
        buffer[i + 3] = a;
    }
}

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
