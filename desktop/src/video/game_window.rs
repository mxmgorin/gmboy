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
use crate::video::frame_blend::FrameBlendMode;

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
    pub frame_blend_type: FrameBlendMode,
}

impl GameWindow {
    pub fn new(
        scale: u32,
        video_subsystem: &VideoSubsystem,
        text_color: PixelColor,
        bg_color: PixelColor,
        frame_blend_type: FrameBlendMode,
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
        if self.prev_framebuffer.len() != pixel_buffer.len() {
            self.prev_framebuffer = pixel_buffer.to_vec().into_boxed_slice();
        }

        let dim = self.frame_blend_type.get_dim();

        for (i, curr) in pixel_buffer.iter().enumerate() {
            let curr = *curr;
            let prev = self.prev_framebuffer[i];

            let (cr, cg, cb) = ((curr >> 16) & 0xFF, (curr >> 8) & 0xFF, curr & 0xFF);

            let (pr, pg, pb) = ((prev >> 16) & 0xFF, (prev >> 8) & 0xFF, prev & 0xFF);

            let (r, g, b) = match &self.frame_blend_type {
                FrameBlendMode::None => return false,
                FrameBlendMode::Linear(x) => {
                    let alpha = x.alpha;
                    (
                        ((cr as f32 * alpha + pr as f32 * (1.0 - alpha)) as u32),
                        ((cg as f32 * alpha + pg as f32 * (1.0 - alpha)) as u32),
                        ((cb as f32 * alpha + pb as f32 * (1.0 - alpha)) as u32),
                    )
                }
                FrameBlendMode::Exponential(x) => {
                    let fr = (pr as f32 * x.fade) as u32;
                    let fg = (pg as f32 * x.fade) as u32;
                    let fb = (pb as f32 * x.fade) as u32;

                    if cr | cg | cb == 0 {
                        (fr, fg, fb)
                    } else {
                        (cr, cg, cb)
                    }
                }
                FrameBlendMode::Additive(x) => {
                    let fr = (pr as f32 * x.fade + cr as f32 * x.alpha).min(255.0) as u32;
                    let fg = (pg as f32 * x.fade + cg as f32 * x.alpha).min(255.0) as u32;
                    let fb = (pb as f32 * x.fade + cb as f32 * x.alpha).min(255.0) as u32;
                    (fr, fg, fb)
                }
                FrameBlendMode::GammaCorrected(x) => {
                    // convert to linear
                    let (lr, lg, lb) = (
                        srgb_to_linear(pr as u8),
                        srgb_to_linear(pg as u8),
                        srgb_to_linear(pb as u8),
                    );
                    let (crl, cgl, cbl) = (
                        srgb_to_linear(cr as u8),
                        srgb_to_linear(cg as u8),
                        srgb_to_linear(cb as u8),
                    );

                    // blend in linear space
                    let br = lr * x.fade + crl * x.alpha;
                    let bg = lg * x.fade + cgl * x.alpha;
                    let bb = lb * x.fade + cbl * x.alpha;

                    // convert back
                    (
                        linear_to_srgb(br) as u32,
                        linear_to_srgb(bg) as u32,
                        linear_to_srgb(bb) as u32,
                    )
                }
            };

            // apply final dim factor
            let rf = ((r as f32) * dim).min(255.0) as u32;
            let gf = ((g as f32) * dim).min(255.0) as u32;
            let bf = ((b as f32) * dim).min(255.0) as u32;

            self.prev_framebuffer[i] = (255 << 24) | (rf << 16) | (gf << 8) | bf;
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
            .copy(&self.texture, None, Some(Rect::new(0, 0, 80, 80)))
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
            .copy(&self.notif_texture, None, Some(self.notif_rect))
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

fn srgb_to_linear(c: u8) -> f32 {
    let cf = c as f32 / 255.0;
    cf.powf(2.2)
}

fn linear_to_srgb(c: f32) -> u8 {
    (c.powf(1.0 / 2.2) * 255.0).min(255.0).max(0.0) as u8
}
