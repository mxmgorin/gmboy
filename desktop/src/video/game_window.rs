use crate::config::VideoConfig;
use crate::video::filter::Filters;
use crate::video::frame_blend::{FrameBlend, FrameBlendMode};
use crate::video::ui::UiOverlay;
use crate::video::BYTES_PER_PIXEL;
use core::ppu::tile::PixelColor;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::VideoSubsystem;

pub struct GameWindow {
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    game_texture: Texture,
    game_rect: Rect,
    frame_blend: FrameBlend,
    filters: Filters,
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
        let window = video_subsystem
            .window("GMBoy", win_width, win_height)
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
        let (canvas_win_width, canvas_win_height) = canvas.window().size();
        let game_rect = new_scaled_rect(canvas_win_width, canvas_win_height);

        Ok(Self {
            frame_blend: FrameBlend::new(),
            filters: Filters::new(&mut canvas, &texture_creator, game_rect),
            ui: UiOverlay::new(&texture_creator, game_rect, text_color, bg_color),
            config,
            game_texture,
            game_rect,
            canvas,
            texture_creator,
        })
    }

    pub fn draw_buffer(&mut self, buffer: &[u32]) {
        let buffer = if let FrameBlendMode::None = self.config.frame_blend_mode {
            buffer
        } else {
            self.frame_blend.process_buffer(buffer, &self.config)
        };

        let pitch = VideoConfig::WIDTH * BYTES_PER_PIXEL;
        self.game_texture
            .update(None, bytemuck::cast_slice(buffer), pitch)
            .unwrap();

        self.clear();
        self.canvas
            .copy(&self.game_texture, None, Some(self.game_rect))
            .unwrap();
        self.filters.apply(&mut self.canvas, &self.config);
    }

    pub fn draw_menu(&mut self) {
        self.clear();
        self.canvas
            .copy(&self.ui.menu_texture, None, Some(self.ui.menu_rect))
            .unwrap();
        self.filters.apply(&mut self.canvas, &self.config);
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
            .set_size(calc_win_width(scale), calc_win_height(scale))
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
