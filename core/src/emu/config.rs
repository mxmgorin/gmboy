use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmuConfig {
    pub rewind_size: usize,
    pub slow_speed: f64,
    pub turbo_speed: f64,
    pub is_muted: bool,
}

impl Default for EmuConfig {
    fn default() -> Self {
        Self {
            rewind_size: 4000,
            slow_speed: 50.0,
            turbo_speed: 300.0,
            is_muted: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmuPallet {
    pub name: String,
    pub hex_colors: [String; 4],
}

impl EmuPallet {
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
            hex_colors:   [
                "0xFFFFFFFF".to_string(),
                "0xFFAAAAAA".to_string(),
                "0xFF555555".to_string(),
                "0xFF000000".to_string(),
            ]
        }
    }

    pub fn default_pallets() -> Vec<EmuPallet> {
        vec![
            EmuPallet::classic(),
            EmuPallet {
                name: "Forgotten Swamp".to_string(),
                hex_colors: [
                    "ffd1ada1".to_string(),
                    "ff4d7d65".to_string(),
                    "ff593a5f".to_string(),
                    "ff3b252e".to_string(),
                ],
            },
            EmuPallet {
                name: "Kirokaze".to_string(),
                hex_colors: [
                    "ff332c50".to_string(),
                    "ff46878f".to_string(),
                    "ff94e344".to_string(),
                    "ffe2f3e4".to_string(),
                ],
            },
            EmuPallet {
                name: "Rustic".to_string(),
                hex_colors: [
                    "ff2c2137".to_string(),
                    "ff764462".to_string(),
                    "ffa96868".to_string(),
                    "ffedb4a1".to_string(),
                ],
            },
            EmuPallet {
                name: "Swamp".to_string(),
                hex_colors: [
                    "fffafca4".to_string(),
                    "ffd68e49".to_string(),
                    "ff308013".to_string(),
                    "ff400c01".to_string(),
                ],
            },
            EmuPallet {
                name: "Kirokaze".to_string(),
                hex_colors: [
                    "ff332c50".to_string(),
                    "ff46878f".to_string(),
                    "ff94e344".to_string(),
                    "ffe2f3e4".to_string(),
                ],
            },
            EmuPallet {
                name: "Ice Cream".to_string(),
                hex_colors: [
                    "ff7c3f58".to_string(),
                    "ffeb6b6f".to_string(),
                    "fff9a875".to_string(),
                    "fffff6d3".to_string(),
                ],
            },
            EmuPallet {
                name: "2bit demichrome".to_string(),
                hex_colors: [
                    "ff211e20".to_string(),
                    "ff555568".to_string(),
                    "ffa0a08b".to_string(),
                    "ffe9efec".to_string(),
                ],
            },
            EmuPallet {
                name: "Mist".to_string(),
                hex_colors: [
                    "ff2d1b00".to_string(),
                    "ff1e606e".to_string(),
                    "ff5ab9a8".to_string(),
                    "ffc4f0c2".to_string(),
                ],
            },
            EmuPallet {
                name: "AYY4".to_string(),
                hex_colors: [
                    "ff00303b".to_string(),
                    "ffff7777".to_string(),
                    "ffffce96".to_string(),
                    "fff1f2da".to_string(),
                ],
            },
            EmuPallet {
                name: "Wish".to_string(),
                hex_colors: [
                    "ff622e4c".to_string(),
                    "ff7550e8".to_string(),
                    "ff608fcf".to_string(),
                    "ff8be5ff".to_string(),
                ],
            },
            EmuPallet {
                name: "Crimson".to_string(),
                hex_colors: [
                    "ffeff9d6".to_string(),
                    "ffba5044".to_string(),
                    "ff7a1c4b".to_string(),
                    "ff1b0326".to_string(),
                ],
            },
            EmuPallet {
                name: "SpaceHaze".to_string(),
                hex_colors: [
                    "fff8e3c4".to_string(),
                    "ffcc3495".to_string(),
                    "ff6b1fb1".to_string(),
                    "ff0b0630".to_string(),
                ],
            },
            EmuPallet {
                name: "Velvet Cherry".to_string(),
                hex_colors: [
                    "ff2d162c".to_string(),
                    "ff412752".to_string(),
                    "ff683a68".to_string(),
                    "ff9775a6".to_string(),
                ],
            },
            EmuPallet {
                name: "Nintendo Super Gameboy".to_string(),
                hex_colors: [
                    "ff331e50".to_string(),
                    "ffa63725".to_string(),
                    "ffd68e49".to_string(),
                    "fff7e7c6".to_string(),
                ],
            },
            EmuPallet {
                name: "Fiery Plague".to_string(),
                hex_colors: [
                    "ff1a2129".to_string(),
                    "ff312137".to_string(),
                    "ff512839".to_string(),
                    "ff713141".to_string(),
                ],
            },
            EmuPallet {
                name: "Coldfire".to_string(),
                hex_colors: [
                    "ff46425e".to_string(),
                    "ff5b768d".to_string(),
                    "ffd17c7c".to_string(),
                    "fff6c6a8".to_string(),
                ],
            },
        ]
    }
}
