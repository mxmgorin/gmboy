use crate::video::draw_text::{
    calc_text_height, calc_text_width_str, draw_text_lines, CenterAlignedText, FontSize,
};
use crate::video::fill_texture;
use core::ppu::tile::PixelColor;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::VideoSubsystem;

pub struct GameWindow {
    canvas: Canvas<Window>,
    pub texture: Texture,
    pub popup_texture: Texture,
    popup_rect: Rect,
    fps_rect: Rect,
    game_rect: Rect,
}

impl GameWindow {
    pub fn new(scale: u32, video_subsystem: &VideoSubsystem) -> Result<Self, String> {
        let win_width = calc_win_width(scale);
        let win_height = calc_win_height(scale);
        let window = video_subsystem
            .window("GMBoy", win_width, win_height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                LCD_X_RES as u32,
                LCD_Y_RES as u32,
            )
            .unwrap();
        texture.set_blend_mode(sdl2::render::BlendMode::Blend);
        let (canvas_win_width, canvas_win_height) = canvas.window().size();
        let mut pop_texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                canvas_win_width,
                canvas_win_height,
            )
            .unwrap();
        pop_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

        Ok(Self {
            canvas,
            texture,
            popup_texture: pop_texture,
            popup_rect: Rect::new(0, 0, canvas_win_width, canvas_win_height),
            fps_rect: Rect::new(0, 0, 80, 80),
            game_rect: new_scaled_rect(canvas_win_width, canvas_win_height),
        })
    }

    pub fn draw_buffer(&mut self, pixel_buffer: &[u32]) {
        self.canvas.clear();

        self.texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                let pitch_u32 = pitch / 4;
                let buffer_u32 = unsafe {
                    std::slice::from_raw_parts_mut(
                        buffer.as_mut_ptr() as *mut u32,
                        buffer.len() / 4,
                    )
                };

                if pitch_u32 == LCD_X_RES as usize {
                    buffer_u32.copy_from_slice(pixel_buffer);
                } else {
                    for y in 0..LCD_Y_RES as usize {
                        let dst = y * pitch_u32;
                        let src = y * LCD_X_RES as usize;
                        buffer_u32[dst..dst + LCD_X_RES as usize]
                            .copy_from_slice(&pixel_buffer[src..src + LCD_X_RES as usize]);
                    }
                }
            })
            .unwrap();

        self.canvas
            .copy(&self.texture, None, Some(self.game_rect))
            .unwrap();
    }

    pub fn draw_text_lines(
        &mut self,
        lines: &[&str],
        size: FontSize,
        color: PixelColor,
        bg_color: PixelColor,
        center: bool,
        align_center: bool,
    ) {
        self.canvas.clear();
        let (align_center, text_width) = if align_center {
            let center = CenterAlignedText::new(lines, size);

            (Some(center), center.longest_text_width)
        } else {
            (None, calc_text_width_str(lines[0], size))
        };
        let text_height = calc_text_height(size) * lines.len();
        let mut x = LCD_X_RES as usize - text_width;
        let mut y = LCD_Y_RES as usize - text_height;

        if center {
            x /= 2;
            y /= 2;
        }

        fill_texture(&mut self.texture, bg_color);
        draw_text_lines(&mut self.texture, lines, color, x, y, size, 1, align_center);

        self.canvas
            .copy(&self.texture, None, Some(self.game_rect))
            .unwrap();
    }

    pub fn draw_fps(&mut self, fps: &str, size: FontSize, color: PixelColor) {
        fill_texture(&mut self.texture, PixelColor::from_u32(0));
        draw_text_lines(&mut self.texture, &[fps], color, 5, 5, size, 3, None);

        self.canvas
            .copy(&self.texture, None, Some(self.fps_rect))
            .unwrap();
    }

    pub fn draw_popup(&mut self, lines: &[&str], size: FontSize, color: PixelColor) {
        fill_texture(&mut self.popup_texture, PixelColor::from_u32(0));
        draw_text_lines(&mut self.popup_texture, lines, color, 5, 20, size, 2, None);

        self.canvas
            .copy(&self.popup_texture, None, Some(self.popup_rect))
            .unwrap();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn set_scale(&mut self, scale: u32) -> Result<(), String> {
        let window = self.canvas.window_mut();
        window
            .set_size(calc_win_width(scale), calc_win_height(scale))
            .map_err(|e| e.to_string())?;
        window.set_position(
            sdl2::video::WindowPos::Centered,
            sdl2::video::WindowPos::Centered,
        );
        let (win_width, win_height) = self.canvas.window().size();
        self.game_rect = new_scaled_rect(win_width, win_height);

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

        let (win_width, win_height) = self.canvas.window().size();
        self.game_rect = new_scaled_rect(win_width, win_height);
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.canvas.window().position()
    }
}

fn calc_win_height(scale: u32) -> u32 {
    LCD_Y_RES as u32 * scale
}

fn calc_win_width(scale: u32) -> u32 {
    LCD_X_RES as u32 * scale
}

fn new_scaled_rect(window_width: u32, window_height: u32) -> Rect {
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
