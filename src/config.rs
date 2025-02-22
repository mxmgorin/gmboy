use serde::Deserialize;
use std::{fs, io};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub graphics: GraphicsConfig,
}
#[derive(Debug, Deserialize, Clone)]
pub struct GraphicsConfig {
    pub selected_pallet_idx: usize,
    pub pallets: Vec<Pallet>,
    pub scale: f32,
}

#[derive(Debug, Deserialize, Clone)]
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
}
