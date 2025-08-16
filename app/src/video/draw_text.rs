use crate::video::char::get_char_bitmap;
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
    pub fn new(lines: &[&str], size: FontSize, max: usize) -> Self {
        let len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

        Self {
            longest_text_width: calc_text_width(len, size).min(max),
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
    bytes_per_pixel: usize,
) {
    if lines.is_empty() {
        return;
    }


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

    let total_height =
        lines.len() * ((size.height() * scale) + size.line_spacing()) - size.line_spacing();

    // Draw background rectangle with padding
    if let Some(bg_color) = bg_color {
        const BG_PADDING: usize = 4;

        for py in y.saturating_sub(BG_PADDING)..y + total_height + BG_PADDING {
            for px in x.saturating_sub(BG_PADDING)..x + max_line_width + BG_PADDING {
                let offset = (py * pitch) + (px * bytes_per_pixel);
                draw_colors(buffer, offset, bg_color, bytes_per_pixel);
            }
        }
    }

    // Draw text on top
    for (line_index, line) in lines.iter().enumerate() {
        let mut line_width = 0;
        for c in line.chars() {
            if c == ' ' || get_char_bitmap(c, size).is_some() {
                line_width += (size.width() * scale) + size.spacing();
            }
        }
        if line_width >= size.spacing() {
            line_width -= size.spacing();
        }

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
                                let offset = (py.saturating_mul(pitch))
                                    + (px.saturating_mul(bytes_per_pixel));
                                draw_colors(buffer, offset, text_color, bytes_per_pixel);
                            }
                        }
                    }
                }
            }
            cursor_x += (size.width() * scale) + size.spacing();
        }
    }
}

pub fn draw_colors(buffer: &mut [u8], offset: usize, color: PixelColor, bytes_per_pixel: usize) {
    if bytes_per_pixel == 2 {
        let colors = color.as_rgb565_bytes();
        buffer[offset] = colors[0];
        buffer[offset + 1] = colors[1];
    } else {
        let colors = color.as_rgba_bytes();
        buffer[offset] = colors[0];
        buffer[offset + 1] = colors[1];
        buffer[offset + 2] = colors[2];

        if bytes_per_pixel == 4 {
            buffer[offset + 3] = colors[3];
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
