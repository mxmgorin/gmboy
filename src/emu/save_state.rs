use std::{env, fs};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::bus::Bus;
use crate::Cpu;
use crate::mbc::MbcVariant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmuSaveState {
    pub cpu_without_bus: Cpu,
    pub bus_without_cart: Bus,
    pub cart_mbc: MbcVariant,
}

impl EmuSaveState {
    pub fn save_file(&self, game_name: &str, index: usize) -> Result<(), String> {
        let path = Self::generate_path(game_name, index);

        if let Some(parent) = Path::new(&path).parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let encoded: Vec<u8> = bincode::serialize(self).map_err(|e| e.to_string())?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(&encoded).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn load_file(game_name: &str, index: usize) -> Result<Self, String> {
        let path = Self::generate_path(game_name, index);
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        let decoded = bincode::deserialize(&buffer).map_err(|e| e.to_string())?;

        Ok(decoded)
    }

    pub fn generate_path(game_name: &str, index: usize) -> PathBuf {
        let exe_path = env::current_exe().expect("Failed to get executable path");
        let exe_dir = exe_path
            .parent()
            .expect("Failed to get executable directory");

        exe_dir
            .join("save_states")
            .join(format!("{game_name}_{index}.state"))
    }
}