use crate::video::draw_text::{
    calc_text_height, calc_text_width_str, draw_text_lines, CenterAlignedText, FontSize,
};
use crate::video::{fill_texture, BYTES_PER_PIXEL};
use core::ppu::tile::PixelColor;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;
use sdl2::rect::Rect;

pub struct UiOverlay {
    pub notif_buffer: Box<[u8]>,
    pub fps_buffer: Box<[u8]>,
    pub menu_buffer: Box<[u8]>,
    pub text_color: PixelColor,
    pub bg_color: PixelColor,
    font_size: FontSize,
    pub notif_pitch: usize,
    pub fps_pitch: usize,
    pub menu_pitch: usize,
}

impl UiOverlay {
    pub fn new(
        menu_rect: Rect,
        fps_rect: Rect,
        notif_rect: Rect,
        text_color: PixelColor,
        bg_color: PixelColor,
    ) -> Self {
        let menu_pitch = menu_rect.w as usize * BYTES_PER_PIXEL;
        let fps_pitch = fps_rect.w as usize * BYTES_PER_PIXEL;
        let notif_pitch = notif_rect.w as usize * BYTES_PER_PIXEL;

        Self {
            font_size: FontSize::Small,
            menu_buffer: vec![0; menu_pitch * menu_rect.h as usize].into_boxed_slice(),
            fps_buffer: vec![0; fps_pitch * fps_rect.h as usize].into_boxed_slice(),
            notif_buffer: vec![0; notif_pitch * notif_rect.h as usize].into_boxed_slice(),
            notif_pitch,
            fps_pitch,
            menu_pitch,
            bg_color,
            text_color,
        }
    }

    pub fn update_menu(&mut self, lines: &[&str], center: bool, align_center: bool) {
        let (align_center_opt, text_width) = if align_center {
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

        fill_texture(&mut self.menu_buffer, self.bg_color);

        draw_text_lines(
            &mut self.menu_buffer,
            self.menu_pitch,
            lines,
            self.text_color,
            None,
            x,
            y,
            self.font_size,
            1,
            align_center_opt,
        );
    }

    pub fn update_fps(&mut self, fps: &str) {
        fill_texture(&mut self.fps_buffer, PixelColor::from_u32(0));

        draw_text_lines(
            &mut self.fps_buffer,
            self.fps_pitch,
            &[fps],
            self.text_color,
            Some(self.bg_color),
            2,
            2,
            self.font_size,
            2,
            None,
        );
    }

    pub fn update_notif(&mut self, lines: &[&str]) {
        fill_texture(&mut self.notif_buffer, PixelColor::from_u32(0));

        draw_text_lines(
            &mut self.notif_buffer,
            self.notif_pitch,
            lines,
            self.text_color,
            Some(self.bg_color),
            10,
            10,
            self.font_size,
            2,
            None,
        );
    }
}
