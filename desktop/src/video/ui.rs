use crate::video::draw_text::{
    calc_text_height, calc_text_width_str, draw_text_lines, CenterAlignedText, FontSize,
};
use crate::video::fill_texture;
use core::ppu::tile::PixelColor;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub enum UiTexture {
    Sdl(Texture),
    Gl {
        id: u32,
        width: u32,
        height: u32,
        buffer: Vec<u8>,
    },
}

pub struct UiOverlay {
    font_size: FontSize,
    pub notif_texture: UiTexture,
    pub fps_texture: UiTexture,
    pub menu_texture: UiTexture,
    pub menu_rect: Rect,
    pub fps_rect: Rect,
    pub notif_rect: Rect,
    pub text_color: PixelColor,
    pub bg_color: PixelColor,
}

impl UiOverlay {
    pub fn new_gl(menu_rect: Rect, text_color: PixelColor, bg_color: PixelColor) -> Self {
        fn create_gl_texture(w: u32, h: u32) -> UiTexture {
            let mut id = 0;
            unsafe {
                gl::GenTextures(1, &mut id);
                gl::BindTexture(gl::TEXTURE_2D, id);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RGBA as i32,
                    w as i32,
                    h as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    std::ptr::null(),
                );
            }
            UiTexture::Gl {
                id,
                width: w,
                height: h,
                buffer: vec![0; (w * h * 4) as usize],
            }
        }

        let notif_rect = Rect::new(0, 0, LCD_X_RES as u32 * 3, LCD_Y_RES as u32 * 2);
        let fps_rect = Rect::new(2, 2, 70, 70);

        Self {
            font_size: FontSize::Small,
            notif_texture: create_gl_texture(notif_rect.width(), notif_rect.height()),
            fps_texture: create_gl_texture(fps_rect.width(), fps_rect.height()),
            menu_texture: create_gl_texture(menu_rect.width(), menu_rect.height()),
            menu_rect,
            fps_rect,
            notif_rect,
            bg_color,
            text_color,
        }
    }

    pub fn new_sdl(
        creator: &TextureCreator<WindowContext>,
        menu_rect: Rect,
        text_color: PixelColor,
        bg_color: PixelColor,
    ) -> Self {
        let notif_rect = Rect::new(0, 0, LCD_X_RES as u32 * 3, LCD_Y_RES as u32 * 2);
        let fps_rect = Rect::new(2, 2, 70, 70);

        Self {
            font_size: FontSize::Small,
            notif_texture: UiTexture::Sdl(
                creator
                    .create_texture_streaming(
                        sdl2::pixels::PixelFormatEnum::ARGB8888,
                        notif_rect.width(),
                        notif_rect.height(),
                    )
                    .unwrap(),
            ),
            fps_texture: UiTexture::Sdl(
                creator
                    .create_texture_streaming(
                        sdl2::pixels::PixelFormatEnum::ARGB8888,
                        fps_rect.width(),
                        fps_rect.height(),
                    )
                    .unwrap(),
            ),
            menu_texture: UiTexture::Sdl(
                creator
                    .create_texture_streaming(
                        sdl2::pixels::PixelFormatEnum::ARGB8888,
                        menu_rect.width(),
                        menu_rect.height(),
                    )
                    .unwrap(),
            ),
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

        update_texture(
            &mut self.menu_texture,
            lines,
            x,
            y,
            self.text_color,
            self.bg_color,
            self.font_size
        );
    }

    pub fn update_fps(&mut self, fps: &str) {
        update_texture(
            &mut self.fps_texture,
            &[fps],
            2,
            2,
            self.text_color,
            self.bg_color,
            self.font_size
        );
    }

    pub fn update_notif(&mut self, lines: &[&str]) {
        update_texture(
            &mut self.notif_texture,
            lines,
            10,
            10,
            self.text_color,
            self.bg_color,
            self.font_size
        );
    }
}

fn update_texture(
    texture: &mut UiTexture,
    lines: &[&str],
    x: usize,
    y: usize,
    text_color: PixelColor,
    bg_color: PixelColor,
    font_size: FontSize,
) {
    match texture {
        UiTexture::Sdl(tex) => {
            tex.with_lock(None, |buf, pitch| {
                fill_texture(buf, bg_color);
                draw_text_lines(
                    buf, pitch, lines, text_color, None, x, y, font_size, 1, None,
                );
            })
            .unwrap();
        }
        UiTexture::Gl {
            id,
            buffer,
            width,
            height,
        } => {
            // Draw into CPU buffer
            fill_texture(buffer, bg_color);
            draw_text_lines(
                buffer,
                *width as usize * 4,
                lines,
                text_color,
                None,
                x,
                y,
                font_size,
                1,
                None,
            );
            // Upload to GL
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, *id);
                gl::TexSubImage2D(
                    gl::TEXTURE_2D,
                    0,
                    0,
                    0,
                    *width as i32,
                    *height as i32,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    buffer.as_ptr() as *const _,
                );
            }
        }
    }
}
