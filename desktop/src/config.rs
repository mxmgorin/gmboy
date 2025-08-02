use crate::input::config::InputConfig;
use crate::video::frame_blend::FrameBlendMode;
use crate::video::shader::ShaderFrameBlendMode;
use core::apu::apu::ApuConfig;
use core::emu::config::EmuConfig;
use core::ppu::palette::LcdPalette;
use core::ppu::tile::PixelColor;
use core::ppu::LCD_X_RES;
use core::ppu::LCD_Y_RES;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub emulation: EmuConfig, // only for deserialization

    pub auto_save_state: bool,
    pub current_save_index: usize,
    pub current_load_index: usize,
    pub auto_continue: bool,
    pub roms_dir: Option<String>,
    pub interface: InterfaceConfig,
    pub audio: AudioConfig,
    pub video: VideoConfig,
    pub input: InputConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum VideoBackendType {
    Sdl2,
    Gl,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sdl2Config {
    pub grid_enabled: bool,
    pub subpixel_enabled: bool,
    pub dot_matrix_enabled: bool,
    pub scanline_enabled: bool,
    pub vignette_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlConfig {
    pub shader_name: String,
    pub shader_frame_blend_mode: ShaderFrameBlendMode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoConfig {
    pub frame_blend_mode: FrameBlendMode,
    pub blend_dim: f32,
    pub backend: VideoBackendType,
    pub sdl2: Sdl2Config,
    pub gl: GlConfig,
}

impl VideoConfig {
    pub fn change_dim(&mut self, v: f32) {
        self.blend_dim = core::change_f32_rounded(self.blend_dim, v).clamp(0.0, 1.0)
    }

    pub const WIDTH: usize = LCD_X_RES as usize;
    pub const HEIGHT: usize = LCD_Y_RES as usize;
}

impl AppConfig {
    pub fn get_emu_config(&self) -> &EmuConfig {
        &self.emulation
    }

    pub fn set_emu_config(&mut self, config: EmuConfig) {
        self.emulation = config;
    }

    pub fn inc_save_index(&mut self) {
        self.current_save_index = core::move_next_wrapped(self.current_save_index, 99);
    }

    pub fn dec_save_index(&mut self) {
        self.current_save_index = core::move_prev_wrapped(self.current_save_index, 99);
    }

    pub fn inc_load_index(&mut self) {
        self.current_load_index = core::move_next_wrapped(self.current_load_index, 99);
    }

    pub fn dec_load_index(&mut self) {
        self.current_load_index = core::move_prev_wrapped(self.current_load_index, 99);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioConfig {
    pub mute: bool,
    pub mute_turbo: bool,
    pub mute_slow: bool,
    pub buffer_size: usize,
    pub volume: f32,
}

impl AudioConfig {
    pub fn get_apu_config(&self) -> ApuConfig {
        ApuConfig::new(self.buffer_size, self.volume)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterfaceConfig {
    pub selected_palette_idx: usize,
    pub scale: f32,
    pub is_fullscreen: bool,
    pub show_fps: bool,
    pub tile_window: bool,
    pub is_palette_inverted: bool,
}

impl InterfaceConfig {
    pub fn get_palette_colors(&self, palettes: &[LcdPalette]) -> [PixelColor; 4] {
        let idx = self.selected_palette_idx;

        let mut colors = core::into_pixel_colors(&palettes[idx].hex_colors);

        if self.is_palette_inverted {
            colors.reverse();
        }

        colors
    }
}

impl AppConfig {
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&data)?;

        Ok(config)
    }

    pub fn save_file(&self) -> Result<(), io::Error> {
        let save_path = AppConfig::default_path();

        // Open file in write mode, truncating (overwriting) any existing content
        let mut file = File::create(save_path)?;
        let json = serde_json::to_string_pretty(self)?;
        file.write_all(json.as_bytes())
    }

    pub fn default_path() -> PathBuf {
        // Get the directory where the binary is running from
        let exe_path = env::current_exe().expect("Failed to get executable path");
        let exe_dir = exe_path
            .parent()
            .expect("Failed to get executable directory");

        exe_dir.join("config.json")
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let apu_config = ApuConfig::default();

        Self {
            auto_save_state: false,
            current_save_index: 0,
            current_load_index: 0,
            emulation: Default::default(),
            interface: InterfaceConfig {
                selected_palette_idx: 0,
                scale: 5.0,
                is_fullscreen: false,
                show_fps: false,
                tile_window: false,
                is_palette_inverted: false,
            },
            audio: AudioConfig {
                mute: false,
                mute_turbo: true,
                mute_slow: true,
                buffer_size: apu_config.buffer_size,
                volume: apu_config.volume,
            },
            input: InputConfig::default(),
            video: VideoConfig {
                frame_blend_mode: FrameBlendMode::None,
                blend_dim: 1.0,
                backend: VideoBackendType::Sdl2,
                sdl2: Sdl2Config {
                    grid_enabled: true,
                    subpixel_enabled: true,
                    dot_matrix_enabled: false,
                    scanline_enabled: false,
                    vignette_enabled: false,
                },
                gl: GlConfig {
                    shader_name: "passthrough".to_string(),
                    shader_frame_blend_mode: ShaderFrameBlendMode::Simple,
                },
            },
            roms_dir: None,
            auto_continue: false,
        }
    }
}
