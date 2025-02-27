use crate::apu::NR52;

pub trait DacEnable {
    fn is_dac_enabled(&self) -> bool;
}

pub trait DigitalOutputProducer {
    fn get_output(&self, nr52: NR52) -> u8;
}

/// If a DAC is enabled, the digital range $0 to $F is linearly translated to the analog range -1 to 1, in arbitrary units. Importantly, the slope is negative: “digital 0” maps to “analog 1”, not “analog -1”.
/// If a DAC is disabled, it fades to an analog value of 0, which corresponds to “digital 7.5”. The nature of this fade is not entirely deterministic and varies between models.
pub fn apply_dac<T: DacEnable + DigitalOutputProducer>(nr52: NR52, producer: &T) -> f32 {
    if !producer.is_dac_enabled() {
        return 0.0;
    }

    let output = producer.get_output(nr52);

    if output == 0 {
        return 1.0;
    }

    (output as f32 / 7.5) - 1.0
}
