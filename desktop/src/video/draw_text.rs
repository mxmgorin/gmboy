use crate::video::char::get_char_bitmap;
use crate::video::BYTES_PER_PIXEL;
use core::ppu::tile::PixelColor;
use sdl2::pixels::Color;
use sdl2::render::Texture;

const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 8;
const CHAR_SPACING: usize = 2;
const _CHAR_COLOR: Color = Color::WHITE;
const LINE_SPACING: usize = 3;

/// Calculate the text width based on character count, scale, and character width
pub fn calc_text_width_str(text: &str, scale: usize) -> usize {
    calc_text_width(text.len(), scale)
}

pub fn calc_text_width(len: usize, scale: usize) -> usize {
    len * CHAR_WIDTH * scale + (len - 1) * CHAR_SPACING * scale
}

pub fn calc_text_height(scale: usize) -> usize {
    CHAR_HEIGHT * scale
}

#[derive(Clone, Copy)]
pub struct CenterAlignedText {
    pub longest_text_width: usize,
}

impl CenterAlignedText {
    pub fn new(lines: &[&str], scale: usize) -> Self {
        let len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

        Self {
            longest_text_width: calc_text_width(len, scale),
        }
    }
}

pub fn draw_text_lines(
    texture: &mut Texture,
    lines: &[&str],
    color: PixelColor,
    x: usize, // left edge of the whole block
    y: usize,
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
                    if c == ' ' || get_char_bitmap(c).is_some() {
                        line_width += (CHAR_WIDTH * scale) + CHAR_SPACING;
                    }
                }

                if line_width >= CHAR_SPACING {
                    line_width -= CHAR_SPACING;
                }

                // Shift shorter lines right to center under longest
                let x_offset = if align_center.is_some() {
                    x + (max_line_width - line_width) / 2
                } else {
                    x
                };

                let y_offset = y + line_index * ((CHAR_HEIGHT * scale) + LINE_SPACING);
                let mut cursor_x = x_offset;

                for c in line.chars() {
                    if c == ' ' {
                        cursor_x += (CHAR_WIDTH * scale) + CHAR_SPACING;
                        continue;
                    }

                    let Some(bitmap) = get_char_bitmap(c) else {
                        //cursor_x += (CHAR_WIDTH * scale) + CHAR_SPACING;
                        continue;
                    };

                    for (row, pixel) in bitmap.iter().enumerate() {
                        for col in 0..CHAR_WIDTH {
                            if (pixel >> (7 - col)) & 1 == 1 {
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

                    cursor_x += (CHAR_WIDTH * scale) + CHAR_SPACING;
                }
            }
        })
        .unwrap();
}
