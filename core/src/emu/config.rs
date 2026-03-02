use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum GbModel {
    Auto,
    Dmg,
    Cgb,
}

impl Default for GbModel {
    fn default() -> Self {
        GbModel::Auto
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmuConfig {
    pub rewind_size: usize,
    pub rewind_frames: usize,
    pub normal_speed: f64,
    pub slow_speed: f64,
    pub turbo_speed: f64,
    pub spin_duration: Duration,
    pub model: GbModel,
}

impl Default for EmuConfig {
    fn default() -> Self {
        Self {
            rewind_size: 120,
            rewind_frames: 60,
            normal_speed: 1.0,
            slow_speed: 0.5,
            turbo_speed: 5.0,
            spin_duration: Duration::from_millis(1),
            model: GbModel::Dmg,
        }
    }
}
