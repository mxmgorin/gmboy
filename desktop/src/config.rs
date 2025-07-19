use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs, io};
use core::emu::config::{EmuConfig, EmuPallet};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DesktopEmuConfig {
    pub last_cart_path: Option<String>,
    pub load_save_state_at_start: bool,
    pub emulation: EmuConfig,
    pub graphics: UiConfig,
}
impl DesktopEmuConfig {
    pub fn get_last_cart_file_stem(&self) -> Option<Cow<str>> {
        let path = Path::new(self.last_cart_path.as_ref()?);

        Some(path.file_stem()?.to_string_lossy())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmulationConfig {
    pub rewind_size: usize,
    pub slow_speed: f64,
    pub turbo_speed: f64,
    pub is_muted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiConfig {
    pub selected_pallet_idx: usize,
    pub pallets: Vec<EmuPallet>,
    pub scale: f32,
    pub is_fullscreen: bool,
    pub show_fps: bool,
    pub text_scale: usize,
}

impl DesktopEmuConfig {
    pub fn from_file(path: &str) -> io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&data)?;

        Ok(config)
    }

    pub fn save_file(&self) -> Result<(), io::Error> {
        let save_path = DesktopEmuConfig::default_path();

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

impl Default for DesktopEmuConfig {
    fn default() -> Self {
        Self {
            last_cart_path: None,
            load_save_state_at_start: false,
            emulation: Default::default(),
            graphics: UiConfig {
                selected_pallet_idx: 0,
                pallets: EmuPallet::default_pallets(),
                scale: 5.0,
                is_fullscreen: false,
                show_fps: true,
                text_scale: 1,
            },
        }
    }
}
