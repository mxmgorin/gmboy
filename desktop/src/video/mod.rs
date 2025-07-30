use core::ppu::tile::PixelColor;
use sdl2::render::Texture;
use crate::video::gl_backend::GlBackend;
use crate::video::sdl_backend::Sdl2Backend;

mod char;
pub mod draw_text;
pub mod game_window;
pub mod tiles_window;
pub mod frame_blend;
mod filter;
mod ui;
mod gl_backend;
mod shader;
mod sdl_backend;

const BYTES_PER_PIXEL: usize = 4;

pub fn fill_texture(texture: &mut Texture, color: PixelColor) {
    let (r, g, b, a) = color.as_rgba();

    texture
        .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            for i in (0..buffer.len()).step_by(BYTES_PER_PIXEL) {
                buffer[i] = r;
                buffer[i + 1] = g;
                buffer[i + 2] = b;
                buffer[i + 3] = a;
            }
        })
        .unwrap();
}

pub enum VideoBackend {
    Sdl2(Sdl2Backend),
    Gl(GlBackend),    
}
