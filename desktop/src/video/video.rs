use crate::config::{VideoBackendType, VideoConfig};
use crate::video::frame_blend::FrameBlend;
use crate::video::gl_backend::GlBackend;
use crate::video::sdl2_backend::Sdl2Backend;
use crate::video::overlay::Overlay;
use crate::video::{
    calc_win_height, calc_win_width, new_scaled_rect, VideoBackend, BYTES_PER_PIXEL,
};
use core::ppu::tile::PixelColor;
use sdl2::rect::Rect;
use sdl2::VideoSubsystem;

pub struct VideoTexture {
    pub pitch: usize,
    pub buffer: Box<[u8]>,
    pub rect: Rect,
}

impl VideoTexture {
    pub fn new(rect: Rect, bytes_per_pixel: usize) -> Self {
        let pitch = rect.w as usize * bytes_per_pixel;

        Self {
            pitch,
            buffer: vec![0; pitch * rect.h as usize].into_boxed_slice(),
            rect,
        }
    }

    pub fn fill(&mut self, color: PixelColor) {
        let (r, g, b, a) = color.as_rgba();

        for i in (0..self.buffer.len()).step_by(BYTES_PER_PIXEL) {
            self.buffer[i] = r;
            self.buffer[i + 1] = g;
            self.buffer[i + 2] = b;
            self.buffer[i + 3] = a;
        }
    }
}

pub struct AppVideo {
    frame_blend: Option<FrameBlend>,
    backend: VideoBackend,
    pub ui: Overlay,
    config: VideoConfig,
}

impl AppVideo {
    pub fn new(
        scale: u32,
        video_subsystem: &VideoSubsystem,
        text_color: PixelColor,
        bg_color: PixelColor,
        config: VideoConfig,
    ) -> Result<Self, String> {
        let win_width = calc_win_width(scale);
        let win_height = calc_win_height(scale);
        let game_rect = new_scaled_rect(win_width, win_height);
        let menu_rect = Rect::new(0, 0, VideoConfig::WIDTH as u32, VideoConfig::HEIGHT as u32);
        let notif_rect = Rect::new(
            6,
            6,
            VideoConfig::WIDTH as u32 * 3,
            VideoConfig::HEIGHT as u32 * 3,
        );

        let (backend, ui) = match config.backend {
            VideoBackendType::Sdl2 => create_sdl2_backend(
                video_subsystem,
                game_rect,
                menu_rect,
                notif_rect,
                text_color,
                bg_color,
            ),
            VideoBackendType::Gl => {
                let fps_rect = Rect::new(
                    6,
                    6,
                    VideoConfig::WIDTH as u32 * 3,
                    VideoConfig::WIDTH as u32 * 3,
                );
                let ui = Overlay::new(menu_rect, fps_rect, notif_rect, text_color, bg_color, 1);
                let gl_backend =
                    GlBackend::new(video_subsystem, game_rect, fps_rect, notif_rect, &config.gl);

                if let Ok(gl_backend) = gl_backend {
                    (VideoBackend::Gl(gl_backend), ui)
                } else {
                    println!("Failed to create GL backend. Fallback to SDL2");
                    create_sdl2_backend(
                        video_subsystem,
                        game_rect,
                        menu_rect,
                        notif_rect,
                        text_color,
                        bg_color,
                    )
                }
            }
        };

        Ok(Self {
            frame_blend: FrameBlend::new(&config.frame_blend_mode),
            config,
            backend,
            ui,
        })
    }

    pub fn update_config(&mut self, config: &VideoConfig) {
        self.frame_blend = FrameBlend::new(&config.frame_blend_mode);
        self.backend.update_config(config);
        self.config = config.clone();
    }

    pub fn draw_buffer(&mut self, buffer: &[u32]) {
        let buffer = if let Some(blend) = &mut self.frame_blend {
            blend.process_buffer(buffer, &self.config)
        } else {
            buffer
        };

        let buffer: &[u8] = bytemuck::cast_slice(buffer);
        self.backend.draw_buffer(buffer, &self.config);
    }

    pub fn draw_menu(&mut self) {
        self.backend.draw_menu(&self.ui.menu_texture, &self.config)
    }

    pub fn draw_fps(&mut self) {
        self.backend.draw_fps(&self.ui.fps_texture);
    }

    pub fn draw_notif(&mut self) {
        self.backend.draw_notif(&self.ui.notif_texture);
    }

    pub fn show(&mut self) {
        self.backend.show();
    }

    pub fn set_scale(&mut self, scale: u32) -> Result<(), String> {
        self.backend.set_scale(scale)
    }

    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        self.backend.set_fullscreen(fullscreen);
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.backend.get_position()
    }
}

pub fn create_sdl2_backend(
    video_subsystem: &VideoSubsystem,
    game_rect: Rect,
    menu_rect: Rect,
    notif_rect: Rect,
    text_color: PixelColor,
    bg_color: PixelColor,
) -> (VideoBackend, Overlay) {
    let fps_rect = Rect::new(6, 6, 76, 76);
    let ui = Overlay::new(menu_rect, fps_rect, notif_rect, text_color, bg_color, 2);
    let backend = Sdl2Backend::new(video_subsystem, game_rect, fps_rect, notif_rect);

    (VideoBackend::Sdl2(backend), ui)
}
