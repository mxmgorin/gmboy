use core::ppu::tile::PixelColor;

mod char;
pub mod draw_text;
pub mod game_window;
pub mod tiles_window;
pub mod frame_blend;
mod filter;
mod ui;

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
