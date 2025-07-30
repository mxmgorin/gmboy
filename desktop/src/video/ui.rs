use crate::config::VideoConfig;
use crate::video::draw_text::{
    calc_text_height, calc_text_width_str, draw_text_lines, CenterAlignedText, FontSize,
};
use crate::video::fill_texture;
use core::ppu::tile::PixelColor;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct UiOverlay {
    font_size: FontSize,
    pub notif_texture: Texture,
    pub fps_texture: Texture,
    pub menu_texture: Texture,
    pub fps_rect: Rect,
    pub notif_rect: Rect,
    pub menu_rect: Rect,
    pub text_color: PixelColor,
    pub bg_color: PixelColor,
}

impl UiOverlay {
    pub fn new(
        texture_creator: &TextureCreator<WindowContext>,
        menu_rect: Rect,
        text_color: PixelColor,
        bg_color: PixelColor,
    ) -> Self {
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

        Self {
            font_size: FontSize::Small,
            notif_texture,
            fps_texture,
            menu_texture,
            menu_rect,
            fps_rect,
            notif_rect,
            bg_color,
            text_color,
        }
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

        self.menu_texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                fill_texture(buffer, self.bg_color);

                draw_text_lines(
                    buffer,
                    pitch,
                    lines,
                    self.text_color,
                    None,
                    x,
                    y,
                    self.font_size,
                    1,
                    align_center,
                );
            })
            .unwrap();
    }

    pub fn update_fps(&mut self, fps: &str) {
        self.fps_texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                fill_texture(buffer, PixelColor::from_u32(0));

                draw_text_lines(
                    buffer,
                    pitch,
                    &[fps],
                    self.text_color,
                    Some(self.bg_color),
                    2,
                    2,
                    self.font_size,
                    2,
                    None,
                );
            })
            .unwrap();
    }

    pub fn update_notif(&mut self, lines: &[&str]) {
        self.fps_texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                fill_texture(buffer, PixelColor::from_u32(0));

                draw_text_lines(
                    buffer,
                    pitch,
                    lines,
                    self.text_color,
                    Some(self.bg_color),
                    10,
                    10,
                    self.font_size,
                    2,
                    None,
                );
            })
            .unwrap();
    }
}
