use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs, io};
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub last_cart_path: Option<String>,
    pub emulation: EmulationConfig,
    pub graphics: GraphicsConfig,
}
impl Config {
    pub fn get_last_cart_file_stem(&self) -> Option<Cow<str>> {
        let path = Path::new(self.last_cart_path.as_ref()?);
        
        Some(path.file_stem().unwrap().to_string_lossy())
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
pub struct GraphicsConfig {
    pub selected_pallet_idx: usize,
    pub pallets: Vec<Pallet>,
    pub scale: f32,
    pub is_fullscreen: bool,
    pub show_fps: bool,
    pub text_scale: usize,
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

        exe_dir.join("config.json")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            last_cart_path: Some("/home/mxmgo/Emulation/Roms/GB/SolarStriker (World).gb".to_string()),
            emulation: EmulationConfig {
                rewind_size: 4000,
                slow_speed: 50.0,
                turbo_speed: 300.0,
                is_muted: false,
            },
            graphics: GraphicsConfig {
                selected_pallet_idx: 0,
                pallets: vec![
                    Pallet {
                        name: "Forgotten Swamp".to_string(),
                        hex_colors: [
                            "ffd1ada1".to_string(),
                            "ff4d7d65".to_string(),
                            "ff593a5f".to_string(),
                            "ff3b252e".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Kirokaze".to_string(),
                        hex_colors: [
                            "ff332c50".to_string(),
                            "ff46878f".to_string(),
                            "ff94e344".to_string(),
                            "ffe2f3e4".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Classic".to_string(),
                        hex_colors: [
                            "ff081820".to_string(),
                            "ff346856".to_string(),
                            "ff88c070".to_string(),
                            "ffe0f8d0".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Rustic".to_string(),
                        hex_colors: [
                            "ff2c2137".to_string(),
                            "ff764462".to_string(),
                            "ffa96868".to_string(),
                            "ffedb4a1".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Swamp".to_string(),
                        hex_colors: [
                            "fffafca4".to_string(),
                            "ffd68e49".to_string(),
                            "ff308013".to_string(),
                            "ff400c01".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Kirokaze".to_string(),
                        hex_colors: [
                            "ff332c50".to_string(),
                            "ff46878f".to_string(),
                            "ff94e344".to_string(),
                            "ffe2f3e4".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Ice Cream".to_string(),
                        hex_colors: [
                            "ff7c3f58".to_string(),
                            "ffeb6b6f".to_string(),
                            "fff9a875".to_string(),
                            "fffff6d3".to_string(),
                        ],
                    },
                    Pallet {
                        name: "2bit demichrome".to_string(),
                        hex_colors: [
                            "ff211e20".to_string(),
                            "ff555568".to_string(),
                            "ffa0a08b".to_string(),
                            "ffe9efec".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Mist".to_string(),
                        hex_colors: [
                            "ff2d1b00".to_string(),
                            "ff1e606e".to_string(),
                            "ff5ab9a8".to_string(),
                            "ffc4f0c2".to_string(),
                        ],
                    },
                    Pallet {
                        name: "AYY4".to_string(),
                        hex_colors: [
                            "ff00303b".to_string(),
                            "ffff7777".to_string(),
                            "ffffce96".to_string(),
                            "fff1f2da".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Wish".to_string(),
                        hex_colors: [
                            "ff622e4c".to_string(),
                            "ff7550e8".to_string(),
                            "ff608fcf".to_string(),
                            "ff8be5ff".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Crimson".to_string(),
                        hex_colors: [
                            "ffeff9d6".to_string(),
                            "ffba5044".to_string(),
                            "ff7a1c4b".to_string(),
                            "ff1b0326".to_string(),
                        ],
                    },
                    Pallet {
                        name: "SpaceHaze".to_string(),
                        hex_colors: [
                            "fff8e3c4".to_string(),
                            "ffcc3495".to_string(),
                            "ff6b1fb1".to_string(),
                            "ff0b0630".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Velvet Cherry".to_string(),
                        hex_colors: [
                            "ff2d162c".to_string(),
                            "ff412752".to_string(),
                            "ff683a68".to_string(),
                            "ff9775a6".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Nintendo Super Gameboy".to_string(),
                        hex_colors: [
                            "ff331e50".to_string(),
                            "ffa63725".to_string(),
                            "ffd68e49".to_string(),
                            "fff7e7c6".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Fiery Plague".to_string(),
                        hex_colors: [
                            "ff1a2129".to_string(),
                            "ff312137".to_string(),
                            "ff512839".to_string(),
                            "ff713141".to_string(),
                        ],
                    },
                    Pallet {
                        name: "Coldfire".to_string(),
                        hex_colors: [
                            "ff46425e".to_string(),
                            "ff5b768d".to_string(),
                            "ffd17c7c".to_string(),
                            "fff6c6a8".to_string(),
                        ],
                    },
                ],
                scale: 5.0,
                is_fullscreen: false,
                show_fps: true,
                text_scale: 1,
            },
        }
    }
}
