use crate::config::VideoConfig;
use crate::video::filter::Filters;
use crate::video::game_window::new_scaled_rect;
use crate::video::ui::UiOverlay;
use crate::video::{PixelColor, BYTES_PER_PIXEL};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::VideoSubsystem;

pub struct Sdl2Backend {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    game_texture: Texture,
    game_rect: Rect,
    filters: Filters,
    pub ui: UiOverlay,
}

impl Sdl2Backend {
    pub fn new(
        video_subsystem: VideoSubsystem,
        rect: Rect,
        text_color: PixelColor,
        bg_color: PixelColor,
    ) -> Self {
        let window = video_subsystem
            .window("GMBoy", rect.width(), rect.height())
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();
        let mut game_texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                VideoConfig::WIDTH as u32,
                VideoConfig::HEIGHT as u32,
            )
            .unwrap();
        game_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

        Self {
            filters: Filters::new(&mut canvas, &texture_creator, rect),
            ui: UiOverlay::new(&texture_creator, rect, text_color, bg_color),
            texture_creator,
            canvas,
            game_texture,
            game_rect: rect,
        }
    }

    pub fn draw_buffer(&mut self, buffer: &[u32], config: &VideoConfig) {
        let buffer = bytemuck::cast_slice(buffer);
        let pitch = VideoConfig::WIDTH * BYTES_PER_PIXEL;
        self.game_texture.update(None, buffer, pitch).unwrap();

        self.clear();
        self.canvas
            .copy(&self.game_texture, None, Some(self.game_rect))
            .unwrap();
        self.filters.apply(&mut self.canvas, config);
    }

    pub fn draw_menu(&mut self, config: &VideoConfig) {
        self.clear();
        self.canvas
            .copy(&self.ui.menu_texture, None, Some(self.ui.menu_rect))
            .unwrap();
        self.filters.apply(&mut self.canvas, config);
    }

    pub fn draw_fps(&mut self) {
        self.canvas
            .copy(&self.ui.fps_texture, None, Some(self.ui.fps_rect))
            .unwrap();
    }

    pub fn draw_notif(&mut self) {
        self.canvas
            .copy(&self.ui.notif_texture, None, Some(self.ui.notif_rect))
            .unwrap();
    }

    pub fn show(&mut self) {
        self.canvas.present();
    }

    fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0)); // black
        self.canvas.clear();
    }

    pub fn set_scale(&mut self, scale: u32) -> Result<(), String> {
        let window = self.canvas.window_mut();
        window
            .set_size(
                crate::video::game_window::calc_win_width(scale),
                crate::video::game_window::calc_win_height(scale),
            )
            .map_err(|e| e.to_string())?;
        window.set_position(
            sdl2::video::WindowPos::Centered,
            sdl2::video::WindowPos::Centered,
        );
        self.update_game_rect();

        Ok(())
    }

    fn update_game_rect(&mut self) {
        let (win_width, win_height) = self.canvas.window().size();
        self.game_rect = new_scaled_rect(win_width, win_height);
        self.ui.menu_rect = self.game_rect;
        self.filters = Filters::new(&mut self.canvas, &self.texture_creator, self.game_rect);
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
        self.update_game_rect();
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.canvas.window().position()
    }
}
