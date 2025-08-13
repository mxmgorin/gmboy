use crate::video::draw_text::{
    calc_text_height, calc_text_width_str, draw_text_lines, CenterAlignedText, FontSize,
};
use crate::video::VideoTexture;
use core::ppu::tile::PixelColor;
use sdl2::rect::Rect;

pub struct Overlay {
    pub notif_texture: VideoTexture,
    pub fps_texture: VideoTexture,
    pub menu_texture: VideoTexture,
    pub text_color: PixelColor,
    pub bg_color: PixelColor,
    font_size: FontSize,
    scale: usize,
}

impl Overlay {
    pub fn new(
        menu_rect: Rect,
        fps_rect: Rect,
        notif_rect: Rect,
        text_color: PixelColor,
        bg_color: PixelColor,
        overlay_scale: usize,
    ) -> Self {
        Self {
            font_size: FontSize::Small,
            menu_texture: VideoTexture::new(menu_rect, core::ppu::fetcher::PPU_BYTES_PER_PIXEL),
            fps_texture: VideoTexture::new(fps_rect, 4),
            notif_texture: VideoTexture::new(notif_rect, 4),
            bg_color,
            text_color,
            scale: overlay_scale,
        }
    }

    pub fn update_menu(&mut self, lines: &[&str], center: bool, align_center: bool) {
        let menu_width = self.menu_texture.rect.w as usize;

        let (align_center_opt, text_width) = if align_center {
            let center = CenterAlignedText::new(lines, self.font_size, menu_width);
            (Some(center), center.longest_text_width)
        } else {
            (None, calc_text_width_str(lines[0], self.font_size))
        };

        let text_height = calc_text_height(self.font_size) * lines.len();
        let mut x = menu_width.saturating_sub(text_width);
        let mut y = self.menu_texture.rect.h as usize - text_height;

        if center {
            x /= 2;
            y /= 2;
        }

        self.menu_texture.fill(self.bg_color);
        draw_text_lines(
            &mut self.menu_texture.buffer,
            self.menu_texture.pitch,
            lines,
            self.text_color,
            None,
            x,
            y,
            self.font_size,
            1,
            align_center_opt,
            self.menu_texture.bytes_per_pixel,
        );
    }

    pub fn update_fps(&mut self, fps: &str) {
        self.fps_texture.fill(PixelColor::zero());

        draw_text_lines(
            &mut self.fps_texture.buffer,
            self.fps_texture.pitch,
            &[fps],
            self.text_color,
            Some(self.bg_color),
            self.fps_texture.rect.x as usize,
            self.fps_texture.rect.y as usize,
            self.font_size,
            self.scale,
            None,
            self.fps_texture.bytes_per_pixel,
        );
    }

    pub fn update_notif(&mut self, lines: &[&str]) {
        self.notif_texture.fill(PixelColor::zero());

        draw_text_lines(
            &mut self.notif_texture.buffer,
            self.notif_texture.pitch,
            lines,
            self.text_color,
            Some(self.bg_color),
            self.notif_texture.rect.x as usize,
            self.notif_texture.rect.y as usize,
            self.font_size,
            self.scale,
            None,
            self.notif_texture.bytes_per_pixel,
        );
    }
}
