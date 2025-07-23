use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmuConfig {
    pub rewind_size: usize,
    pub rewind_interval: Duration,
    pub normal_speed: f64,
    pub slow_speed: f64,
    pub turbo_speed: f64,
    pub spin_duration: Duration,
}

impl Default for EmuConfig {
    fn default() -> Self {
        Self {
            rewind_size: 120,
            rewind_interval: Duration::from_secs(2),
            normal_speed: 1.0,
            slow_speed: 0.5,
            turbo_speed: 5.0,
            spin_duration: Duration::from_millis(1),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColorPalette {
    pub name: String,
    pub hex_colors: [String; 4],
}

impl ColorPalette {
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
                "0xFFFFFFFF".to_string(),
                "0xFFAAAAAA".to_string(),
                "0xFF555555".to_string(),
                "0xFF000000".to_string(),
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

    pub fn default_pallets() -> Vec<ColorPalette> {
        vec![
            ColorPalette::crimson(),
            ColorPalette::forgotten_swamp(),
            ColorPalette::classic(),
            ColorPalette {
                name: "Rustic".to_string(),
                hex_colors: [
                    "ff2c2137".to_string(),
                    "ff764462".to_string(),
                    "ffa96868".to_string(),
                    "ffedb4a1".to_string(),
                ],
            },
            ColorPalette {
                name: "Swamp".to_string(),
                hex_colors: [
                    "fffafca4".to_string(),
                    "ffd68e49".to_string(),
                    "ff308013".to_string(),
                    "ff400c01".to_string(),
                ],
            },
            ColorPalette {
                name: "2bit demichrome".to_string(),
                hex_colors: [
                    "ff211e20".to_string(),
                    "ff555568".to_string(),
                    "ffa0a08b".to_string(),
                    "ffe9efec".to_string(),
                ],
            },
            ColorPalette {
                name: "Mist".to_string(),
                hex_colors: [
                    "ff2d1b00".to_string(),
                    "ff1e606e".to_string(),
                    "ff5ab9a8".to_string(),
                    "ffc4f0c2".to_string(),
                ],
            },
            ColorPalette {
                name: "AYY4".to_string(),
                hex_colors: [
                    "ff00303b".to_string(),
                    "ffff7777".to_string(),
                    "ffffce96".to_string(),
                    "fff1f2da".to_string(),
                ],
            },
            ColorPalette {
                name: "Wish".to_string(),
                hex_colors: [
                    "ff622e4c".to_string(),
                    "ff7550e8".to_string(),
                    "ff608fcf".to_string(),
                    "ff8be5ff".to_string(),
                ],
            },
            ColorPalette {
                name: "SpaceHaze".to_string(),
                hex_colors: [
                    "fff8e3c4".to_string(),
                    "ffcc3495".to_string(),
                    "ff6b1fb1".to_string(),
                    "ff0b0630".to_string(),
                ],
            },
            ColorPalette {
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
}
