use crate::config::VideoConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrameBlendMode {
    None,
    Linear(LinearFrameBlend),
    Additive(AdditiveFrameBlend),
    Exp(ExponentialFrameBlend),
    Gamma(GammaCorrectedFrameBlend),
    Accurate(BlendProfile),
}

pub struct FrameBlend {
    prev_framebuffer: Box<[u32]>,
}

impl FrameBlend {
    pub fn new() -> Self {
        Self {
            prev_framebuffer: vec![0; VideoConfig::WIDTH * VideoConfig::HEIGHT].into_boxed_slice(),
        }
    }

    pub fn process_buffer(&mut self, pixel_buffer: &[u32], config: &VideoConfig) -> &[u32] {
        debug_assert_eq!(self.prev_framebuffer.len(), pixel_buffer.len());
        let w = VideoConfig::WIDTH;
        let h = VideoConfig::HEIGHT;

        for (i, &curr) in pixel_buffer.iter().enumerate() {
            let prev = self.prev_framebuffer[i];

            let (cr, cg, cb) = ((curr >> 16) & 0xFF, (curr >> 8) & 0xFF, curr & 0xFF);
            let (pr, pg, pb) = ((prev >> 16) & 0xFF, (prev >> 8) & 0xFF, prev & 0xFF);

            let (r, g, b) = self.compute_pixel(config, i, w, h, (pr, pg, pb), (cr, cg, cb));

            self.prev_framebuffer[i] = (255 << 24) | (r << 16) | (g << 8) | b;
        }

        &self.prev_framebuffer
    }

