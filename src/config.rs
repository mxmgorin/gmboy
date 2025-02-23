use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs, io};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub graphics: GraphicsConfig,
    pub last_cart_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GraphicsConfig {
    pub selected_pallet_idx: usize,
    pub pallets: Vec<Pallet>,
    pub scale: f32,
    pub fps: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pallet {
    pub name: String,
    pub hex_colors: [String; 4],
}

impl Config {
    pub fn from_file(path: &str) -> io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&data)?;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let save_path = Config::default_path();

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

        exe_dir.join("save/config.json")
    }
}
