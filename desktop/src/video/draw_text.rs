use crate::video::BYTES_PER_PIXEL;
use core::ppu::tile::PixelColor;
use sdl2::render::Texture;
use crate::video::char::get_char_bitmap;

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
    texture: &mut Texture,
    lines: &[&str],
    color: PixelColor,
    x: usize, // left edge of the whole block
    y: usize,
    size: FontSize,
    scale: usize,
    align_center: Option<CenterAlignedText>,
) {
    // Compute widest line (in pixels)
    let max_line_width = if let Some(center) = align_center {
        center.longest_text_width
    } else {
        0
    };

    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for (line_index, line) in lines.iter().enumerate() {
                // Compute this line's width
                let mut line_width = 0;

                for c in line.chars() {
                    if c == ' ' || get_char_bitmap(c, size).is_some() {
                        line_width += (size.width() * scale) + size.spacing();
                    }
                }

                if line_width >= size.spacing() {
                    line_width -= size.spacing();
                }

                // Shift shorter lines right to center under longest
                let x_offset = if align_center.is_some() {
                    x + (max_line_width - line_width) / 2
                } else {
                    x
                };

                let y_offset = y + line_index * ((size.height() * scale) + size.spacing());
                let mut cursor_x = x_offset;

                for c in line.chars() {
                    if c == ' ' {
                        cursor_x += (size.width() * scale) + size.spacing();
                        continue;
                    }

                    let Some(bitmap) = get_char_bitmap(c, size) else {
                        //cursor_x += (CHAR_WIDTH * scale) + CHAR_SPACING;
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

                                        let text_offset = (py * pitch) + (px * BYTES_PER_PIXEL);
                                        let (r, g, b, a) = color.as_rgba();
                                        buffer[text_offset] = b;
                                        buffer[text_offset + 1] = g;
                                        buffer[text_offset + 2] = r;
                                        buffer[text_offset + 3] = a;
                                    }
                                }
                            }
                        }
                    }

                    cursor_x += (size.width() * scale) + size.spacing();
                }
            }
        })
        .unwrap();
}

#[derive(Clone, Copy)]
pub enum FontSize {
    Normal,
    Small
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
}

