use crate::bus::Bus;
use crate::cart::CartSaveState;
use crate::cpu::Cpu;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum EmuState {
    Running,
    Rewind,
}

pub enum SaveStateCmd {
    Create,
    Load,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmuSaveState {
    pub cpu: Cpu,
    pub bus_without_cart: Bus,
    pub cart_save_state: CartSaveState,
}

impl EmuSaveState {
    pub fn save_file(&self, name: &str, suffix: &str) -> Result<(), String> {
        let path = Self::generate_path(name, suffix);

        if let Some(parent) = Path::new(&path).parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        let encoded: Vec<u8> = bincode::serialize(self).map_err(|e| e.to_string())?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(&encoded).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn load_file(name: &str, suffix: &str) -> Result<Self, String> {
        let path = Self::generate_path(name, suffix);
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        let decoded = bincode::deserialize(&buffer).map_err(|e| e.to_string())?;

        Ok(decoded)
    }

    pub fn generate_path(game_name: &str, suffix: &str) -> PathBuf {
        let exe_path = env::current_exe().expect("Failed to get executable path");
        let exe_dir = exe_path
            .parent()
            .expect("Failed to get executable directory");

        exe_dir
            .join("save_states")
            .join(format!("{game_name}_{suffix}.state"))
    }
}
