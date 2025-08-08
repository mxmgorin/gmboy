use crate::video::char::get_char_bitmap;
use crate::video::BYTES_PER_PIXEL;
use core::ppu::tile::PixelColor;

/// Calculate the text width based on character count, scale, and character width
pub fn calc_text_width_str(text: &str, size: FontSize) -> usize {
    calc_text_width(text.len(), size)
}

pub fn calc_text_width(len: usize, size: FontSize) -> usize {
    len * size.width() + (len - 1) * size.spacing()
}

pub fn calc_text_height(size: FontSize) -> usize {
    size.height()
}

#[derive(Clone, Copy)]
pub struct CenterAlignedText {
    pub longest_text_width: usize,
}

impl CenterAlignedText {
    pub fn new(lines: &[&str], size: FontSize) -> Self {
        let len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

        Self {
            longest_text_width: calc_text_width(len, size),
        }
    }
}

pub fn draw_text_lines(
    buffer: &mut [u8],
    pitch: usize,
    lines: &[&str],
    text_color: PixelColor,
    bg_color: Option<PixelColor>,
    x: usize,
    y: usize,
    size: FontSize,
    scale: usize,
    align_center: Option<CenterAlignedText>,
) {
    if lines.is_empty() {
        return;
    }

    const PADDING: usize = 4;

    let max_line_width = if let Some(center) = align_center {
        center.longest_text_width
    } else if bg_color.is_some() {
        lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if c == ' ' || get_char_bitmap(c, size).is_some() {
                            (size.width() * scale) + size.spacing()
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
                    .saturating_sub(size.spacing())
            })
            .max()
            .unwrap_or(0)
    } else {
        0
    };

    // Compute total height of the text block
    let total_height =
        lines.len() * ((size.height() * scale) + size.line_spacing()) - size.line_spacing();

    // 1. Draw background rectangle with padding
    if let Some(bg_color) = bg_color {
        for py in y.saturating_sub(PADDING)..y + total_height + PADDING {
            for px in x.saturating_sub(PADDING)..x + max_line_width + PADDING {
                let offset = (py * pitch) + (px * BYTES_PER_PIXEL);
                buffer[offset] = bg_color.r;
                buffer[offset + 1] = bg_color.g;
                buffer[offset + 2] = bg_color.b;
                buffer[offset + 3] = bg_color.a;
            }
        }
    }

    // 2. Draw text on top
    for (line_index, line) in lines.iter().enumerate() {
        // Compute line width
        let mut line_width = 0;
        for c in line.chars() {
            if c == ' ' || get_char_bitmap(c, size).is_some() {
                line_width += (size.width() * scale) + size.spacing();
            }
        }
        if line_width >= size.spacing() {
            line_width -= size.spacing();
        }

        // Align center if needed
        let x_offset = if align_center.is_some() {
            x + (max_line_width - line_width) / 2
        } else {
            x
        };

        let y_offset = y + line_index * ((size.height() * scale) + size.line_spacing());
        let mut cursor_x = x_offset;

        for c in line.chars() {
            if c == ' ' {
                cursor_x += (size.width() * scale) + size.spacing();
                continue;
            }

            let Some(bitmap) = get_char_bitmap(c, size) else {
                continue;
            };

            for (row, pixel) in bitmap.iter().enumerate() {
                for col in 0..size.width() {
                    if (pixel >> (size.width() - 1 - col)) & 1 == 1 {
                        let text_pixel_x = cursor_x + (col * scale);
                        let text_pixel_y = y_offset + (row * scale);
                        for dy in 0..scale {
                            for dx in 0..scale {
                                let px = text_pixel_x + dx;
                                let py = text_pixel_y + dy;
                                let offset = (py.wrapping_mul(pitch)) + (px.wrapping_mul(BYTES_PER_PIXEL));
                                buffer[offset] = text_color.r;
                                buffer[offset + 1] = text_color.g;
                                buffer[offset + 2] = text_color.b;
                                buffer[offset + 3] = text_color.a;
                            }
                        }
                    }
                }
            }
            cursor_x += (size.width() * scale) + size.spacing();
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum FontSize {
    Normal,
    Small,
}

impl FontSize {
    fn height(self) -> usize {
        match self {
            FontSize::Normal => 8,
            FontSize::Small => 6,
        }
    }

    fn width(self) -> usize {
        match self {
            FontSize::Normal => 8,
            FontSize::Small => 5,
        }
    }

    fn spacing(self) -> usize {
        match self {
            FontSize::Normal => 2,
            FontSize::Small => 1,
        }
    }

    fn line_spacing(self) -> usize {
        match self {
            FontSize::Normal => 2,
            FontSize::Small => 2,
        }
    }
}
