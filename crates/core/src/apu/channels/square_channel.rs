use crate::apu::channels::channel::ChannelType;
use crate::apu::dac::{DacEnable, DigitalSampleProducer};
use crate::apu::registers::{NRx1, NRx2, NRx3x4};
use crate::apu::timers::envelope_timer::EnvelopeTimer;
use crate::apu::timers::length_timer::LengthTimer;
use crate::apu::timers::period_timer::PeriodTimer;
use crate::apu::timers::sweep_timer::SweepTimer;
use crate::apu::NR52;
use crate::get_bit_flag;
use serde::{Deserialize, Serialize};

pub const CH1_START_ADDRESS: u16 = NR10_CH1_SWEEP_ADDRESS;
pub const CH1_END_ADDRESS: u16 = NR14_CH1_PERIOD_HIGH_CONTROL_ADDRESS;

pub const CH2_START_ADDRESS: u16 = NR20_CH2_PERIOD_HIGH_CONTROL_ADDRESS;
pub const CH2_END_ADDRESS: u16 = NR24_CH2_PERIOD_HIGH_CONTROL_ADDRESS;

pub const NR10_CH1_SWEEP_ADDRESS: u16 = 0xFF10;
const NR10_CH1_UNUSED_MASK: u8 = 0b1000_0000;
pub const NR11_CH1_LEN_TIMER_DUTY_CYCLE_ADDRESS: u16 = 0xFF11;
pub const NR12_CH1_VOL_ENVELOPE_ADDRESS: u16 = 0xFF12;
pub const NR13_CH1_PERIOD_LOW_ADDRESS: u16 = 0xFF13;
pub const NR14_CH1_PERIOD_HIGH_CONTROL_ADDRESS: u16 = 0xFF14;

/// There is no NR20 register.
pub const NR20_CH2_PERIOD_HIGH_CONTROL_ADDRESS: u16 = 0xFF15;
pub const NR21_CH2_LEN_TIMER_DUTY_CYCLE_ADDRESS: u16 = 0xFF16;
pub const NR22_CH2_VOL_ENVELOPE_ADDRESS: u16 = 0xFF17;
pub const NR23_CH2_PERIOD_LOW_ADDRESS: u16 = 0xFF18;
pub const NR24_CH2_PERIOD_HIGH_CONTROL_ADDRESS: u16 = 0xFF19;

pub const WAVE_DUTY_PATTERNS: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 0, 1, 1, 1],
    [0, 1, 1, 1, 1, 1, 1, 0],
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SquareChannel {
    // registers
    sweep_timer: Option<SweepTimer>,
    nrx1_len_timer_duty_cycle: NRx1,
    nrx2_volume_envelope_and_dac: NRx2,
    nrx3x4_period_and_ctrl: NRx3x4,

    // other data
    ch_type: ChannelType,
    period_timer: PeriodTimer,
    length_timer: LengthTimer,
    envelope_timer: EnvelopeTimer,
    pub duty_sequence: usize,
    /// A freshly (re)triggered channel outputs digital 0 until the first
    /// duty step after the trigger delay elapses.
    #[serde(default)]
    suppressed: bool,
    /// Output latched at the last duty step: an NR x1 duty change becomes
    /// effective only when the current sample finishes
    /// (same-suite channel_x_duty_delay).
    #[serde(default)]
    current_sample: u8,
}

impl DacEnable for SquareChannel {
    fn is_dac_enabled(&self) -> bool {
        self.nrx2_volume_envelope_and_dac.is_dac_enabled()
    }
}

impl DigitalSampleProducer for SquareChannel {
    fn get_sample(&self, master_ctrl: NR52) -> u8 {
        if !master_ctrl.is_ch_on(self.ch_type) || self.suppressed {
            return 0;
        }

        self.current_sample * self.envelope_timer.get_volume()
    }
}

impl SquareChannel {
    pub fn new_ch1() -> SquareChannel {
        Self::new(ChannelType::CH1)
    }

    pub fn new_ch2() -> SquareChannel {
        Self::new(ChannelType::CH2)
    }

    fn new(ch_type: ChannelType) -> Self {
        Self {
            sweep_timer: if ch_type == ChannelType::CH1 {
                Some(Default::default())
            } else {
                None
            },
            nrx1_len_timer_duty_cycle: NRx1::new(ch_type),
            nrx2_volume_envelope_and_dac: Default::default(),
            nrx3x4_period_and_ctrl: Default::default(),
            ch_type,
            length_timer: LengthTimer::new(ch_type),
            period_timer: PeriodTimer::new(ch_type),
            duty_sequence: 0,
            envelope_timer: Default::default(),
            suppressed: false,
            current_sample: 0,
        }
    }

    #[inline]
    pub fn read(&self, address: u16) -> u8 {
        let offset = self.get_offset(address);

        // Write-only bits read back as 1: NRx0 bit 7, NRx1 length bits,
        // NRx3 entirely, NRx4 everything but the length-enable bit.
        match offset {
            0 => {
                if let Some(sweep_timer) = &self.sweep_timer {
                    sweep_timer.nr10.byte | NR10_CH1_UNUSED_MASK
                } else {
                    0xFF
                }
            }
            1 => self.nrx1_len_timer_duty_cycle.byte | 0x3F,
            2 => self.nrx2_volume_envelope_and_dac.byte,
            3 => 0xFF,
            4 => self.nrx3x4_period_and_ctrl.nrx4.read() | 0xBF,
            _ => panic!("Invalid Square address: {address:#X}"),
        }
    }

