use crate::cpu::CPU_CLOCK_SPEED;
use crate::emu::config::GbModel;
use serde::{Deserialize, Serialize};

/// A high-pass filter (HPF) removes constant biases over time. The HPFs therefore remove the DC
/// offset created by inactive channels with an enabled DAC, and off-center waveforms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hpf {
    left_capacitor: f32,
    right_capacitor: f32,
    charge_factor: f32,

    pub dac1_enabled: bool,
    pub dac2_enabled: bool,
    pub dac3_enabled: bool,
    pub dac4_enabled: bool,
}

impl Hpf {
    pub fn new(sampling_freq: u32) -> Hpf {
        Self {
            left_capacitor: 0.0,
            right_capacitor: 0.0,
            charge_factor: calc_charge_factor(sampling_freq, GbModel::default()),
            dac1_enabled: false,
            dac2_enabled: false,
            dac3_enabled: false,
            dac4_enabled: false,
        }
    }

    /// The capacitor leak differs between models; the bus applies the loaded
    /// model here when it resolves it (construction happens earlier).
    pub fn set_model(&mut self, sampling_freq: u32, model: GbModel) {
        self.charge_factor = calc_charge_factor(sampling_freq, model);
    }
}

impl Hpf {
    /// When all four channel DACs are off, the master volume units are disconnected from the sound
    /// output and the output level becomes 0. When any channel DAC is on, a high-pass filter
    /// capacitor is connected which slowly removes any DC component from the signal
    #[inline(always)]
    pub fn apply_filter(&mut self, left_input: f32, right_input: f32) -> (f32, f32) {
        if self.dac1_enabled || self.dac2_enabled || self.dac3_enabled || self.dac4_enabled {
            let left_out = left_input - self.left_capacitor;
            let right_out = right_input - self.right_capacitor;

            // capacitor slowly charges to 'in' via their difference
            self.left_capacitor = left_input - left_out * self.charge_factor;
            self.right_capacitor = right_input - right_out * self.charge_factor;

            return (left_out, right_out);
        }

        (0.0, 0.0)
    }
}

/// Per-4MHz-tick capacitor leak, measured by SameBoy: 0.999958 on DMG,
/// 0.998943 on MGB/CGB. Raised to the ticks-per-sample power to apply at the
/// output rate.
#[inline(always)]
fn calc_charge_factor(sampling_freq: u32, model: GbModel) -> f32 {
    let leak: f32 = match model {
        GbModel::Dmg => 0.999958,
        GbModel::Cgb => 0.998943,
    };

    leak.powf(CPU_CLOCK_SPEED as f32 / sampling_freq as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charge_factor() {
        let output_rate = 44100;

        let dmg = calc_charge_factor(output_rate, GbModel::Dmg);
        assert!((dmg - 0.996).abs() < 0.001, "DMG: got {dmg}");

        let cgb = calc_charge_factor(output_rate, GbModel::Cgb);
        assert!((cgb - 0.904).abs() < 0.001, "CGB: got {cgb}");
    }
}
