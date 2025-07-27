use serde::{Deserialize, Serialize};

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