use core::ppu::tile::PixelColor;
use sdl2::pixels::Color;
use sdl2::render::Texture;

const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 8;
const CHAR_SPACING: usize = 2;
const _CHAR_COLOR: Color = Color::WHITE;
const BYTES_PER_PIXEL: usize = 4;
const LINE_SPACING: usize = 3;

/// Calculate the text width based on character count, scale, and character width
pub fn calc_text_width_str(text: &str, scale: usize) -> usize {
    calc_text_width(text.len(), scale)
}

pub fn calc_text_width(len: usize, scale: usize) -> usize {
    len * CHAR_WIDTH * scale + (len - 1) * CHAR_SPACING * scale
}

pub fn get_text_height(scale: usize) -> usize {
    CHAR_HEIGHT * scale
}

pub fn fill_texture(texture: &mut Texture, color: PixelColor) {
    let (r, g, b, a) = color.as_rgba();

    texture
        .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            for i in (0..buffer.len()).step_by(BYTES_PER_PIXEL) {
                buffer[i] = r;
                buffer[i + 1] = g;
                buffer[i + 2] = b;
                buffer[i + 3] = a;
            }
        })
        .unwrap();
}

#[derive(Clone, Copy)]
pub struct CenterText {
    pub longest_text_width: usize,
}

impl CenterText {
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
    center: Option<CenterText>,
) {
    // Compute widest line (in pixels)
    let max_line_width = if let Some(center) = center {
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
                    if c == ' ' || get_font_index(c).is_some() {
                        line_width += (CHAR_WIDTH * scale) + CHAR_SPACING;
                    }
                }
                if line_width >= CHAR_SPACING {
                    line_width -= CHAR_SPACING;
                }

                // Shift shorter lines right to center under longest
                let x_offset = if center.is_some() {
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

                    let Some(char_index) = get_font_index(c) else {
                        continue;
                    };

                    if char_index >= FONT.len() {
                        cursor_x += (CHAR_WIDTH * scale) + CHAR_SPACING;
                        continue;
                    }

                    let bitmap = FONT[char_index];

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

fn get_font_index(c: char) -> Option<usize> {
    match c {
        'A'..='Z' => Some((c as usize) - ('A' as usize)), // A=0, B=1, ..., Z=25
        '0'..='9' => Some((c as usize) - ('0' as usize) + 26), // 0=26, 1=27, ..., 9=35
        _ => None,                                        // Unsupported character
    }
}

const FONT: [[u8; 8]; 36] = [
    // A-Z
    [
        0b01111110, 0b10000001, 0b10000001, 0b11111111, 0b10000001, 0b10000001, 0b10000001,
        0b00000000,
    ], // 'A'
    [
        0b11111110, 0b10000001, 0b10000001, 0b11111110, 0b10000001, 0b10000001, 0b11111110,
        0b00000000,
    ], // 'B'
    [
        0b01111110, 0b10000001, 0b10000000, 0b10000000, 0b10000000, 0b10000001, 0b01111110,
        0b00000000,
    ], // 'C'
    [
        0b11111100, 0b10000010, 0b10000001, 0b10000001, 0b10000001, 0b10000010, 0b11111100,
        0b00000000,
    ], // 'D'
    [
        0b11111111, 0b10000000, 0b10000000, 0b11111110, 0b10000000, 0b10000000, 0b11111111,
        0b00000000,
    ], // 'E'
    [
        0b11111111, 0b10000000, 0b10000000, 0b11111110, 0b10000000, 0b10000000, 0b10000000,
        0b00000000,
    ], // 'F'
    [
        0b01111110, 0b10000001, 0b10000000, 0b10001111, 0b10000001, 0b10000001, 0b01111110,
        0b00000000,
    ], // 'G'
    [
        0b10000001, 0b10000001, 0b10000001, 0b11111111, 0b10000001, 0b10000001, 0b10000001,
        0b00000000,
    ], // 'H'
    [
        0b00111100, 0b00011000, 0b00011000, 0b00011000, 0b00011000, 0b00011000, 0b00111100,
        0b00000000,
    ], // 'I'
    [
        0b00011111, 0b00000110, 0b00000110, 0b00000110, 0b00000110, 0b10000110, 0b01111100,
        0b00000000,
    ], // 'J'
    [
        0b10000001, 0b10000010, 0b10000100, 0b11111000, 0b10000100, 0b10000010, 0b10000001,
        0b00000000,
    ], // 'K'
    [
        0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b11111111,
        0b00000000,
    ], // 'L'
    [
        0b10000001, 0b11000011, 0b10100101, 0b10011001, 0b10000001, 0b10000001, 0b10000001,
        0b00000000,
    ], // 'M'
    [
        0b10000001, 0b11000001, 0b10100001, 0b10010001, 0b10001001, 0b10000101, 0b10000011,
        0b00000000,
    ], // 'N'
    [
        0b01111110, 0b10000001, 0b10000001, 0b10000001, 0b10000001, 0b10000001, 0b01111110,
        0b00000000,
    ], // 'O'
    [
        0b11111110, 0b10000001, 0b10000001, 0b11111110, 0b10000000, 0b10000000, 0b10000000,
        0b00000000,
    ], // 'P'
    [
        0b01111110, 0b10000001, 0b10000001, 0b10000001, 0b10001001, 0b10000101, 0b01111110,
        0b00000001,
    ], // 'Q'
    [
        0b11111110, 0b10000001, 0b10000001, 0b11111110, 0b10001000, 0b10000100, 0b10000010,
        0b00000000,
    ], // 'R'
    [
        0b01111110, 0b10000001, 0b10000000, 0b01111110, 0b00000001, 0b10000001, 0b01111110,
        0b00000000,
    ], // 'S'
    [
        0b11111111, 0b00011000, 0b00011000, 0b00011000, 0b00011000, 0b00011000, 0b00011000,
        0b00000000,
    ], // 'T'
    [
        0b10000001, 0b10000001, 0b10000001, 0b10000001, 0b10000001, 0b10000001, 0b01111110,
        0b00000000,
    ], // 'U'
    [
        0b10000001, 0b10000001, 0b10000001, 0b01000010, 0b01000010, 0b00100100, 0b00011000,
        0b00000000,
    ], // 'V'
    [
        0b10000001, 0b10000001, 0b10000001, 0b10011001, 0b10011001, 0b01100110, 0b01100110,
        0b00000000,
    ], // 'W'
    [
        0b10000001, 0b01000010, 0b00100100, 0b00011000, 0b00100100, 0b01000010, 0b10000001,
        0b00000000,
    ], // 'X'
    [
        0b10000001, 0b01000010, 0b00100100, 0b00011000, 0b00011000, 0b00011000, 0b00011000,
        0b00000000,
    ], // 'Y'
    [
        0b11111111, 0b00000010, 0b00000100, 0b00001000, 0b00010000, 0b00100000, 0b11111111,
        0b00000000,
    ], // 'Z'
    // 0-9
    [
        0b01111110, 0b10000001, 0b10000011, 0b10100101, 0b11000001, 0b10000001, 0b01111110,
        0b00000000,
    ], // '0'
    [
        0b00011000, 0b00111000, 0b00011000, 0b00011000, 0b00011000, 0b00011000, 0b01111110,
        0b00000000,
    ], // '1'
    [
        0b01111110, 0b10000001, 0b00000001, 0b00001110, 0b00110000, 0b01000000, 0b11111111,
        0b00000000,
    ], // '2'
    [
        0b01111110, 0b10000001, 0b00000001, 0b00111110, 0b00000001, 0b10000001, 0b01111110,
        0b00000000,
    ], // '3'
    [
        0b00001000, 0b00011000, 0b00101000, 0b01001000, 0b11111111, 0b00001000, 0b00001000,
        0b00000000,
    ], // '4'
    [
        0b11111111, 0b10000000, 0b10000000, 0b11111110, 0b00000001, 0b10000001, 0b01111110,
        0b00000000,
    ], // '5'
    [
        0b01111110, 0b10000001, 0b10000000, 0b11111110, 0b10000001, 0b10000001, 0b01111110,
        0b00000000,
    ], // '6'
    [
        0b11111111, 0b00000001, 0b00000010, 0b00000100, 0b00001000, 0b00010000, 0b00100000,
        0b00000000,
    ], // '7'
    [
        0b01111110, 0b10000001, 0b10000001, 0b01111110, 0b10000001, 0b10000001, 0b01111110,
        0b00000000,
    ], // '8'
    [
        0b01111110, 0b10000001, 0b10000001, 0b01111111, 0b00000001, 0b10000001, 0b01111110,
        0b00000000,
    ], // '9'
];
