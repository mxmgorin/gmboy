use serde::{Deserialize, Serialize};
use std::time::Duration;

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
pub struct EmuPalette {
    pub name: String,
    pub hex_colors: [String; 4],
}

impl EmuPalette {
    pub fn default_palettes() -> Vec<EmuPalette> {
        vec![
            EmuPalette::crimson(),
            EmuPalette::forgotten_swamp(),
            EmuPalette::classic(),
            EmuPalette::rustic(),
            EmuPalette {
                name: "Swamp".to_string(),
                hex_colors: [
                    "fffafca4".to_string(),
                    "ffd68e49".to_string(),
                    "ff308013".to_string(),
                    "ff400c01".to_string(),
                ],
            },
            EmuPalette {
                name: "2bit demichrome".to_string(),
                hex_colors: [
                    "ff211e20".to_string(),
                    "ff555568".to_string(),
                    "ffa0a08b".to_string(),
                    "ffe9efec".to_string(),
                ],
            },
            EmuPalette {
                name: "SpaceHaze".to_string(),
                hex_colors: [
                    "fff8e3c4".to_string(),
                    "ffcc3495".to_string(),
                    "ff6b1fb1".to_string(),
                    "ff0b0630".to_string(),
                ],
            },
            EmuPalette {
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
        EmuPalette {
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
