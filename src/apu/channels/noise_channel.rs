use crate::apu::registers::{NRx2, NRx4};
use crate::channels::channel::ChannelType;
use crate::dac::{DacEnable, DigitalSampleProducer};
use crate::registers::NRx1;
use crate::timers::envelope_timer::EnvelopeTimer;
use crate::timers::length_timer::LengthTimer;
use crate::{get_bit_flag, NR52};

pub const CH4_START_ADDRESS: u16 = NR41_CH4_LENGTH_TIMER_ADDRESS;
pub const CH4_END_ADDRESS: u16 = NR44_CH4_CONTROL_ADDRESS;

pub const NR41_CH4_LENGTH_TIMER_ADDRESS: u16 = 0xFF20;
pub const NR42_CH4_VOLUME_ENVELOPE_ADDRESS: u16 = 0xFF21;
pub const NR43_CH4_FREQUENCY_RANDOMNESS_ADDRESS: u16 = 0xFF22;
pub const NR44_CH4_CONTROL_ADDRESS: u16 = 0xFF23;

const DIVISORS: [u16; 8] = [8, 16, 32, 48, 64, 80, 96, 112];

#[derive(Debug, Clone)]
pub struct NoiseChannel {
    nrx1_len: NRx1,
    nrx2_envelope_and_dac: NRx2,
    nr43_freq_and_rnd: NR43,
    nrx4_ctrl: NRx4,

    length_timer: LengthTimer,
    envelope_timer: EnvelopeTimer,
    freq_timer: u16,
    /// linear feedback shift register:
    /// 15 bits for its current state and 1 bit to temporarily store the next bit to shift in
    lfsr: u16,
}

impl Default for NoiseChannel {
    fn default() -> NoiseChannel {
        let ch_type = ChannelType::CH4;

        Self {
            nrx1_len: NRx1::new(ch_type),
            nrx2_envelope_and_dac: Default::default(),
            nr43_freq_and_rnd: Default::default(),
            nrx4_ctrl: Default::default(),
            length_timer: LengthTimer::new(ch_type),
            envelope_timer: Default::default(),
            freq_timer: 0,
            lfsr: 0x7FFF,
        }
    }
}

impl DacEnable for NoiseChannel {
    fn is_dac_enabled(&self) -> bool {
        self.nrx2_envelope_and_dac.is_dac_enabled()
    }
}

impl DigitalSampleProducer for NoiseChannel {
    fn get_sample(&self, master_ctrl: NR52) -> u8 {
        // If the bit shifted out is a 0, the channel emits a 0; otherwise, it emits the volume selected in NR42
        if master_ctrl.is_ch4_on() && (self.lfsr & 0b01) == 0 {
            return self.envelope_timer.get_volume();
        }

        0
    }
}

impl NoiseChannel {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            NR41_CH4_LENGTH_TIMER_ADDRESS => 0xFF,
            NR42_CH4_VOLUME_ENVELOPE_ADDRESS => self.nrx2_envelope_and_dac.byte,
            NR43_CH4_FREQUENCY_RANDOMNESS_ADDRESS => self.nr43_freq_and_rnd.byte,
            NR44_CH4_CONTROL_ADDRESS => self.nrx4_ctrl.read(),
            _ => panic!("Invalid NoiseChannel address: {:#X}", addr),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8, master_ctrl: &mut NR52) {
        match addr {
            NR41_CH4_LENGTH_TIMER_ADDRESS => self.nrx1_len.byte = value,
            NR42_CH4_VOLUME_ENVELOPE_ADDRESS => self.nrx2_envelope_and_dac.byte = value,
            NR43_CH4_FREQUENCY_RANDOMNESS_ADDRESS => self.nr43_freq_and_rnd.byte = value,
            NR44_CH4_CONTROL_ADDRESS => {
                self.nrx4_ctrl.write(value);

                if self.nrx4_ctrl.is_triggered() {
                    self.trigger(master_ctrl);
                }
            }
            _ => panic!("Invalid NoiseChannel address: {:#X}", addr),
        }
    }

    pub fn tick(&mut self) {
        if self.freq_timer > 0 {
            self.freq_timer -= 1;
        }

        // If the frequency timer decrement to 0, it is reloaded with the formula
        // `divisor_code << clock_shift` and wave position is advanced by one.
        if self.freq_timer == 0 {            
            self.reload_freq_timer();

            // The XOR result of the 0th and 1st bit of LFSR is computed
            let xor_result = (self.lfsr & 0b01) ^ ((self.lfsr & 0b10) >> 1);
            self.lfsr = (self.lfsr >> 1) | (xor_result << 14);

            // If the width mode bit is set, the XOR result is also stored in bit 6.
            if self.nr43_freq_and_rnd.lfsr_width() {
                self.lfsr &= !(1 << 6);
                self.lfsr |= xor_result << 6;
            }
        }
    }

    pub fn tick_length(&mut self, master_ctrl: &mut NR52) {
        self.length_timer.tick(master_ctrl, &mut self.nrx4_ctrl);
    }

    pub fn tick_envelope(&mut self) {
        self.envelope_timer.tick(self.nrx2_envelope_and_dac);
    }

    fn trigger(&mut self, nr52: &mut NR52) {
        nr52.activate_ch4();

        if self.length_timer.is_expired() {
            self.length_timer.reload(&self.nrx1_len);
        }

        self.reload_freq_timer();
        self.envelope_timer.reload(self.nrx2_envelope_and_dac);
        self.lfsr = 0x7FFF;
    }
    
    fn reload_freq_timer(&mut self) {
        // Reload the frequency timer with the correct divisor
        let divisor = DIVISORS[self.nr43_freq_and_rnd.clock_divider() as usize];
        self.freq_timer = divisor << self.nr43_freq_and_rnd.clock_shift();
    }
}

/// FF22 — NR43: Channel 4 frequency & randomness
/// This register allows controlling the way the amplitude is randomly switched.
#[derive(Debug, Clone, Default)]
pub struct NR43 {
    byte: u8,
}

impl NR43 {
    pub fn clock_shift(&self) -> u8 {
        self.byte & 0b1111_0000
    }

    pub fn get_lfsr_width(&self) -> LfsrWidth {
        if get_bit_flag(self.byte, 3) {
            LfsrWidth::Bit7
        } else {
            LfsrWidth::Bit15
        }
    }

    pub fn lfsr_width(&self) -> bool {
        get_bit_flag(self.byte, 3)
    }

    pub fn clock_divider(&self) -> u8 {
        self.byte & 0b0000_0111
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum LfsrWidth {
    Bit15 = 0,
    Bit7 = 1,
}
