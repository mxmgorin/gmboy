use crate::config::VideoConfig;
use crate::video::draw_text::{
    calc_text_height, calc_text_width_str, draw_text_lines, CenterAlignedText, FontSize,
};
use crate::video::fill_texture;
use crate::video::frame_blend::FrameBlendMode;
use core::ppu::tile::PixelColor;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::VideoSubsystem;
use serde::{Deserialize, Serialize};

pub struct PixelGrid {
    pub enabled: bool,
    pub strength: f32, // 0.0 - 1.0 darkness
    pub softness: f32, // 0.0 = sharp edges
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AccurateBlendProfile {
    DMG,    // original Game Boy (greenish)
    Pocket, // Game Boy Pocket (neutral B/W)
}

pub struct BlendProfile {
    pub rise: f32,
    pub fall: f32,
    pub bleed: f32,
    pub tint: (f32, f32, f32),
}

impl BlendProfile {
    pub fn new(rise: f32, fall: f32, bleed: f32, tint: (f32, f32, f32)) -> Self {
        Self {
            rise,
            fall,
            bleed,
            tint,
        }
    }
}

impl AccurateBlendProfile {
    pub fn name(&self) -> &str {
        match self {
            AccurateBlendProfile::DMG => "DMG",
            AccurateBlendProfile::Pocket => "Pocket",
        }
    }

    pub fn get(&self) -> BlendProfile {
        match self {
            AccurateBlendProfile::DMG => BlendProfile::new(0.35, 0.08, 0.15, (0.78, 0.86, 0.71)),
            AccurateBlendProfile::Pocket => BlendProfile::new(0.5, 0.15, 0.07, (1.0, 1.0, 1.0)),
        }
    }
}

pub struct GameWindow {
    canvas: Canvas<Window>,
    texture: Texture,
    notif_texture: Texture,
    fps_texture: Texture,
    fps_rect: Rect,
    notif_rect: Rect,
    game_rect: Rect,
    pub text_color: PixelColor,
    pub bg_color: PixelColor,
    font_size: FontSize,
    prev_framebuffer: Box<[u32]>,
    pub config: VideoConfig,
    pub grid: PixelGrid,
}

impl GameWindow {
    pub fn new(
        scale: u32,
        video_subsystem: &VideoSubsystem,
        text_color: PixelColor,
        bg_color: PixelColor,
        config: VideoConfig,
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
        let notif_rect = Rect::new(0, 0, win_width / 3, win_width / 3);
        let mut notif_texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                notif_rect.width(),
                notif_rect.height(),
            )
            .unwrap();
        notif_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

