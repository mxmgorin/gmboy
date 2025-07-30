use crate::video::gl_backend::GlBackend;
use crate::video::sdl_backend::Sdl2Backend;
use core::ppu::tile::PixelColor;
use sdl2::render::Texture;

mod char;
pub mod draw_text;
mod filter;
pub mod frame_blend;
pub mod game_window;
mod gl_backend;
mod sdl_backend;
mod shader;
pub mod tiles_window;
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

pub enum VideoBackend {
    Sdl2(Sdl2Backend),
    Gl(GlBackend),
}
