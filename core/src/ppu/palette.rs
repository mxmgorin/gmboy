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
            LcdPalette::nostalgia(),
            LcdPalette::crimson(),
            LcdPalette::forgotten_swamp(),
            LcdPalette::bit2_matrix(),
            LcdPalette::purple_dawn(),
            LcdPalette::cave_4(),
            LcdPalette::gb_mcebuius(),
            LcdPalette::candy_pop(),
            LcdPalette::red_is_dead(),
            LcdPalette::rabbit_5pm(),
            LcdPalette::blueberry(),

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
        ]
    }

    pub fn nostalgia() -> Self {
        Self {
            name: "Nostalgia".to_string(),
            hex_colors: [
                "ffd0d058".to_string(),
                "ffa0a840".to_string(),
                "ff708028".to_string(),
                "ff405010".to_string(),
            ],
        }
    }

    pub fn bit2_matrix() -> Self {
        Self {
            name: "2 bit matrix".to_string(),
            hex_colors: [
                "fff2fff2".to_string(),
                "ffadd9bc".to_string(),
                "ff5b8c7c".to_string(),
                "ff0d1a1a".to_string(),
            ],
        }
    }

    // Created by WildLeoKnight
    pub fn purple_dawn() -> Self {
        Self {
            name: "Purple dawn".to_string(),
            hex_colors: [
                "ffeefded".to_string(),
                "ff9a7bbc".to_string(),
                "ff2d757e".to_string(),
                "ff001b2e".to_string(),
            ],
        }
    }

    // Created by Qirlfriend
    pub fn peach_blizzard() -> Self {
        Self {
            name: "Peach blizzard".to_string(),
            hex_colors: [
                "fffbd6ad".to_string(),
                "fff19386".to_string(),
                "ffa54371".to_string(),
                "ff4c388a".to_string(),
            ],
        }
    }

    // Created by Isa
    pub fn blueberry() -> Self {
        Self {
            name: "Blueberry".to_string(),
            hex_colors: [
                "ff280b0b".to_string(),
                "ff6c2e53".to_string(),
                "ffaa73e0".to_string(),
                "ffb0ecf9".to_string(),
            ],
        }
    }

    // Created by RABBITKNG
    pub fn rabbit_5pm() -> Self {
        Self {
            name: "Rabbit 5pm".to_string(),
            hex_colors: [
                "ffffe7cd".to_string(),
                "ffe4a39f".to_string(),
                "ff629098".to_string(),
                "ff4c3457".to_string(),
            ],
        }
    }

    pub fn candy_pop() -> Self {
        Self {
            name: "Candy pop!".to_string(),
            hex_colors: [
                "ff301221".to_string(),
                "ff854576".to_string(),
                "ff9e81d0".to_string(),
                "ffeebff5".to_string(),
            ],
        }
    }

    // Created by Devine Devine
    pub fn red_is_dead() -> Self {
        Self {
            name: "Red is dead".to_string(),
            hex_colors: [
                "fffffcfe".to_string(),
                "ffff0015".to_string(),
                "ff860020".to_string(),
                "ff11070a".to_string(),
            ],
        }
    }

    // Created by Polyducks
    pub fn cave_4() -> Self {
        Self {
            name: "Cave4".to_string(),
            hex_colors: [
                "ffe4cbbf".to_string(),
                "ff938282".to_string(),
                "ff4f4e80".to_string(),
                "ff2c0016".to_string(),
            ],
        }
    }

    // Created by RABBITKNG
    pub fn gb_mcebuius() -> Self {
        Self {
            name: "Mcebuius".to_string(),
            hex_colors: [
                "fff1e0cd".to_string(),
                "ffffa49a".to_string(),
                "ffda3467".to_string(),
                "ff35333f".to_string(),
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
