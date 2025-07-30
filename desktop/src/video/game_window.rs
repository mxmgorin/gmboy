use crate::config::VideoConfig;
use crate::video::frame_blend::{FrameBlend, FrameBlendMode};
use crate::video::gl_backend::GlBackend;
use core::ppu::tile::PixelColor;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;
use sdl2::rect::Rect;
use sdl2::VideoSubsystem;
use crate::video::ui::UiOverlay;

pub struct GameWindow {
    frame_blend: FrameBlend,
    gl_renderer: GlBackend,
    pub config: VideoConfig,
    pub ui: UiOverlay,
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
        let game_rect = new_scaled_rect(win_width, win_height);


        let mut gl_renderer = GlBackend::new(video_subsystem, game_rect);
        gl_renderer.load_shader("crt")?;
        let ui = UiOverlay::new_gl(game_rect, text_color, bg_color);

        Ok(Self {
            frame_blend: FrameBlend::new(),
            config,
            gl_renderer,
            ui
        })
    }

    pub fn draw_buffer(&mut self, buffer: &[u32]) {
        let buffer = if let FrameBlendMode::None = self.config.frame_blend_mode {
            buffer
        } else {
            self.frame_blend.process_buffer(buffer, &self.config)
        };

        self.gl_renderer.draw_buffer(buffer);
    }

    pub fn update_menu(&mut self, lines: &[&str], center: bool, align_center: bool) {}

    pub fn update_notif(&mut self, lines: &[&str]) {}

    pub fn draw_menu(&mut self) {}

    pub fn draw_fps(&mut self) {
        self.gl_renderer.draw_ui_texture_gl(&self.ui.fps_texture, self.ui.fps_rect);
    }

    pub fn draw_notif(&mut self) {}

    pub fn show(&mut self) {
        self.gl_renderer.present();
    }

    pub fn set_scale(&mut self, scale: u32) -> Result<(), String> {
        Ok(())
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {}

    pub fn get_position(&self) -> (i32, i32) {
        (0, 0)
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