        let fps_rect = Rect::new(2, 2, 70, 70);
        let mut fps_texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                fps_rect.width(),
                fps_rect.height(),
            )
            .unwrap();
        fps_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

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
            config,
            grid: PixelGrid {
                enabled: false,
                strength: 0.3, // try 0.3-0.5 for visibility
                softness: 1.5, // 1.0 = sharp, >2.0 = smoother
            },
            fps_texture,
            fps_rect,
        })
    }

    fn apply_pixel_grid(&self, i: usize, w: usize, color: (f32, f32, f32)) -> (f32, f32, f32) {
        let grid = &self.grid;
        if !grid.enabled {
            return color;
        }

        let (mut r, mut g, mut b) = color;
        let strength = grid.strength;
        let softness = grid.softness;

        // define how many screen pixels one GB pixel uses
        let scale = 4.0;

        // Coordinates in "screen pixels"
        let px = ((i % w) as f32) * scale;
        let py = ((i / w) as f32) * scale;

        // Position inside the scaled pixel (0..1)
        let subx = (px / scale) % 1.0;
        let suby = (py / scale) % 1.0;

        // Distance to nearest edge
        let edge_dist = (subx.min(1.0 - subx) + suby.min(1.0 - suby)) * 2.0;
        let edge = edge_dist.powf(softness);

        // Darken edges
        let mask = (1.0 - edge * strength).clamp(0.3, 1.0);

        r *= mask;
        g *= mask;
        b *= mask;

        (r, g, b)
    }

    pub fn draw_buffer(&mut self, pixel_buffer: &[u32]) {
        let w = LCD_X_RES as usize;
        let h = LCD_Y_RES as usize;

        let pixel_buffer = if let FrameBlendMode::None = self.config.frame_blend_mode {
            pixel_buffer
        } else {
            if self.prev_framebuffer.len() != pixel_buffer.len() {
                self.prev_framebuffer = pixel_buffer.to_vec().into_boxed_slice();
            }

            for (i, curr) in pixel_buffer.iter().enumerate() {
                let curr = *curr;
                let prev = self.prev_framebuffer[i];

                // Extract RGB
                let (cr, cg, cb) = ((curr >> 16) & 0xFF, (curr >> 8) & 0xFF, curr & 0xFF);
                let (pr, pg, pb) = ((prev >> 16) & 0xFF, (prev >> 8) & 0xFF, prev & 0xFF);

                // --- Use ghosting mode to get blended pixel ---
                let (r, g, b) = self.compute_pixel(i, w, h, (pr, pg, pb), (cr, cg, cb));

                // Store result
                self.prev_framebuffer[i] = (255 << 24) | (r << 16) | (g << 8) | b;
            }

            &self.prev_framebuffer
        };

        self.canvas.clear();
        self.texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                let pitch_u32 = pitch / 4;
                let buf_u32 = unsafe {
                    std::slice::from_raw_parts_mut(
                        buffer.as_mut_ptr() as *mut u32,
                        buffer.len() / 4,
                    )
                };

                for y in 0..h {
                    let dst = y * pitch_u32;
                    let src = y * w;
                    buf_u32[dst..dst + w].copy_from_slice(&pixel_buffer[src..src + w]);
                }
            })
            .unwrap();

        self.canvas
            .copy(&self.texture, None, Some(self.game_rect))
            .unwrap();
    }

    pub fn compute_pixel(
        &self,
        i: usize,
        w: usize,
        h: usize,
        prev: (u32, u32, u32),
        curr: (u32, u32, u32),
    ) -> (u32, u32, u32) {
        let (pr, pg, pb) = prev;
        let (cr, cg, cb) = curr;

        // Convert to linear [0..1]
        let pr_lin = pr as f32 / 255.0;
        let pg_lin = pg as f32 / 255.0;
        let pb_lin = pb as f32 / 255.0;

        let cr_lin = cr as f32 / 255.0;
        let cg_lin = cg as f32 / 255.0;
        let cb_lin = cb as f32 / 255.0;

        // Final RGB values
        let (mut lr, mut lg, mut lb) = match &self.config.frame_blend_mode {
            FrameBlendMode::None => (cr_lin, cg_lin, cb_lin),
            FrameBlendMode::Linear(x) => {
                let a = x.alpha;
                (
                    pr_lin * (1.0 - a) + cr_lin * a,
                    pg_lin * (1.0 - a) + cg_lin * a,
                    pb_lin * (1.0 - a) + cb_lin * a,
                )
            }

            FrameBlendMode::Exponential(x) => {
                let fade = x.fade;
                (
                    pr_lin * fade + cr_lin * (1.0 - fade),
                    pg_lin * fade + cg_lin * (1.0 - fade),
                    pb_lin * fade + cb_lin * (1.0 - fade),
                )
            }
            FrameBlendMode::Additive(x) => {
                let mut fr = pr_lin * x.fade + cr_lin * x.alpha;
                let mut fg = pg_lin * x.fade + cg_lin * x.alpha;
                let mut fb = pb_lin * x.fade + cb_lin * x.alpha;

                // Clamp to prevent overexposure
                fr = fr.min(1.0);
                fg = fg.min(1.0);
                fb = fb.min(1.0);

                (fr, fg, fb)
            }

            FrameBlendMode::GammaCorrected(x) => {
                let gamma = 2.2;
                let fade = x.fade;

                fn to_linear(v: f32, g: f32) -> f32 {
                    v.powf(g)
                }
                fn to_srgb(v: f32, g: f32) -> f32 {
                    v.powf(1.0 / g)
                }

                let pr_l = to_linear(pr_lin, gamma);
                let pg_l = to_linear(pg_lin, gamma);
                let pb_l = to_linear(pb_lin, gamma);

                let cr_l = to_linear(cr_lin, gamma);
                let cg_l = to_linear(cg_lin, gamma);
                let cb_l = to_linear(cb_lin, gamma);

                (
                    to_srgb(pr_l * fade + cr_l * (1.0 - fade), gamma),
                    to_srgb(pg_l * fade + cg_l * (1.0 - fade), gamma),
                    to_srgb(pb_l * fade + cb_l * (1.0 - fade), gamma),
                )
            }

            FrameBlendMode::Accurate(x) => {
                let x = x.get();

                // Scanline timing & jitter
                let y = i / w;
                let jitter = ((y as f32).sin() * 0.003) + 1.0;
                let scan_delay = (1.0 - (y as f32 / h as f32) * 0.2) * jitter;

                fn lcd_step(prev: f32, curr: f32, rise: f32, fall: f32, delay: f32) -> f32 {
                    let rate = if curr > prev { rise } else { fall };
                    prev + (curr - prev) * rate * delay
                }

                // LCD response curve
                let mut lr = lcd_step(pr_lin, cr_lin, x.rise, x.fall, scan_delay);
                let mut lg = lcd_step(pg_lin, cg_lin, x.rise, x.fall, scan_delay);
                let mut lb = lcd_step(pb_lin, cb_lin, x.rise, x.fall, scan_delay);

                // Pixel bleeding (left + top)
                let left_idx = if i % w == 0 { i } else { i - 1 };
                let top_idx = if y == 0 { i } else { i - w };

                let left = self.prev_framebuffer[left_idx];
                let top = self.prev_framebuffer[top_idx];

                let lpr = ((left >> 16) & 0xFF) as f32 / 255.0;
                let lpg = ((left >> 8) & 0xFF) as f32 / 255.0;
                let lpb = (left & 0xFF) as f32 / 255.0;

                let tpr = ((top >> 16) & 0xFF) as f32 / 255.0;
                let tpg = ((top >> 8) & 0xFF) as f32 / 255.0;
                let tpb = (top & 0xFF) as f32 / 255.0;

                lr = lr * (1.0 - x.bleed) + ((lpr + tpr) * 0.5) * x.bleed;
                lg = lg * (1.0 - x.bleed) + ((lpg + tpg) * 0.5) * x.bleed;
                lb = lb * (1.0 - x.bleed) + ((lpb + tpb) * 0.5) * x.bleed;

                // Tint
                lr *= x.tint.0;
                lg *= x.tint.1;
                lb *= x.tint.2;

                (lr, lg, lb)
            }
        };

        (lr, lg, lb) = self.apply_pixel_grid(i, w, (lr, lg, lb));

        let dim = self.config.dim;
        lr *= dim;
        lg *= dim;
        lb *= dim;

        // Convert back to 0..255
        (
            (lr * 255.0).clamp(0.0, 255.0) as u32,
            (lg * 255.0).clamp(0.0, 255.0) as u32,
            (lb * 255.0).clamp(0.0, 255.0) as u32,
        )
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

    pub fn draw_fps(&mut self, fps: &str, clear: bool) {
        if clear {
            fill_texture(&mut self.fps_texture, PixelColor::from_u32(0));
        }

        draw_text_lines(
            &mut self.fps_texture,
            &[fps],
            self.text_color,
            Some(self.bg_color),
            2,
            2,
            self.font_size,
            2,
            None,
        );

        self.canvas
            .copy(&self.fps_texture, None, Some(self.fps_rect))
            .unwrap();
    }

    pub fn draw_notification(&mut self, lines: &[&str], clear: bool) {
        if clear {
            fill_texture(&mut self.notif_texture, PixelColor::from_u32(0));
        }

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
