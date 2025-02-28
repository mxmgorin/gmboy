/// A high-pass filter (HPF) removes constant biases over time. The HPFs therefore remove the DC
/// offset created by inactive channels with an enabled DAC, and off-center waveforms.
#[derive(Debug, Clone)]
pub struct Hpf {
    capacitor: f32,
    charge_factor: f32,

    pub dac1_enabled: bool,
    pub dac2_enabled: bool,
    pub dac3_enabled: bool,
    pub dac4_enabled: bool,
}

impl Hpf {
    pub fn new(sampling_freq: i32) -> Hpf {
        Self {
            capacitor: 0.0,
            charge_factor: calc_charge_factor(sampling_freq),
            dac1_enabled: false,
            dac2_enabled: false,
            dac3_enabled: false,
            dac4_enabled: false,
        }
    }
}

impl Hpf {
    /// When all four channel DACs are off, the master volume units are disconnected from the sound
    /// output and the output level becomes 0. When any channel DAC is on, a high-pass filter
    /// capacitor is connected which slowly removes any DC component from the signal
    pub fn apply_filter(&mut self, input: f32) -> f32 {
        let mut out = 0.0;

        if self.dac1_enabled || self.dac2_enabled || self.dac3_enabled || self.dac4_enabled {
            out = input - self.capacitor;

            // capacitor slowly charges to 'in' via their difference
            self.capacitor = input - out * self.charge_factor; // 0.999958 and 0.998943 for MGB&CGB
        }

        out
    }
}

fn calc_charge_factor(sampling_freq: i32) -> f32 {
    0.999958_f32.powf(4194304.0 / sampling_freq as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charge_factor() {
        let output_rate = 44100;
        let expected_factor = 0.996;
        let calculated_factor = calc_charge_factor(output_rate);
        assert!(
            (calculated_factor - expected_factor).abs() < 0.001,
            "Expected {}, got {}",
            expected_factor,
            calculated_factor
        );
    }
}
