use crate::video::draw_text::{draw_text_lines, CenterAlignedText, FontSize};
use crate::video::{fill_buffer, VideoTexture};
use core::ppu::tile::PixelColor;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;
use sdl2::rect::Rect;

pub struct Overlay {
    pub notif_texture: VideoTexture,
    pub text_color: PixelColor,
    pub bg_color: PixelColor,
    font_size: FontSize,
    scale: usize,
}

impl Overlay {
    pub fn new(
        notif_rect: Rect,
        text_color: PixelColor,
        bg_color: PixelColor,
        overlay_scale: usize,
    ) -> Self {
        Self {
            font_size: FontSize::Font5x6,
            notif_texture: VideoTexture::new(notif_rect, 4),
            bg_color,
            text_color,
            scale: overlay_scale,
        }
    }

    pub fn update_menu(&self, buffer: &mut [u8], lines: &[&str], center: bool, align_center: bool) {
        let menu_width = LCD_X_RES as usize;

        let (align_center_opt, text_width) = if align_center {
            let center = CenterAlignedText::new(lines, self.font_size, menu_width);
            (Some(center), center.longest_text_width)
        } else {
            (None, self.font_size.calc_text_width(lines[0]))
        };

        let text_height = self.font_size.height() * lines.len();
        let lines_height = self.font_size.line_spacing() * (lines.len().saturating_sub(1));
        let text_height = text_height + lines_height;
        let mut x = menu_width.saturating_sub(text_width);
        let mut y = LCD_Y_RES as usize - text_height;

        if center {
            x /= 2;
            y /= 2;
        }

        fill_buffer(buffer, self.bg_color, core::ppu::PPU_BYTES_PER_PIXEL);
        draw_text_lines(
            buffer,
            core::ppu::PPU_PITCH,
            lines,
            self.text_color,
            None,
            x,
            y,
            self.font_size,
            1,
            align_center_opt,
            core::ppu::PPU_BYTES_PER_PIXEL,
        );
    }

    pub fn update_notif(&mut self, lines: &[&str]) {
        self.notif_texture.clear();

        draw_text_lines(
            &mut self.notif_texture.buffer,
            self.notif_texture.pitch,
            lines,
            self.text_color,
            Some(self.bg_color),
            self.notif_texture.rect.x as usize,
            self.notif_texture.rect.y as usize,
            self.font_size,
            self.scale,
            None,
            self.notif_texture.bytes_per_pixel,
        );
    }

    pub fn draw_hud_to_buff(&mut self, buffer: &mut [u8], text: &str) {
        let font_size = FontSize::Font3x4;
        let padding = font_size.padding();

        draw_text_lines(
            buffer,
            core::ppu::PPU_PITCH,
            &[text],
            self.text_color,
            Some(self.bg_color),
            LCD_X_RES as usize - padding - font_size.calc_text_width(text),
            LCD_Y_RES as usize - padding - font_size.height(),
            font_size,
            1,
            None,
            core::ppu::PPU_BYTES_PER_PIXEL,
        );
    }
}
