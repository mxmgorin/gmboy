use crate::video::sdl2_text::{calc_text_width_str, draw_text_lines, fill_texture, get_text_height, CenterText};
use core::ppu::tile::PixelColor;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::VideoSubsystem;

pub struct Layout {
    pub win_width: u32,
    pub win_height: u32,
}

impl Layout {
    pub fn new(scale: u32) -> Self {
        Self {
            win_width: LCD_X_RES as u32 * scale,
            win_height: LCD_Y_RES as u32 * scale,
        }
    }
}

pub struct Sdl2Renderer {
    canvas: Canvas<Window>,
    texture: Texture,
    fps_texture: Texture,
    overlay_texture: Texture,
    layout: Layout,
}

impl Sdl2Renderer {
    pub fn new(scale: u32, video_subsystem: &VideoSubsystem) -> Result<Self, String> {
        let layout = Layout::new(scale);
        let window = video_subsystem
            .window("GMBoy", layout.win_width, layout.win_height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                LCD_X_RES as u32,
                LCD_Y_RES as u32,
            )
            .unwrap();

        let mut overlay_texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                LCD_X_RES as u32,
                LCD_Y_RES as u32,
            )
            .unwrap();
        overlay_texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        let mut fps_texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ARGB8888, 50, 50)
            .unwrap();
        fps_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

        Ok(Self {
            canvas,
            layout,
            texture,
            fps_texture,
            overlay_texture,
        })
    }

    pub fn draw_buffer(&mut self, pixel_buffer: &[u32]) {
        self.canvas.clear();

        self.texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                // use u32 for less indexing
                let pitch_u32 = pitch / 4;
                let buffer_u32 = unsafe {
                    std::slice::from_raw_parts_mut(
                        buffer.as_mut_ptr() as *mut u32,
                        buffer.len() / 4,
                    )
                };

                for y in 0..LCD_Y_RES as usize {
                    for x in 0..LCD_X_RES as usize {
                        let offset = y * pitch_u32 + x;
                        buffer_u32[offset] = pixel_buffer[x + y * LCD_X_RES as usize];
                    }
                }
            })
            .unwrap();

        let (win_width, win_height) = self.canvas.window().size();
        let dest_rect = calculate_scaled_rect(win_width, win_height);

        // Copy the texture while maintaining aspect ratio
        self.canvas
            .copy(&self.texture, None, Some(dest_rect))
            .unwrap();
    }

    pub fn draw_text_lines(
        &mut self,
        lines: &[&str],
        scale: usize,
        color: PixelColor,
        bg_color: PixelColor,
        center: bool,
    ) {
        self.canvas.clear();
        let (center, text_width) = if center {
            let center = CenterText::new(lines, scale);

            (Some(center), center.longest_text_width)
        } else {
            (None, calc_text_width_str(lines[0], scale))
        };
        let text_height = get_text_height(scale) * lines.len();
        let (win_width, win_height) = self.canvas.window().size();
        // Calculate the x and y positions to center the text
        let x = (LCD_X_RES as u32 as usize - text_width) / 2;
        let y = (LCD_Y_RES as u32 as usize - text_height) / 2;

        fill_texture(&mut self.overlay_texture, bg_color);

        draw_text_lines(&mut self.overlay_texture, lines, color, x, y, scale, center);
        let dest_rect = calculate_scaled_rect(win_width, win_height);

        self.canvas
            .copy(&self.overlay_texture, None, Some(dest_rect))
            .unwrap();
    }

    pub fn draw_fps(&mut self, fps: usize, color: PixelColor) {
        let text = fps.to_string();
        fill_texture(&mut self.fps_texture, PixelColor::from_u32(0));
        draw_text_lines(&mut self.fps_texture, &[&text], color, 5, 5, 1, None);

        self.canvas
            .copy(&self.fps_texture, None, Some(Rect::new(0, 0, 80, 80)))
            .unwrap();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn set_scale(&mut self, scale: u32) -> Result<(), String> {
        self.layout = Layout::new(scale);
        let window = self.canvas.window_mut();
        window
            .set_size(self.layout.win_width, self.layout.win_height)
            .map_err(|e| e.to_string())?;
        window.set_position(
            sdl2::video::WindowPos::Centered,
            sdl2::video::WindowPos::Centered,
        );

        Ok(())
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        if fullscreen {
            self.canvas
                .window_mut()
                .set_fullscreen(sdl2::video::FullscreenType::Desktop)
                .unwrap();
        } else {
            self.canvas
                .window_mut()
                .set_fullscreen(sdl2::video::FullscreenType::Off)
                .unwrap();
        }
    }

    pub fn position(&self) -> (i32, i32) {
        self.canvas.window().position()
    }
}

fn calculate_scaled_rect(window_width: u32, window_height: u32) -> Rect {
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
