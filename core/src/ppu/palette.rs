use crate::{get_exe_path, save_json_file};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LcdPalette {
    pub name: String,
    pub hex_colors: [String; 4],
}

impl LcdPalette {
    pub fn save_palettes_file(palettes: &Box<[LcdPalette]>) -> Result<(), io::Error> {
        let path = Self::default_palettes_path();

        save_json_file(&path, palettes)
    }

    pub fn default_palettes_path() -> PathBuf {
        let exe_dir = get_exe_path();

        exe_dir.join("palettes.json")
    }

    pub fn default_palettes() -> Vec<LcdPalette> {
        vec![
            LcdPalette::crimson(),
            LcdPalette::forgotten_swamp(),
            LcdPalette::classic(),
            LcdPalette::rustic(),
            LcdPalette {
                name: "Swamp".to_string(),
                hex_colors: [
                    "fffafca4".to_string(),
                    "ffd68e49".to_string(),
                    "ff308013".to_string(),
                    "ff400c01".to_string(),
                ],
            },
            LcdPalette {
                name: "2bit demichrome".to_string(),
                hex_colors: [
                    "ff211e20".to_string(),
                    "ff555568".to_string(),
                    "ffa0a08b".to_string(),
                    "ffe9efec".to_string(),
                ],
            },
            LcdPalette {
                name: "SpaceHaze".to_string(),
                hex_colors: [
                    "fff8e3c4".to_string(),
                    "ffcc3495".to_string(),
                    "ff6b1fb1".to_string(),
                    "ff0b0630".to_string(),
                ],
            },
            LcdPalette {
                name: "Nintendo Super Gameboy".to_string(),
                hex_colors: [
                    "ff331e50".to_string(),
                    "ffa63725".to_string(),
                    "ffd68e49".to_string(),
                    "fff7e7c6".to_string(),
                ],
            },
        ]
    }

    pub fn classic() -> Self {
        Self {
            name: "Classic".to_string(),
            hex_colors: [
                "ff081820".to_string(),
                "ff346856".to_string(),
                "ff88c070".to_string(),
                "ffe0f8d0".to_string(),
            ],
        }
    }

    pub fn black_white() -> Self {
        Self {
            name: "Back and White".to_string(),
            hex_colors: [
                "ffffffff".to_string(),
                "FFAAAAAA".to_string(),
                "FF555555".to_string(),
                "FF000000".to_string(),
            ],
        }
    }

    pub fn forgotten_swamp() -> Self {
        Self {
            name: "Forgotten Swamp".to_string(),
            hex_colors: [
                "ffd1ada1".to_string(),
                "ff4d7d65".to_string(),
                "ff593a5f".to_string(),
                "ff3b252e".to_string(),
            ],
        }
    }

    pub fn crimson() -> Self {
        Self {
            name: "Crimson".to_string(),
            hex_colors: [
                "ffeff9d6".to_string(),
                "ffba5044".to_string(),
                "ff7a1c4b".to_string(),
                "ff1b0326".to_string(),
            ],
        }
    }

    pub fn rustic() -> Self {
        LcdPalette {
            name: "Rustic".to_string(),
            hex_colors: [
                "ff2c2137".to_string(),
                "ff764462".to_string(),
                "ffa96868".to_string(),
                "ffedb4a1".to_string(),
            ],
        }
    }
}
