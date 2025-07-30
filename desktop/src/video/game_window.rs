use crate::config::VideoConfig;
use crate::video::draw_text::{
    calc_text_height, calc_text_width_str, draw_text_lines, CenterAlignedText, FontSize,
};
use crate::video::fill_texture;
use crate::video::filter::Filters;
use crate::video::frame_blend::{FrameBlend, FrameBlendMode};
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
    notif_texture: Texture,
    fps_texture: Texture,
    menu_texture: Texture,
    fps_rect: Rect,
    notif_rect: Rect,
    game_rect: Rect,
    font_size: FontSize,
    frame_blend: FrameBlend,
    filters: Filters,
    pub text_color: PixelColor,
    pub bg_color: PixelColor,
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

        let notif_rect = Rect::new(
            0,
            0,
            VideoConfig::WIDTH as u32 * 3,
            VideoConfig::HEIGHT as u32 * 2,
        );
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

        let mut menu_texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB8888,
                VideoConfig::WIDTH as u32,
                VideoConfig::HEIGHT as u32,
            )
            .unwrap();
        menu_texture.set_blend_mode(sdl2::render::BlendMode::Blend);

        Ok(Self {
            game_texture,
            notif_texture,
            notif_rect,
            game_rect,
            text_color,
            bg_color,
            font_size: FontSize::Small,
            frame_blend: FrameBlend::new(),
            config,
            fps_texture,
            fps_rect,
            filters: Filters::new(&mut canvas, &texture_creator, game_rect),
            canvas,
            texture_creator,
            menu_texture,
        })
    }

    pub fn draw_buffer(&mut self, pixel_buffer: &[u32]) {
        self.clear();
        let w = VideoConfig::WIDTH;
        let h = VideoConfig::HEIGHT;

        let pixel_buffer = if let FrameBlendMode::None = self.config.frame_blend_mode {
            pixel_buffer
        } else {
            self.frame_blend.process_buffer(pixel_buffer, &self.config)
        };

        self.game_texture
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

        self.clear();
        self.canvas
            .copy(&self.game_texture, None, Some(self.game_rect))
            .unwrap();

        self.filters.apply(&mut self.canvas, &self.config);
    }

    pub fn update_menu(&mut self, lines: &[&str], center: bool, align_center: bool) {
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

        fill_texture(&mut self.menu_texture, self.bg_color);
        draw_text_lines(
            &mut self.menu_texture,
            lines,
            self.text_color,
            None,
            x,
            y,
            self.font_size,
            1,
            align_center,
        );
    }

    pub fn draw_menu(&mut self) {
        self.clear();

        self.canvas
            .copy(&self.menu_texture, None, Some(self.game_rect))
            .unwrap();

        self.filters.apply(&mut self.canvas, &self.config);
    }

    pub fn update_fps(&mut self, fps: &str) {
        fill_texture(&mut self.fps_texture, PixelColor::from_u32(0));

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
    }

    pub fn draw_fps(&mut self) {
        self.canvas
            .copy(&self.fps_texture, None, Some(self.fps_rect))
            .unwrap();
    }

    pub fn update_notif(&mut self, lines: &[&str]) {
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
    }

    pub fn draw_notif(&mut self) {
        self.canvas
            .copy(&self.notif_texture, None, Some(self.notif_rect))
            .unwrap();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn clear(&mut self) {
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
