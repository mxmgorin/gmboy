use crate::tile::PixelColor;
use crate::ui::BYTES_PER_PIXEL;
use sdl2::pixels::Color;
use sdl2::render::Texture;

pub const CHAR_WIDTH: usize = 8;
pub const CHAR_HEIGHT: usize = 8;
pub const CHAR_SPACING: usize = 2;
pub const _CHAR_COLOR: Color = Color::WHITE;
pub const BACKGROUND_COLOR: Color = Color::RGBA(0, 0, 0, 0); // transparent

/// Calculate the text width based on character count, scale, and character width
pub fn calc_text_width(text: &str, scale: usize) -> usize {
    text.len() * CHAR_WIDTH * scale + (text.len() - 1) * CHAR_SPACING * scale
}

pub fn get_text_height(scale: usize) -> usize {
    CHAR_HEIGHT * scale
}

pub fn draw_text(
    texture: &mut Texture,
    text: &str,
    color: PixelColor,
    x: usize,
    y: usize,
    scale: usize,
) {
    texture
        .with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for i in (0..buffer.len()).step_by(BYTES_PER_PIXEL) {
                buffer[i] = BACKGROUND_COLOR.r; // R
                buffer[i + 1] = BACKGROUND_COLOR.g; // G
                buffer[i + 2] = BACKGROUND_COLOR.b; // B
                buffer[i + 3] = BACKGROUND_COLOR.a; // A
            }

            let mut x_offset = x;
            for c in text.chars() {
                if c == ' ' {
                    x_offset += (CHAR_WIDTH * scale) + CHAR_SPACING; // Move forward for space
                    continue;
                }

                if let Some(char_index) = get_font_index(c) {
                    if char_index >= FONT.len() {
                        x_offset += (CHAR_WIDTH * scale) + CHAR_SPACING;
                        continue;
                    }

                    let bitmap = FONT[char_index];

                    for (row, pixel) in bitmap.iter().enumerate() {
                        for col in 0..CHAR_WIDTH {
                            if (pixel >> (7 - col)) & 1 == 1 {
                                let text_pixel_x = x_offset + (col * scale);
                                let text_pixel_y = y + (row * scale);

                                for dy in 0..scale {
                                    for dx in 0..scale {
                                        let px = text_pixel_x + dx;
                                        let py = text_pixel_y + dy;

                                        let text_offset = (py * pitch) + (px * BYTES_PER_PIXEL);
                                        let (r, g, b, a) = color.as_rgba();
                                        buffer[text_offset] = r;
                                        buffer[text_offset + 1] = g;
                                        buffer[text_offset + 2] = b;
                                        buffer[text_offset + 3] = a;
                                    }
                                }
                            }
                        }
                    }

                    // Move to the next character with spacing
                    x_offset += (CHAR_WIDTH * scale) + CHAR_SPACING;
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
