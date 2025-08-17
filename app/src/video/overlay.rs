use crate::video::draw_text::{
    fill_str_outlined, fill_text_lines, CenterAlignedText, FontSize, TextLinesStyle, TextStyle,
};
use crate::video::{fill_buffer, FrameBuffer};
use core::ppu::tile::PixelColor;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;

pub struct Overlay {
    pub text_color: PixelColor,
    pub bg_color: PixelColor,
}

impl Overlay {
    pub fn new(text_color: PixelColor, bg_color: PixelColor) -> Self {
        Self {
            bg_color,
            text_color,
        }
    }

    pub fn update_menu(&self, buffer: &mut [u8], lines: &[&str], center: bool, align_center: bool) {
        let menu_width = LCD_X_RES as usize;
        let size = FontSize::Font5x6;

        let (align_center, text_width) = if align_center {
            let center = CenterAlignedText::new(lines, size, menu_width);
            (Some(center), center.max_text_width)
        } else {
            (None, size.calc_text_width(lines[0]))
        };

        let text_height = size.height() * lines.len();
        let lines_height = size.line_spacing() * (lines.len().saturating_sub(1));
        let text_height = text_height + lines_height;
        let mut x = menu_width.saturating_sub(text_width);
        let mut y = LCD_Y_RES as usize - text_height;

        if center {
            x /= 2;
            y /= 2;
        }
        let style = TextLinesStyle {
            text_color: self.text_color,
            bg_color: None,
            size,
            align_center,
        };

        fill_buffer(buffer, self.bg_color, core::ppu::PPU_BYTES_PER_PIXEL);
        fill_text_lines(&mut FrameBuffer::new_ppu(buffer), lines, style, x, y);
    }

    pub fn fill_fps(&mut self, buffer: &mut [u8], text: &str) {
        let style = TextStyle {
            text_color: self.text_color,
            bg_color: self.bg_color,
            size: FontSize::Font3x4,
        };
        let padding = style.size.padding();
        let x = LCD_X_RES as usize - padding - style.size.calc_text_width(text);
        let y = LCD_Y_RES as usize - padding - style.size.height();

        fill_str_outlined(&mut FrameBuffer::new_ppu(buffer), text, style, x, y);
    }

    pub fn fill_notif(&mut self, buffer: &mut [u8], lines: &[&str]) {
        let mut fb = FrameBuffer::new_ppu(buffer);
        let style = TextLinesStyle {
            text_color: self.text_color,
            bg_color: Some(self.bg_color),
            size: FontSize::Font4x5,
            align_center: None,
        };
        let padding = style.size.padding();

        fill_text_lines(&mut fb, lines, style, padding, padding);
    }
}
