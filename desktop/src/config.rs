use core::apu::apu::ApuConfig;
use core::emu::config::EmuConfig;
use core::ppu::palette::LcdPalette;
use core::ppu::tile::PixelColor;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, fs, io};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub emulation: EmuConfig, // only for deserialization

    pub last_cart_path: Option<String>,
    pub auto_save_state: bool,
    pub current_save_index: usize,
    pub current_load_index: usize,
    pub interface: InterfaceConfig,
    pub audio: AudioConfig,
    pub input: InputConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputConfig {
    pub combo_interval: Duration,
}

impl AppConfig {
    pub fn get_last_file_stem(&self) -> Option<Cow<str>> {
        let path = Path::new(self.last_cart_path.as_ref()?);

        Some(path.file_stem()?.to_string_lossy())
    }

    pub fn get_emu_config(&self) -> &EmuConfig {
        &self.emulation
    }

    pub fn set_emu_config(&mut self, config: EmuConfig) {
        self.emulation = config;
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
            last_cart_path: None,
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
            input: InputConfig {
                combo_interval: Duration::from_millis(500),
            },
        }
    }
}
