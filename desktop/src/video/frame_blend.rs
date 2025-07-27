use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrameBlendMode {
    None,
    Linear(LinearFrameBlend),
    Additive(AdditiveFrameBlend),
    Exponential(ExponentialFrameBlend),
    GammaCorrected(GammaCorrectedFrameBlend),
    Accurate(BlendProfile),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GammaCorrectedFrameBlend {
    pub alpha: f32,
    pub fade: f32,
}

impl Default for GammaCorrectedFrameBlend {
    fn default() -> Self {
        Self {
            alpha: 0.5,
            fade: 0.5,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExponentialFrameBlend {
    pub fade: f32,
}

impl Default for ExponentialFrameBlend {
    fn default() -> Self {
        Self { fade: 0.5 }
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
            FrameBlendMode::Accurate(_) => "Accurate",
        }
    }

    pub fn change_alpha(&mut self, v: f32) {
        match self {
            FrameBlendMode::None => {}
            FrameBlendMode::Linear(x) => {
                x.alpha = core::change_f32_rounded(x.alpha, v).clamp(0.0, 1.0)
            }
            FrameBlendMode::Additive(x) => {
                x.alpha = core::change_f32_rounded(x.alpha, v).clamp(0.0, 1.0)
            }
            FrameBlendMode::Exponential(_) => {}
            FrameBlendMode::GammaCorrected(x) => {
                x.alpha = core::change_f32_rounded(x.alpha, v).clamp(0.0, 1.0)
            }
            FrameBlendMode::Accurate(_) => {}
        }
    }

    pub fn change_fade(&mut self, v: f32) {
        match self {
            FrameBlendMode::None => {}
            FrameBlendMode::Linear(_) => {}
            FrameBlendMode::Additive(x) => {
                x.fade = core::change_f32_rounded(x.fade, v).clamp(0.0, 1.0)
            }
            FrameBlendMode::Exponential(x) => {
                x.fade = core::change_f32_rounded(x.fade, v).clamp(0.0, 1.0)
            }
            FrameBlendMode::GammaCorrected(x) => {
                x.fade = core::change_f32_rounded(x.fade, v).clamp(0.0, 1.0)
            }
            FrameBlendMode::Accurate(_) => {}
        }
    }

    pub fn get_alpha(&self) -> f32 {
        match self {
            FrameBlendMode::None => 0.0,
            FrameBlendMode::Linear(x) => x.alpha,
            FrameBlendMode::Additive(x) => x.alpha,
            FrameBlendMode::Exponential(_) => 0.0,
            FrameBlendMode::GammaCorrected(x) => x.alpha,
            FrameBlendMode::Accurate(_) => 0.0,
        }
    }

    pub fn get_fade(&self) -> f32 {
        match self {
            FrameBlendMode::None => 0.0,
            FrameBlendMode::Linear(_) => 0.0,
            FrameBlendMode::Additive(x) => x.fade,
            FrameBlendMode::Exponential(x) => x.fade,
            FrameBlendMode::GammaCorrected(x) => x.fade,
            FrameBlendMode::Accurate(_) => 0.0,
        }
    }

    pub fn get_profile(&self) -> &str {
        match self {
            FrameBlendMode::None => "",
            FrameBlendMode::Linear(_) => "",
            FrameBlendMode::Additive(_) => "",
            FrameBlendMode::Exponential(_) => "",
            FrameBlendMode::GammaCorrected(_) => "",
            FrameBlendMode::Accurate(x) => x.name(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LinearFrameBlend {
    /// (0.0..1.0), smaller = stronger ghosting
    pub alpha: f32,
}

impl Default for LinearFrameBlend {
    fn default() -> Self {
        Self { alpha: 0.45 }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct AdditiveFrameBlend {
    /// controls trail persistence
    pub fade: f32,
    /// controls how strong new pixels add
    pub alpha: f32,
}

impl Default for AdditiveFrameBlend {
    fn default() -> Self {
        Self {
            fade: 0.65,
            alpha: 0.35,
        }
    }
}

pub const DMG_PROFILE: BlendProfile =
    BlendProfile::new(0.35, 0.08, 0.15, BlendProfileTint::new(0.78, 0.86, 0.71));
pub const POCKET_PROFILE: BlendProfile =
    BlendProfile::new(0.5, 0.15, 0.07, BlendProfileTint::new(1.0, 1.0, 1.0));

pub struct PixelGrid {
    pub enabled: bool,
    pub strength: f32, // 0.0 - 1.0 darkness
    pub softness: f32, // 0.0 = sharp edges
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BlendProfile {
    pub rise: f32,
    pub fall: f32,
    pub bleed: f32,
    pub tint: BlendProfileTint,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BlendProfileTint {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl BlendProfileTint {
    pub const fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }
}

impl BlendProfile {
    pub const fn new(rise: f32, fall: f32, bleed: f32, tint: BlendProfileTint) -> Self {
        Self {
            rise,
            fall,
            bleed,
            tint,
        }
    }

    pub fn name(&self) -> &str {
        if self == &DMG_PROFILE {
            "DMG"
        } else if self == &POCKET_PROFILE {
            "POCKET"
        } else {
            "CUSTOM"
        }
    }
}