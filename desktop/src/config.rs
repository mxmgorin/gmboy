use core::emu::config::{EmuConfig, ColorPalette};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    emulation: EmuConfig, // only for deserialization

    pub last_cart_path: Option<String>,
    pub save_state_on_exit: bool,
    pub interface: InterfaceConfig,
}

impl AppConfig {
    pub fn get_last_file_stem(&self) -> Option<Cow<str>> {
        let path = Path::new(self.last_cart_path.as_ref()?);

        Some(path.file_stem()?.to_string_lossy())
    }

    pub fn get_emu(&self) -> &EmuConfig {
        &self.emulation
    }

    pub fn set_emu(&mut self, config: EmuConfig) {
        self.emulation = config;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterfaceConfig {
    pub selected_palette_idx: usize,
    pub palettes: Vec<ColorPalette>,
    pub scale: f32,
    pub is_fullscreen: bool,
    pub show_fps: bool,
    pub text_scale: usize,
    pub tile_viewer: bool,
}

impl InterfaceConfig {
    pub fn get_current_palette(&self) -> [core::ppu::lcd::PixelColor; 4] {
        let idx = self.selected_palette_idx;

        core::into_palette(&self.palettes[idx].hex_colors)
    }
}

impl AppConfig {
    pub fn from_file(path: &str) -> io::Result<Self> {
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
        Self {
            last_cart_path: None,
            save_state_on_exit: false,
            emulation: Default::default(),
            interface: InterfaceConfig {
                selected_palette_idx: 0,
                palettes: ColorPalette::default_pallets(),
                scale: 5.0,
                is_fullscreen: false,
                show_fps: false,
                text_scale: 1,
                tile_viewer: false,
            },
        }
    }
}
