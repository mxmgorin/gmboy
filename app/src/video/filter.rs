use crate::config::{RenderConfig, Sdl2Config};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

pub struct Filters {
    grid_texture: Texture,
    subpixel_texture: Texture,
    scan_line_texture: Texture,
    dot_matrix_texture: Texture,
    vignette_texture: Texture,
    rect: Rect,
}

impl Filters {
    pub fn new(
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        rect: Rect,
    ) -> Self {
        Self {
            grid_texture: generate_grid_texture(
                canvas,
                texture_creator,
                rect.width(),
                rect.height(),
            ),
            subpixel_texture: generate_subpixel_texture(
                canvas,
                texture_creator,
                rect.width(),
                rect.height(),
            ),
            scan_line_texture: generate_scanline_texture(
                canvas,
                texture_creator,
                rect.width(),
                rect.height(),
            ),
            dot_matrix_texture: generate_dot_matrix_texture(
                canvas,
                texture_creator,
                rect.width(),
                rect.height(),
            ),
            vignette_texture: generate_vignette_texture(
                canvas,
                texture_creator,
                rect.width(),
                rect.height(),
            ),
            rect,
        }
    }

    pub fn apply(&self, canvas: &mut Canvas<Window>, config: &Sdl2Config) {
        if config.vignette_enabled {
            canvas
                .copy(&self.vignette_texture, None, Some(self.rect))
                .unwrap();
        }

        if config.dot_matrix_enabled {
            canvas
                .copy(&self.dot_matrix_texture, None, Some(self.rect))
                .unwrap();
        }

        if config.scanline_enabled {
            canvas
                .copy(&self.scan_line_texture, None, Some(self.rect))
                .unwrap();
        }

        if config.grid_enabled {
            canvas
                .copy(&self.grid_texture, None, Some(self.rect))
                .unwrap();
        }

        if config.subpixel_enabled {
            canvas
                .copy(&self.subpixel_texture, None, Some(self.rect))
                .unwrap();
        }
    }
}

fn generate_subpixel_texture(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    width: u32,
    height: u32,
) -> Texture {
    let mut tex = texture_creator
        .create_texture_target(PixelFormatEnum::ABGR8888, width, height)
        .unwrap();

    canvas
        .with_texture_canvas(&mut tex, |subcanvas| {
            // Define three shades for the stripe pattern
            let colors = [
                Color::RGBA(180, 200, 180, 40), // light
                Color::RGBA(140, 170, 140, 40), // medium
                Color::RGBA(100, 140, 100, 40), // dark
            ];

            let stripe_width = 1; // 1-pixel wide stripes
            let mut x = 0;

            while x < width as i32 {
                for (i, color) in colors.iter().enumerate() {
                    let rect = Rect::new(x + i as i32, 0, stripe_width, height);
                    subcanvas.set_draw_color(*color);
                    subcanvas.fill_rect(rect).unwrap();
                }
                x += colors.len() as i32;
            }
        })
        .unwrap();

    tex.set_blend_mode(sdl2::render::BlendMode::Blend);
    tex
}

fn generate_grid_texture(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    width: u32,
    height: u32,
) -> Texture {
    let mut grid_texture = texture_creator
        .create_texture_target(PixelFormatEnum::ABGR8888, width, height)
        .unwrap();
    canvas
        .with_texture_canvas(&mut grid_texture, |tex| {
            tex.set_draw_color(Color::RGBA(32, 32, 32, 80));
            for i in 0..=RenderConfig::WIDTH {
                let x = (i as f32 * width as f32 / RenderConfig::WIDTH as f32) as i32;
                tex.draw_line((x, 0), (x, height as i32)).unwrap();
            }
            for j in 0..=RenderConfig::HEIGHT {
                let y = (j as f32 * height as f32 / RenderConfig::HEIGHT as f32) as i32;
                tex.draw_line((0, y), (width as i32, y)).unwrap();
            }
        })
        .unwrap();
    grid_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

    grid_texture
}

fn generate_scanline_texture(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    width: u32,
    height: u32,
) -> Texture {
    let mut tex = texture_creator
        .create_texture_target(PixelFormatEnum::ABGR8888, width, height)
        .unwrap();

    canvas
        .with_texture_canvas(&mut tex, |subcanvas| {
            subcanvas.set_draw_color(Color::RGBA(0, 0, 0, 40));
            for y in (0..height).step_by(2) {
                // every 2 pixels
                subcanvas
                    .draw_line((0, y as i32), (width as i32, y as i32))
                    .unwrap();
            }
        })
        .unwrap();

    tex.set_blend_mode(sdl2::render::BlendMode::Blend);
    tex
}

fn generate_dot_matrix_texture(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    width: u32,
    height: u32,
) -> Texture {
    let mut tex = texture_creator
        .create_texture_target(PixelFormatEnum::ABGR8888, width, height)
        .unwrap();

    canvas
        .with_texture_canvas(&mut tex, |subcanvas| {
            subcanvas.set_draw_color(Color::RGBA(0, 0, 0, 30));
            let spacing = 4; // distance between dots
            let size = 1; // dot size

            for y in (0..height).step_by(spacing) {
                for x in (0..width).step_by(spacing) {
                    let rect = Rect::new(x as i32, y as i32, size, size);
                    subcanvas.fill_rect(rect).unwrap();
                }
            }
        })
        .unwrap();

    tex.set_blend_mode(sdl2::render::BlendMode::Blend);
    tex
}

fn generate_vignette_texture(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    width: u32,
    height: u32,
) -> Texture {
    let mut tex = texture_creator
        .create_texture_target(PixelFormatEnum::ABGR8888, width, height)
        .unwrap();

    canvas
        .with_texture_canvas(&mut tex, |subcanvas| {
            let center_x = width as f32 / 2.0;
            let center_y = height as f32 / 2.0;
            let max_dist = (center_x.powi(2) + center_y.powi(2)).sqrt();

            for y in 0..height {
                for x in 0..width {
                    let dx = x as f32 - center_x;
                    let dy = y as f32 - center_y;
                    let dist = ((dx * dx + dy * dy).sqrt()) / max_dist;
                    let alpha = (dist * 120.0) as u8; // intensity
                    subcanvas.set_draw_color(Color::RGBA(0, 0, 0, alpha));
                    subcanvas
                        .draw_point(Point::new(x as i32, y as i32))
                        .unwrap();
                }
            }
        })
        .unwrap();

    tex.set_blend_mode(sdl2::render::BlendMode::Blend);
    tex
}
