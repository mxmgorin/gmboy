use crate::config::VideoConfig;
use crate::video::frame_blend::{FrameBlend, FrameBlendMode};
use crate::video::sdl2_backend::Sdl2Backend;
use crate::video::ui::UiOverlay;
use crate::video::{calc_win_height, calc_win_width, new_scaled_rect};
use core::ppu::tile::PixelColor;
use sdl2::rect::Rect;
use sdl2::VideoSubsystem;

pub struct GameWindow {
    frame_blend: FrameBlend,
    pub backend: Sdl2Backend,
    pub ui: UiOverlay,
    pub config: VideoConfig,
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
        let fps_rect = Rect::new(2, 2, 70, 70);
        let notif_rect = Rect::new(
            0,
            0,
            VideoConfig::WIDTH as u32 * 3,
            VideoConfig::HEIGHT as u32 * 2,
        );

        Ok(Self {
            frame_blend: FrameBlend::new(),
            config,
            backend: Sdl2Backend::new(video_subsystem, game_rect, fps_rect, notif_rect),
            ui: UiOverlay::new(game_rect, fps_rect, notif_rect, text_color, bg_color),
        })
    }

    pub fn draw_buffer(&mut self, buffer: &[u32]) {
        let buffer = if let FrameBlendMode::None = self.config.frame_blend_mode {
            buffer
        } else {
            self.frame_blend.process_buffer(buffer, &self.config)
        };

        self.backend.draw_buffer(buffer, &self.config);
    }

    pub fn draw_menu(&mut self) {
        self.backend.draw_menu(&self.ui.menu_buffer, self.ui.menu_pitch, &self.config)
    }

    pub fn draw_fps(&mut self) {
        self.backend.draw_fps(&self.ui.fps_buffer, self.ui.fps_pitch);
    }

    pub fn draw_notif(&mut self) {
        self.backend.draw_notif(&self.ui.notif_buffer, self.ui.notif_pitch);
    }

    pub fn show(&mut self) {
        self.backend.show();
    }

    pub fn set_scale(&mut self, scale: u32) -> Result<(), String> {
        self.backend.set_scale(scale)
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.backend.set_fullscreen(fullscreen);
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.backend.get_position()
    }
}