    #[inline]
    pub fn write(
        &mut self,
        address: u16,
        value: u8,
        master_ctrl: &mut NR52,
        len_first_half: bool,
        trigger_delay: u16,
    ) {
        let offset = self.get_offset(address);

        match offset {
            0 => {
                if let Some(sweep_timer) = self.sweep_timer.as_mut() {
                    sweep_timer.nr10.byte = value;
                }
            }
            1 => {
                self.nrx1_len_timer_duty_cycle.byte = value;
                self.length_timer.reload(self.nrx1_len_timer_duty_cycle); // research: do it must be reloaded after write?
            }
            // Writes to this register while the channel is on require re-triggering it after wards.
            // If the write turns the channel off, re-triggering is not necessary (it would do nothing).
            2 => {
                self.nrx2_volume_envelope_and_dac.byte = value;
                master_ctrl.on_dac_update(
                    self.nrx2_volume_envelope_and_dac.is_dac_enabled(),
                    self.ch_type,
                );
            }
            3 => self.nrx3x4_period_and_ctrl.period_low.write(value),
            4 => {
                let was_len_enabled = self.nrx3x4_period_and_ctrl.nrx4.is_length_enabled();
                self.nrx3x4_period_and_ctrl.nrx4.write(value);
                let nrx4 = self.nrx3x4_period_and_ctrl.nrx4;

                if len_first_half && !was_len_enabled && nrx4.is_length_enabled() {
                    self.length_timer
                        .extra_clock(master_ctrl, nrx4.is_triggered());
                }

                if nrx4.is_triggered() {
                    self.trigger(master_ctrl, len_first_half, trigger_delay);
                }
            }
            _ => panic!("Invalid Square address: {:#X}", address),
        }
    }

    #[inline]
    fn get_offset(&self, address: u16) -> u16 {
        address - self.ch_type.get_start_address()
    }

    #[inline]
    pub fn tick_envelope(&mut self) {
        self.envelope_timer.tick(self.nrx2_volume_envelope_and_dac);
    }

    #[inline]
    pub fn tick_sweep(&mut self, nr52: &mut NR52) {
        if let Some(sweep) = self.sweep_timer.as_mut() {
            sweep.tick(nr52, &mut self.nrx3x4_period_and_ctrl);
        }
    }

    #[inline]
    pub fn tick_length(&mut self, master_ctrl: &mut NR52) {
        self.length_timer
            .tick(master_ctrl, &mut self.nrx3x4_period_and_ctrl.nrx4);
    }

    #[inline]
    pub fn tick(&mut self) {
        if self.period_timer.tick(&self.nrx3x4_period_and_ctrl) {
            self.duty_sequence = (self.duty_sequence + 1) & 0x07;
            let duty = self.nrx1_len_timer_duty_cycle.get_duty_cycle_idx() as usize;
            self.current_sample = WAVE_DUTY_PATTERNS[duty][self.duty_sequence];
            self.suppressed = false;
        }
    }

    /// APU power-off: the duty position and the latched output restart.
    #[inline]
    pub fn reset_duty(&mut self) {
        self.duty_sequence = 0;
        self.current_sample = 0;
    }

    #[inline]
    fn trigger(&mut self, nr52: &mut NR52, len_first_half: bool, trigger_delay: u16) {
        let was_active = nr52.is_ch_on(self.ch_type);
        nr52.activate_ch(self.ch_type);

        if self.length_timer.is_expired() {
            let extra = len_first_half && self.nrx3x4_period_and_ctrl.nrx4.is_length_enabled();
            self.length_timer.reset(extra);
        }

        // Trigger-to-first-duty-step latency: an already-active channel
        // restarts one 2 MHz cycle (4 T) sooner than an inactive one.
        let delay = trigger_delay - if was_active { 4 } else { 0 };
        self.period_timer
            .reload_with_delay(&self.nrx3x4_period_and_ctrl, delay);

        // A channel activated from off holds digital 0 until its first duty
        // step; retriggering an active one keeps outputting the current step.
        if !was_active {
            self.suppressed = true;
        }

        self.envelope_timer
            .reload(self.nrx2_volume_envelope_and_dac);

        if let Some(sweep_timer) = self.sweep_timer.as_mut() {
            sweep_timer.reload(nr52, self.nrx3x4_period_and_ctrl);
        }
    }
}

/// FF10 — NR10: Channel 1 sweep
/// This register controls CH1’s period sweep functionality.
#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize)]
pub struct NR10 {
    pub byte: u8,
}

impl NR10 {
    /// This dictates how often sweep “iterations” happen, in units of 128 Hz ticks5 (7.8 ms). Note
    /// that the value written to this field is not re-read by the hardware until a sweep iteration
    /// completes, or the channel is (re)triggered.
    /// However, if 0 is written to this field, then iterations are instantly disabled (but see below),
    /// and it will be reloaded as soon as it’s set to something else.
    #[inline]
    pub fn pace(&self) -> u8 {
        self.byte & 0b0111_0000
    }

    /// 0 = Addition (period increases); 1 = Subtraction (period decreases)
    #[inline]
    pub fn direction_down(&self) -> bool {
        get_bit_flag(self.byte, 3)
    }

    #[inline]
    pub fn individual_step(&self) -> u8 {
        self.byte & 0b0000_0111
    }
}
