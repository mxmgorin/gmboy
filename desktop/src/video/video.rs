use crate::config::{AppConfig, VideoBackendType, VideoConfig};
use crate::video::frame_blend::FrameBlend;
use crate::video::gl_backend::GlBackend;
use crate::video::overlay::Overlay;
use crate::video::sdl2_backend::Sdl2Backend;
use crate::video::tiles::TilesWindow;
use crate::video::{calc_win_height, calc_win_width, new_scaled_rect, VideoBackend};
use core::ppu::tile::PixelColor;
use core::ppu::tile::TileData;
use sdl2::rect::Rect;
use sdl2::{Sdl, VideoSubsystem};

pub struct AppVideo {
    video_subsystem: VideoSubsystem,
    frame_blend: Option<FrameBlend>,
    backend: VideoBackend,
    config: VideoConfig,
    tiles_window: Option<TilesWindow>,
    pub ui: Overlay,
}

impl AppVideo {
    pub fn new(
        sdl: &Sdl,
        text_color: PixelColor,
        bg_color: PixelColor,
        config: &AppConfig,
    ) -> Result<Self, String> {
        let video_subsystem = sdl.video()?;
        let scale = config.interface.scale as u32;
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
        let mut tile_window = None;

        let (mut backend, ui) = match config.video.backend {
            VideoBackendType::Sdl2 => {
                if config.interface.tile_window {
                    tile_window = Some(TilesWindow::new(&video_subsystem));
                }

                create_sdl2_backend(
                    &video_subsystem,
                    game_rect,
                    menu_rect,
                    notif_rect,
                    text_color,
                    bg_color,
                )
            }
            VideoBackendType::Gl => {
                let fps_rect = Rect::new(
                    6,
                    6,
                    VideoConfig::WIDTH as u32 * 3,
                    VideoConfig::WIDTH as u32 * 3,
                );
                let ui = Overlay::new(menu_rect, fps_rect, notif_rect, text_color, bg_color, 1);
                let gl_backend = GlBackend::new(
                    &video_subsystem,
                    game_rect,
                    fps_rect,
                    notif_rect,
                    &config.video.gl,
                );

                if let Ok(gl_backend) = gl_backend {
                    (VideoBackend::Gl(gl_backend), ui)
                } else {
                    println!("Failed to create GL backend. Fallback to SDL2");
                    create_sdl2_backend(
                        &video_subsystem,
                        game_rect,
                        menu_rect,
                        notif_rect,
                        text_color,
                        bg_color,
                    )
                }
            }
        };
        backend.set_fullscreen(config.interface.is_fullscreen);

        Ok(Self {
            frame_blend: FrameBlend::new(&config.video.frame_blend_mode),
            config: config.video.clone(),
            tiles_window: tile_window,
            video_subsystem,
            backend,
            ui,
        })
    }

    /// Closes the window and returns do should quit.
    pub fn close_window(&mut self, id: u32) -> bool {
        if let Some(window) = self.tiles_window.as_mut() {
            if window.get_window_id() == id {
                self.toggle_tile_window();
                return false;
            }
        }

        true
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

    pub fn draw_tiles(&mut self, tiles: impl Iterator<Item = TileData>) {
        if let Some(tiles_window) = self.tiles_window.as_mut() {
            tiles_window.draw_tiles(tiles);
        }
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

    pub fn toggle_tile_window(&mut self) {
        if self.tiles_window.is_some() {
            self.tiles_window = None;
        } else if self.config.backend == VideoBackendType::Sdl2 {
            self.tiles_window = Some(TilesWindow::new(&self.video_subsystem));
        }
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
