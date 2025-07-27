use crate::config::{FrameAdditiveBlend, FrameBlendType, FrameLinearBlend};
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
    texture: Texture,
    notif_texture: Texture,
    notif_rect: Rect,
    game_rect: Rect,
    pub text_color: PixelColor,
    pub bg_color: PixelColor,
    font_size: FontSize,
    prev_framebuffer: Box<[u32]>,
    pub frame_blend_type: FrameBlendType,
}

impl GameWindow {
    pub fn new(
        scale: u32,
        video_subsystem: &VideoSubsystem,
        text_color: PixelColor,
        bg_color: PixelColor,
        frame_blend_type: FrameBlendType,
    ) -> Result<Self, String> {
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
        let notif_rect = Rect::new(0, 0, win_width / 4, win_width / 4);
        let mut notif_texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                notif_rect.width(),
                notif_rect.height(),
            )
            .unwrap();
        notif_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

        Ok(Self {
            canvas,
            texture,
            notif_texture,
            notif_rect,
            game_rect: new_scaled_rect(canvas_win_width, canvas_win_height),
            text_color,
            bg_color,
            font_size: FontSize::Small,
            prev_framebuffer: Box::new([]),
            frame_blend_type,
        })
    }

    fn frame_blend(&mut self, pixel_buffer: &[u32]) -> bool {
        match &self.frame_blend_type {
            FrameBlendType::None => false,
            FrameBlendType::Linear(x) => self.linear_alpha_blend(pixel_buffer, *x),
            FrameBlendType::Additive(x) => self.addictive_blend(pixel_buffer, *x),
        }
    }

    fn addictive_blend(&mut self, pixel_buffer: &[u32], frame_blend: FrameAdditiveBlend) -> bool {
        let fade = frame_blend.fade;
        let intensity = frame_blend.intensity;

        if self.prev_framebuffer.len() != pixel_buffer.len() {
            self.prev_framebuffer = pixel_buffer.to_vec().into_boxed_slice();
        } else {
            for (i, curr) in pixel_buffer.iter().enumerate() {
                let prev = self.prev_framebuffer[i];
                let curr = *curr;

                // Extract previous channels and fade
                let pr = ((prev >> 16) & 0xFF) as f32 * fade;
                let pg = ((prev >> 8) & 0xFF) as f32 * fade;
                let pb = (prev & 0xFF) as f32 * fade;

                // Extract current channels
                let cr = ((curr >> 16) & 0xFF) as f32 * intensity;
                let cg = ((curr >> 8) & 0xFF) as f32 * intensity;
                let cb = (curr & 0xFF) as f32 * intensity;

                // Add and clamp to 255
                let r = (pr + cr).min(255.0) as u32;
                let g = (pg + cg).min(255.0) as u32;
                let b = (pb + cb).min(255.0) as u32;

                self.prev_framebuffer[i] = (255 << 24) | (r << 16) | (g << 8) | b;
            }
        }

        true
    }

    fn linear_alpha_blend(&mut self, pixel_buffer: &[u32], frame_blend: FrameLinearBlend) -> bool {
        if frame_blend.alpha >= 1.0 {
            return false;
        }

        let frame_blend_alpha = frame_blend.alpha;

        if self.prev_framebuffer.len() != pixel_buffer.len() {
            // Initialize prev_framebuffer on first run
            self.prev_framebuffer = pixel_buffer.to_vec().into_boxed_slice();
        } else {
            // Blend new frame with previous frame
            for (i, curr) in pixel_buffer.iter().enumerate() {
                let prev = self.prev_framebuffer[i];
                let curr = *curr;

                // Decompose ARGB u32 pixels into components
                let pa = ((prev >> 24) & 0xFF) as f32;
                let pr = ((prev >> 16) & 0xFF) as f32;
                let pg = ((prev >> 8) & 0xFF) as f32;
                let pb = (prev & 0xFF) as f32;

                let ca = ((curr >> 24) & 0xFF) as f32;
                let cr = ((curr >> 16) & 0xFF) as f32;
                let cg = ((curr >> 8) & 0xFF) as f32;
                let cb = (curr & 0xFF) as f32;

                // Blend components
                let a = (ca * frame_blend_alpha + pa * (1.0 - frame_blend_alpha)).round() as u32;
                let r = (cr * frame_blend_alpha + pr * (1.0 - frame_blend_alpha)).round() as u32;
                let g = (cg * frame_blend_alpha + pg * (1.0 - frame_blend_alpha)).round() as u32;
                let b = (cb * frame_blend_alpha + pb * (1.0 - frame_blend_alpha)).round() as u32;

                self.prev_framebuffer[i] = (a << 24) | (r << 16) | (g << 8) | b;
            }
        }

        true
    }

    pub fn draw_buffer(&mut self, pixel_buffer: &[u32]) {
        let pixel_buffer = if self.frame_blend(pixel_buffer) {
            &self.prev_framebuffer
        } else {
            pixel_buffer
        };

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

    pub fn draw_text_lines(&mut self, lines: &[&str], center: bool, align_center: bool) {
        self.canvas.clear();
        let (align_center, text_width) = if align_center {
            let center = CenterAlignedText::new(lines, self.font_size);

            (Some(center), center.longest_text_width)
        } else {
            (None, calc_text_width_str(lines[0], self.font_size))
        };
        let text_height = calc_text_height(self.font_size) * lines.len();
        let mut x = LCD_X_RES as usize - text_width;
        let mut y = LCD_Y_RES as usize - text_height;

        if center {
            x /= 2;
            y /= 2;
        }

        fill_texture(&mut self.texture, self.bg_color);
        draw_text_lines(
            &mut self.texture,
            lines,
            self.text_color,
            None,
            x,
            y,
            self.font_size,
            1,
            align_center,
        );

        self.canvas
            .copy(&self.texture, None, Some(self.game_rect))
            .unwrap();
    }

    pub fn draw_fps(&mut self, fps: &str) {
        fill_texture(&mut self.texture, PixelColor::from_u32(0));
        draw_text_lines(
            &mut self.texture,
            &[fps],
            self.text_color,
            Some(self.bg_color),
            10,
            10,
            self.font_size,
            4,
            None,
        );

        self.canvas
            .copy(&self.texture, None, Some(Rect::new(0,0, 80, 80)))
            .unwrap();
    }

    pub fn draw_notification(&mut self, lines: &[&str]) {
        fill_texture(&mut self.notif_texture, PixelColor::from_u32(0));
        draw_text_lines(
            &mut self.notif_texture,
            lines,
            self.text_color,
            Some(self.bg_color),
            10,
            10,
            self.font_size,
            2,
            None,
        );

        self.canvas
            .copy(
                &self.notif_texture,
                None,
                Some(self.notif_rect),
            )
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
