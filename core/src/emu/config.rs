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