    pub fn compute_pixel(
        &self,
        config: &VideoConfig,
        i: usize,
        w: usize,
        h: usize,
        prev: (u32, u32, u32),
        curr: (u32, u32, u32),
    ) -> (u32, u32, u32) {
        let (pr, pg, pb) = prev;
        let (cr, cg, cb) = curr;

        // Convert to linear [0..1]
        let pr_lin = pr as f32 / 255.0;
        let pg_lin = pg as f32 / 255.0;
        let pb_lin = pb as f32 / 255.0;

        let cr_lin = cr as f32 / 255.0;
        let cg_lin = cg as f32 / 255.0;
        let cb_lin = cb as f32 / 255.0;

        // Final RGB values
        let (mut lr, mut lg, mut lb) = match &config.frame_blend_mode {
            FrameBlendMode::None => (cr_lin, cg_lin, cb_lin),
            FrameBlendMode::Linear(x) => {
                let a = x.alpha;
                (
                    pr_lin * (1.0 - a) + cr_lin * a,
                    pg_lin * (1.0 - a) + cg_lin * a,
                    pb_lin * (1.0 - a) + cb_lin * a,
                )
            }

            FrameBlendMode::Exp(x) => {
                let fade = x.fade;
                (
                    pr_lin * fade + cr_lin * (1.0 - fade),
                    pg_lin * fade + cg_lin * (1.0 - fade),
                    pb_lin * fade + cb_lin * (1.0 - fade),
                )
            }
            FrameBlendMode::Additive(x) => {
                let mut fr = pr_lin * x.fade + cr_lin * x.alpha;
                let mut fg = pg_lin * x.fade + cg_lin * x.alpha;
                let mut fb = pb_lin * x.fade + cb_lin * x.alpha;

                // Clamp to prevent overexposure
                fr = fr.min(1.0);
                fg = fg.min(1.0);
                fb = fb.min(1.0);

                (fr, fg, fb)
            }

            FrameBlendMode::Gamma(x) => {
                let gamma = 2.2;
                let fade = x.fade;

                fn to_linear(v: f32, g: f32) -> f32 {
                    v.powf(g)
                }
                fn to_srgb(v: f32, g: f32) -> f32 {
                    v.powf(1.0 / g)
                }

                let pr_l = to_linear(pr_lin, gamma);
                let pg_l = to_linear(pg_lin, gamma);
                let pb_l = to_linear(pb_lin, gamma);

                let cr_l = to_linear(cr_lin, gamma);
                let cg_l = to_linear(cg_lin, gamma);
                let cb_l = to_linear(cb_lin, gamma);

                (
                    to_srgb(pr_l * fade + cr_l * (1.0 - fade), gamma),
                    to_srgb(pg_l * fade + cg_l * (1.0 - fade), gamma),
                    to_srgb(pb_l * fade + cb_l * (1.0 - fade), gamma),
                )
            }

            FrameBlendMode::Accurate(x) => {
                // Scanline timing & jitter
                let y = i / w;
                let jitter = ((y as f32).sin() * 0.003) + 1.0;
                let scan_delay = (1.0 - (y as f32 / h as f32) * 0.2) * jitter;

                fn lcd_step(prev: f32, curr: f32, rise: f32, fall: f32, delay: f32) -> f32 {
                    let rate = if curr > prev { rise } else { fall };
                    prev + (curr - prev) * rate * delay
                }

                // LCD response curve
                let mut lr = lcd_step(pr_lin, cr_lin, x.rise, x.fall, scan_delay);
                let mut lg = lcd_step(pg_lin, cg_lin, x.rise, x.fall, scan_delay);
                let mut lb = lcd_step(pb_lin, cb_lin, x.rise, x.fall, scan_delay);

                // Pixel bleeding (left + top)
                let left_idx = if i % w == 0 { i } else { i - 1 };
                let top_idx = if y == 0 { i } else { i - w };

                let left = self.prev_framebuffer[left_idx];
                let top = self.prev_framebuffer[top_idx];

                let lpr = ((left >> 16) & 0xFF) as f32 / 255.0;
                let lpg = ((left >> 8) & 0xFF) as f32 / 255.0;
                let lpb = (left & 0xFF) as f32 / 255.0;

                let tpr = ((top >> 16) & 0xFF) as f32 / 255.0;
                let tpg = ((top >> 8) & 0xFF) as f32 / 255.0;
                let tpb = (top & 0xFF) as f32 / 255.0;

                lr = lr * (1.0 - x.bleed) + ((lpr + tpr) * 0.5) * x.bleed;
                lg = lg * (1.0 - x.bleed) + ((lpg + tpg) * 0.5) * x.bleed;
                lb = lb * (1.0 - x.bleed) + ((lpb + tpb) * 0.5) * x.bleed;

                // Tint
                lr *= x.tint.red;
                lg *= x.tint.green;
                lb *= x.tint.blue;

                (lr, lg, lb)
            }
        };

        let dim = config.dim;
        lr *= dim;
        lg *= dim;
        lb *= dim;

        // Convert back to 0..255
        (
            (lr * 255.0).clamp(0.0, 255.0) as u32,
            (lg * 255.0).clamp(0.0, 255.0) as u32,
            (lb * 255.0).clamp(0.0, 255.0) as u32,
        )
    }
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
            FrameBlendMode::Exp(_) => "Exp",
            FrameBlendMode::Gamma(_) => "Gamma",
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
            FrameBlendMode::Exp(_) => {}
            FrameBlendMode::Gamma(x) => {
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
            FrameBlendMode::Exp(x) => {
                x.fade = core::change_f32_rounded(x.fade, v).clamp(0.0, 1.0)
            }
            FrameBlendMode::Gamma(x) => {
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
            FrameBlendMode::Exp(_) => 0.0,
            FrameBlendMode::Gamma(x) => x.alpha,
            FrameBlendMode::Accurate(_) => 0.0,
        }
    }

    pub fn get_fade(&self) -> f32 {
        match self {
            FrameBlendMode::None => 0.0,
            FrameBlendMode::Linear(_) => 0.0,
            FrameBlendMode::Additive(x) => x.fade,
            FrameBlendMode::Exp(x) => x.fade,
            FrameBlendMode::Gamma(x) => x.fade,
            FrameBlendMode::Accurate(_) => 0.0,
        }
    }

    pub fn get_profile(&self) -> Option<&BlendProfile> {
        match self {
            FrameBlendMode::None
            | FrameBlendMode::Linear(_)
            | FrameBlendMode::Additive(_)
            | FrameBlendMode::Exp(_)
            | FrameBlendMode::Gamma(_) => None,
            FrameBlendMode::Accurate(x) => Some(x),
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

    pub fn reset(&mut self) {
        self.red = 1.0;
        self.green = 1.0;
        self.blue = 1.0;
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
