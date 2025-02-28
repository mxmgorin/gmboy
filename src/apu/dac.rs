use crate::apu::NR52;

pub trait DacEnable {
    fn is_dac_enabled(&self) -> bool;
}

pub trait DigitalSampleProducer {
    fn get_sample(&self, nr52: NR52) -> u8;
}

/// If a DAC is enabled, the digital range $0 to $F is linearly translated to the analog range -1 to 1, in arbitrary units. Importantly, the slope is negative: “digital 0” maps to “analog 1”, not “analog -1”.
/// If a DAC is disabled, it fades to an analog value of 0, which corresponds to “digital 7.5”. The nature of this fade is not entirely deterministic and varies between models.
pub fn apply_dac<T: DacEnable + DigitalSampleProducer>(nr52: NR52, producer: &T) -> (bool, f32) {
    let dac_enabled = producer.is_dac_enabled();

    if dac_enabled {
        let sample = producer.get_sample(nr52);

        if sample == 0 {
            return (dac_enabled, 1.0);
        }

        let sample = (sample as f32 / 7.5) - 1.0;

        return (dac_enabled, sample);
    }

    (false, 0.0)
}
