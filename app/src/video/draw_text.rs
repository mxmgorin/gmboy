use crate::video::font::get_char_bitmap;
use crate::video::{draw_color, FrameBuffer};
use core::ppu::tile::PixelColor;

pub struct TextStyle {
    pub text_color: PixelColor,
    pub bg_color: PixelColor,
    pub size: FontSize,
}

pub struct TextLinesStyle {
    pub text_color: PixelColor,
    pub bg_color: Option<PixelColor>,
    pub size: FontSize,
    pub align_center: Option<CenterAlignedText>,
}

#[derive(Clone, Copy)]
pub struct CenterAlignedText {
    pub max_text_width: usize,
}

impl CenterAlignedText {
    pub fn new(lines: &[&str], size: FontSize, max: usize) -> Self {
        let len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

        Self {
            max_text_width: size.calc_len_width(len).min(max),
        }
    }
}

#[inline]
pub fn fill_str_outlined(fb: &mut FrameBuffer, line: &str, style: TextStyle, x: usize, y: usize) {
    let padding = style.size.padding();
    let rect_w = x + style.size.calc_text_width(line) + padding;
    let rect_h = y + style.size.height() + padding;
    let rect_x = x.saturating_sub(padding);
    let rect_y = y.saturating_sub(padding);

    fill_str_rect(fb, rect_w, rect_h, style.bg_color, rect_x, rect_y);
    fill_str(fb, line, style.text_color, x, y, style.size);
}

#[inline]
pub fn fill_str_rect(
    fb: &mut FrameBuffer,
    w: usize,
    h: usize,
    color: PixelColor,
    x: usize,
    y: usize,
) {
    for py in y..h {
        for px in x..w {
            let offset = (py * fb.pitch) + (px * fb.bytes_per_pixel);
            draw_color(fb.buffer, offset, color, fb.bytes_per_pixel);
        }
    }
}

#[inline]
pub fn fill_str(
    fb: &mut FrameBuffer,
    line: &str,
    text_color: PixelColor,
    mut cursor_x: usize,
    y: usize,
    size: FontSize,
) {
    for c in line.chars() {
        let bitmap = get_char_bitmap(c, size);

        for (row, pixel) in bitmap.iter().enumerate() {
            for col in 0..size.width() {
                if (pixel >> (size.width() - 1 - col)) & 1 == 1 {
                    let text_pixel_x = cursor_x + (col);
                    let text_pixel_y = y + (row);
                    let px = text_pixel_x;
                    let py = text_pixel_y;
                    let offset =
                        (py.saturating_mul(fb.pitch)) + (px.saturating_mul(fb.bytes_per_pixel));

                    draw_color(fb.buffer, offset, text_color, fb.bytes_per_pixel);
                }
            }
        }

        cursor_x += (size.width()) + size.spacing();
    }
}

pub fn fill_text_lines(
    fb: &mut FrameBuffer,
    lines: &[&str],
    style: TextLinesStyle,
    x: usize,
    y: usize,
) {
    if lines.is_empty() {
        return;
    }

    let max_line_width = if let Some(center) = style.align_center {
        center.max_text_width
    } else if style.bg_color.is_some() {
        lines
            .iter()
            .map(|line| style.size.calc_text_width(line))
            .max()
            .unwrap_or(0)
    } else {
        0
    };

    // Draw background rectangle with padding
    if let Some(bg_color) = style.bg_color {
        let text_height = style.size.height() * lines.len();
        let lines_height = style.size.line_spacing() * (lines.len().saturating_sub(1));
        let text_height = text_height + lines_height;

        let padding = style.size.padding();
        let w = x + max_line_width + padding;
        let h = y + text_height + padding;
        let x = y.saturating_sub(padding);
        let y = x.saturating_sub(padding);
        fill_str_rect(fb, w, h, bg_color, x, y);
    }

    // Draw text on top
    for (line_index, line) in lines.iter().enumerate() {
        let mut line_width = style.size.calc_text_width(line);

        if line_width >= style.size.spacing() {
            line_width -= style.size.spacing();
        }

        let x_offset = if style.align_center.is_some() {
            x + ((max_line_width - line_width) / 2)
        } else {
            x
        };

        let y_offset = y + line_index * ((style.size.height()) + style.size.line_spacing());
        fill_str(fb, line, style.text_color, x_offset, y_offset, style.size);
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
#[repr(u8)]
pub enum FontSize {
    Font3x4,
    Font4x5,
    Font5x5,
    Font5x6,
    Font8x8,
}

impl FontSize {
    #[inline]
    pub const fn height(self) -> usize {
        match self {
            FontSize::Font8x8 => 8,
            FontSize::Font5x6 => 6,
            FontSize::Font3x4 => 4,
            FontSize::Font4x5 => 5,
            FontSize::Font5x5 => 5,
        }
    }

    #[inline]
    pub const fn width(self) -> usize {
        match self {
            FontSize::Font8x8 => 8,
            FontSize::Font5x6 => 5,
            FontSize::Font3x4 => 3,
            FontSize::Font4x5 => 4,
            FontSize::Font5x5 => 5,
        }
    }

    #[inline]
    pub const fn spacing(self) -> usize {
        match self {
            FontSize::Font8x8 => 2,
            FontSize::Font5x6 => 1,
            FontSize::Font3x4 => 1,
            FontSize::Font4x5 => 1,
            FontSize::Font5x5 => 1,
        }
    }

    #[inline]
    pub const fn line_spacing(self) -> usize {
        match self {
            FontSize::Font8x8 => 2,
            FontSize::Font5x6 => 2,
            FontSize::Font3x4 => 1,
            FontSize::Font4x5 => 1,
            FontSize::Font5x5 => 2,
        }
    }

    #[inline]
    pub const fn padding(self) -> usize {
        match self {
            FontSize::Font8x8 => 4,
            FontSize::Font5x6 => 4,
            FontSize::Font3x4 => 1,
            FontSize::Font4x5 => 2,
            FontSize::Font5x5 => 2,
        }
    }

    /// Calculate the text width based on character count, scale, and character width
    #[inline]
    pub fn calc_text_width(&self, text: &str) -> usize {
        self.calc_len_width(text.chars().count())
    }

    #[inline]
    pub fn calc_len_width(&self, len: usize) -> usize {
        len * self.width() + (len - 1) * self.spacing()
    }
}
