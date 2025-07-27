use core::apu::apu::ApuConfig;
use core::emu::config::EmuConfig;
use core::ppu::palette::LcdPalette;
use core::ppu::tile::PixelColor;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{env, fs, io};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub emulation: EmuConfig, // only for deserialization

    pub last_cart_path: Option<String>,
    pub auto_save_state: bool,
    pub current_save_index: usize,
    pub current_load_index: usize,
    pub interface: InterfaceConfig,
    pub audio: AudioConfig,
    pub input: InputConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputConfig {
    pub combo_interval: Duration,
}

impl AppConfig {
    pub fn get_last_file_stem(&self) -> Option<Cow<str>> {
        let path = Path::new(self.last_cart_path.as_ref()?);

        Some(path.file_stem()?.to_string_lossy())
    }

    pub fn get_emu_config(&self) -> &EmuConfig {
        &self.emulation
    }

    pub fn set_emu_config(&mut self, config: EmuConfig) {
        self.emulation = config;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioConfig {
    pub mute: bool,
    pub mute_turbo: bool,
    pub mute_slow: bool,
    pub buffer_size: usize,
    pub volume: f32,
}

impl AudioConfig {
    pub fn get_apu_config(&self) -> ApuConfig {
        ApuConfig::new(self.buffer_size, self.volume)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrameBlendMode {
    None,
    Linear(LinearFrameBlend),
    Additive(AdditiveFrameBlend),
    Exponential(ExponentialFrameBlend),
    GammaCorrected(GammaCorrectedFrameBlend),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GammaCorrectedFrameBlend {
    pub alpha: f32,
    pub fade: f32,
    pub dim: f32,
}

impl Default for GammaCorrectedFrameBlend {
    fn default() -> Self {
        Self {
            alpha: 0.5,
            fade: 0.5,
            dim: 1.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExponentialFrameBlend {
    pub fade: f32,
    pub dim: f32,
}

impl Default for ExponentialFrameBlend {
    fn default() -> Self {
        Self { fade: 0.5, dim: 1.0 }
    }
}

impl FrameBlendMode {
    pub fn get_name(&self) -> &str {
        match self {
            FrameBlendMode::Linear(_) => "Linear",
            FrameBlendMode::Additive(_) => "Additive",
            FrameBlendMode::None => "None",
            FrameBlendMode::Exponential(_) => "Exponential",
            FrameBlendMode::GammaCorrected(_) => "Gamma",
        }
    }

    pub fn change_alpha(&mut self, v: f32) {
        match self {
            FrameBlendMode::None => { },
            FrameBlendMode::Linear(x) => x.alpha = core::change_f32_rounded(x.alpha, v).clamp(0.0, 1.0),
            FrameBlendMode::Additive(x) => x.alpha = core::change_f32_rounded(x.alpha, v).clamp(0.0, 1.0),
            FrameBlendMode::Exponential(_) => {},
            FrameBlendMode::GammaCorrected(x) => x.alpha = core::change_f32_rounded(x.alpha, v).clamp(0.0, 1.0),
        }
    }

    pub fn change_fade(&mut self, v: f32) {
        match self {
            FrameBlendMode::None => {},
            FrameBlendMode::Linear(_) => {},
            FrameBlendMode::Additive(x) => x.fade = core::change_f32_rounded(x.fade, v).clamp(0.0, 1.0),
            FrameBlendMode::Exponential(x) => x.fade = core::change_f32_rounded(x.fade, v).clamp(0.0, 1.0),
            FrameBlendMode::GammaCorrected(x) => x.fade = core::change_f32_rounded(x.fade, v).clamp(0.0, 1.0),
        }
    }

    pub fn change_dim(&mut self, v: f32) {
        match self {
            FrameBlendMode::None => {},
            FrameBlendMode::Linear(x) => x.dim = core::change_f32_rounded(x.dim, v).clamp(0.0, 1.0),
            FrameBlendMode::Additive(x) => x.dim = core::change_f32_rounded(x.dim, v).clamp(0.0, 1.0),
            FrameBlendMode::Exponential(x) => x.dim = core::change_f32_rounded(x.dim, v).clamp(0.0, 1.0),
            FrameBlendMode::GammaCorrected(x) => x.dim = core::change_f32_rounded(x.dim, v).clamp(0.0, 1.0),
        }
    }

    pub fn get_alpha(&self) -> f32{
        match self {
            FrameBlendMode::None => 0.0,
            FrameBlendMode::Linear(x) => x.alpha,
            FrameBlendMode::Additive(x) => x.alpha,
            FrameBlendMode::Exponential(_) => 0.0,
            FrameBlendMode::GammaCorrected(x) => x.alpha,
        }
    }

    pub fn get_fade(&self) -> f32 {
        match self {
            FrameBlendMode::None => 0.0,
            FrameBlendMode::Linear(_) => 0.0,
            FrameBlendMode::Additive(x) => x.fade,
            FrameBlendMode::Exponential(x) => x.fade,
            FrameBlendMode::GammaCorrected(x) => x.fade,
        }
    }

    pub fn get_dim(&self) -> f32 {
        match self {
            FrameBlendMode::None => 0.0,
            FrameBlendMode::Linear(x) => x.dim,
            FrameBlendMode::Additive(x) => x.dim,
            FrameBlendMode::Exponential(x) => x.dim,
            FrameBlendMode::GammaCorrected(x) => x.dim,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LinearFrameBlend {
    /// (0.0..1.0), smaller = stronger ghosting
    pub alpha: f32,
    pub dim: f32,
}

impl Default for LinearFrameBlend {
    fn default() -> Self {
        Self { alpha: 0.45, dim: 1.0 }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AdditiveFrameBlend {
    /// controls trail persistence
    pub fade: f32,
    /// controls how strong new pixels add
    pub alpha: f32,
    pub dim: f32,

}

impl Default for AdditiveFrameBlend {
    fn default() -> Self {
        Self {
            fade: 0.65,
            alpha: 0.35,
            dim: 1.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterfaceConfig {
    pub selected_palette_idx: usize,
    pub scale: f32,
    pub is_fullscreen: bool,
    pub show_fps: bool,
    pub tile_window: bool,
    pub is_palette_inverted: bool,
    pub frame_blend_mode: FrameBlendMode,
}

impl InterfaceConfig {
    pub fn get_palette_colors(&self, palettes: &[LcdPalette]) -> [PixelColor; 4] {
        let idx = self.selected_palette_idx;

        let mut colors = core::into_pixel_colors(&palettes[idx].hex_colors);

        if self.is_palette_inverted {
            colors.reverse();
        }

        colors
    }
}

impl AppConfig {
    pub fn from_file(path: &Path) -> io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&data)?;

        Ok(config)
    }

    pub fn save_file(&self) -> Result<(), io::Error> {
        let save_path = AppConfig::default_path();

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

impl Default for AppConfig {
    fn default() -> Self {
        let apu_config = ApuConfig::default();

        Self {
            last_cart_path: None,
            auto_save_state: false,
            current_save_index: 0,
            current_load_index: 0,
            emulation: Default::default(),
            interface: InterfaceConfig {
                selected_palette_idx: 0,
                scale: 5.0,
                is_fullscreen: false,
                show_fps: false,
                tile_window: false,
                is_palette_inverted: false,
                frame_blend_mode: FrameBlendMode::None,
            },
            audio: AudioConfig {
                mute: false,
                mute_turbo: true,
                mute_slow: true,
                buffer_size: apu_config.buffer_size,
                volume: apu_config.volume,
            },
            input: InputConfig {
                combo_interval: Duration::from_millis(500),
            },
        }
    }
}
