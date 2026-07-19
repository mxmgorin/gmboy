use crate::apu::channels::channel::ChannelType;
use crate::apu::channels::noise_channel::NR41_CH4_LENGTH_TIMER_ADDRESS;
use crate::apu::channels::noise_channel::{NoiseChannel, CH4_END_ADDRESS, CH4_START_ADDRESS};
use crate::apu::channels::square_channel::{
    SquareChannel, CH1_END_ADDRESS, CH1_START_ADDRESS, CH2_END_ADDRESS, CH2_START_ADDRESS,
};
use crate::apu::channels::square_channel::{
    NR11_CH1_LEN_TIMER_DUTY_CYCLE_ADDRESS, NR21_CH2_LEN_TIMER_DUTY_CYCLE_ADDRESS,
};
use crate::apu::channels::wave_channel::{
    WaveChannel, CH3_END_ADDRESS, CH3_START_ADDRESS, CH3_WAVE_RAM_END, CH3_WAVE_RAM_START,
};
use crate::apu::dac::{apply_dac, DigitalSampleProducer};
use crate::apu::hpf::Hpf;
use crate::apu::mixer::Mixer;
use crate::cpu::CPU_CLOCK_SPEED;
use crate::{change_f32_rounded, get_bit_flag, set_bit};
use serde::{Deserialize, Serialize};

pub const AUDIO_START_ADDRESS: u16 = 0xFF10;
pub const AUDIO_END_ADDRESS: u16 = 0xFF26;

pub const APU_CLOCK_SPEED: u16 = 512;
pub const AUDIO_MASTER_CONTROL_ADDRESS: u16 = 0xFF26;
pub const SOUND_PLANNING_ADDRESS: u16 = 0xFF25;
pub const MASTER_VOLUME_ADDRESS: u16 = 0xFF24;

pub const FRAME_SEQUENCER_DIV: u16 = (CPU_CLOCK_SPEED / APU_CLOCK_SPEED as u32) as u16;
pub const SAMPLING_FREQUENCY: u32 = 44_100; // native is 41_943
const TICKS_PER_SAMPLE: u32 = CPU_CLOCK_SPEED / SAMPLING_FREQUENCY;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApuConfig {
    pub buffer_size: usize,
    pub volume: f32,
}

impl ApuConfig {
    pub fn new(buffer_size: usize, volume: f32) -> Self {
        Self {
            buffer_size,
            volume,
        }
    }

    pub fn change_volume(&mut self, delta: f32) {
        let val = change_f32_rounded(self.volume, delta);
        self.volume = val.clamp(0.0, 2.0);
    }
}

impl Default for ApuConfig {
    fn default() -> Self {
        Self::new(512, 1.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Apu {
    // internal
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    nr52: NR52,
    mixer: Mixer,

    // other data
    /// Frame sequencer phase, incremented per DIV-APU event (SameBoy's
    /// div_divider): length clocks when it becomes odd, sweep when
    /// `& 3 == 3`, envelope when `& 7 == 0`. Its parity while idle is the
    /// length-period half that the NRx4 extra-clocking quirks key on.
    frame_sequencer_step: u8,
    /// Last sampled DIV-APU bit for falling-edge detection (the frame
    /// sequencer is clocked by DIV, not by an internal timer).
    #[serde(default)]
    prev_div_apu_bit: bool,
    /// Powering the APU on while the DIV-APU bit is set makes the first
    /// falling edge a leftover of the pre-power period: it is swallowed, and
    /// the one after it runs without advancing the phase
    /// (same-suite div_write_trigger_10).
    #[serde(default)]
    skip_div_event: SkipDivEvent,
    ticks_count: u32,
    buffer_idx: usize,
    buffer: Box<[f32]>,
    hpf: Hpf,
    pub config: ApuConfig,
}

impl Default for Apu {
    fn default() -> Self {
        Self::new(ApuConfig::default())
    }
}

impl Apu {
    pub fn new(mut config: ApuConfig) -> Self {
        if config.buffer_size % 2 != 0 {
            config.buffer_size += 1; // we need even buffer
        }

        Self {
            ch1: SquareChannel::new_ch1(),
            ch2: SquareChannel::new_ch2(),
            ch3: WaveChannel::default(),
            ch4: NoiseChannel::default(),
            nr52: NR52::default(),
            mixer: Default::default(),
            frame_sequencer_step: 0,
            prev_div_apu_bit: false,
            skip_div_event: SkipDivEvent::None,
            ticks_count: 0,
            buffer: vec![0.0; config.buffer_size].into_boxed_slice(),
            buffer_idx: 0,
            hpf: Hpf::new(SAMPLING_FREQUENCY),
            config,
        }
    }

    #[inline(always)]
    pub fn update_buffer_size(&mut self) {
        self.buffer = vec![0.0; self.config.buffer_size].into_boxed_slice();
        self.clear_buffer();
    }

    #[inline(always)]
    pub fn tick(&mut self, div_apu_bit: bool) {
        self.ticks_count = self.ticks_count.wrapping_add(1);
        self.sequence_frame(div_apu_bit);

        self.ch1.tick();
        self.ch2.tick();
        self.ch3.tick();
        self.ch4.tick();

        // down sample by nearest-neighbor
        if self.ticks_count % TICKS_PER_SAMPLE == 0 {
            (self.hpf.dac1_enabled, self.mixer.sample1) = apply_dac(self.nr52, &self.ch1);
            (self.hpf.dac2_enabled, self.mixer.sample2) = apply_dac(self.nr52, &self.ch2);
            (self.hpf.dac3_enabled, self.mixer.sample3) = apply_dac(self.nr52, &self.ch3);
            (self.hpf.dac4_enabled, self.mixer.sample4) = apply_dac(self.nr52, &self.ch4);
            let (output_left, output_right) = self.mixer.mix();

            let (output_left, output_right) = self.hpf.apply_filter(output_left, output_right);
            self.push_buffer(output_left, output_right);
        }
    }

    #[inline(always)]
    pub fn push_buffer(&mut self, output_left: f32, output_right: f32) {
        let buffer_len = self.buffer.len();
        debug_assert!(buffer_len % 2 == 0);

        // SAFETY:
        // - `buffer` is aligned to `f32`, which guarantees at least 4-byte alignment.
        // - we create the buffer using `into_boxed_slice` (or `Vec<f32>`), the allocator
        //   ensures the alignment is at least 8 bytes, which is sufficient for `u64`.
        // - `buffer.len()` is guaranteed to be even (checked at creation), so dividing by 2
        //   to reinterpret as `u64` slices is safe and valid.
        let buffer_u64 = unsafe {
            std::slice::from_raw_parts_mut(self.buffer.as_mut_ptr() as *mut u64, buffer_len / 2)
        };

        // SAFETY:
        // - `[f32; 2]` and `u64` have the same size (8 bytes).
        // - `transmute` here performs a bitwise reinterpretation of the two floats as a single u64,
        //   preserving their exact bit pattern without changing the data.
        // - All bit patterns are valid for both `f32` and `u64`, so this is safe and defined behavior.
        // - Endianness affects byte order but does not affect safety or correctness for raw packing.
        let packed = unsafe {
            std::mem::transmute::<[f32; 2], u64>([
                output_left * self.config.volume,
                output_right * self.config.volume,
            ])
        };

        if self.buffer_idx >= buffer_len {
            self.clear_buffer();
        }

        // SAFETY:
        // - `self.buffer_idx` is always even and within bounds of `buffer_u64`.
        // - this is the only place where `buffer_idx` is written to.
        unsafe {
            *buffer_u64.get_unchecked_mut(self.buffer_idx / 2) = packed;
        }

        self.buffer_idx += 2;
    }

    #[inline(always)]
    pub fn get_buffer(&self) -> &[f32] {
        &self.buffer[0..self.buffer_idx]
    }

    #[inline(always)]
    pub fn clear_buffer(&mut self) {
        self.buffer_idx = 0;
    }

    #[inline(always)]
    pub fn buffer_ready(&self) -> bool {
        self.buffer_idx >= self.config.buffer_size / 2
    }

    #[inline(always)]
    pub fn write(&mut self, address: u16, value: u8) {
        if (CH3_WAVE_RAM_START..=CH3_WAVE_RAM_END).contains(&address) {
            self.ch3.wave_ram.write(address, value);
            return;
        }

        if !self.nr52.is_audio_on() && address != AUDIO_MASTER_CONTROL_ADDRESS {
            //return; //todo: research Asteroids tries to write to panning before enabling
        }

        // the length timers (in NRx1) on monochrome models also writable event when turned off
        let value = if !self.nr52.is_audio_on()
            && [
                NR11_CH1_LEN_TIMER_DUTY_CYCLE_ADDRESS,
                NR21_CH2_LEN_TIMER_DUTY_CYCLE_ADDRESS,
                NR41_CH4_LENGTH_TIMER_ADDRESS,
            ]
            .contains(&address)
        {
            value & 0b0011_1111
        } else {
            value
        };

        // First half of a length period: the next sequencer step is one that
        // does NOT clock length (length clocks on even steps). NRx4 writes in
        // this phase get the extra length clocking quirks.
        let len_first_half = self.frame_sequencer_step & 1 == 1;

        match address {
            CH1_START_ADDRESS..=CH1_END_ADDRESS => {
                self.ch1.write(address, value, &mut self.nr52, len_first_half)
            }
            CH2_START_ADDRESS..=CH2_END_ADDRESS => {
                self.ch2.write(address, value, &mut self.nr52, len_first_half)
            }
            CH3_START_ADDRESS..=CH3_END_ADDRESS => {
                self.ch3.write(address, value, &mut self.nr52, len_first_half)
            }
            CH4_START_ADDRESS..=CH4_END_ADDRESS => {
                self.ch4.write(address, value, &mut self.nr52, len_first_half)
            }
            AUDIO_MASTER_CONTROL_ADDRESS => {
                let prev_enable = self.nr52.is_audio_on();
                self.nr52.write(value);

                if !prev_enable && self.nr52.is_audio_on() {
                    // turning on
                    self.ch3.wave_ram.clear_sample_buffer();

                    // Power-on while the DIV-APU bit is set: the leftover
                    // falling edge is swallowed and the sequencer starts in
                    // the first half of a length period (phase 1).
                    if self.prev_div_apu_bit {
                        self.skip_div_event = SkipDivEvent::Skip;
                        self.frame_sequencer_step = 1;
                    }
                } else if prev_enable && !self.nr52.is_audio_on() {
                    // turning_off
                    for addr in CH1_START_ADDRESS..=0xFF25 {
                        self.write(addr, 0x00);
                    }

                    self.frame_sequencer_step = 0;
                    self.skip_div_event = SkipDivEvent::None;
                    self.ch1.duty_sequence = 0;
                    self.ch2.duty_sequence = 0;
                    self.ch3.wave_ram.reset_sample_index();
                }
            }
            SOUND_PLANNING_ADDRESS => self.mixer.nr51_panning.byte = value,
            MASTER_VOLUME_ADDRESS => self.mixer.nr50_volume.byte = value,
            CH3_WAVE_RAM_START..=CH3_WAVE_RAM_END => self.ch3.wave_ram.write(address, value),
            _ => {
                if (AUDIO_START_ADDRESS..=AUDIO_END_ADDRESS).contains(&address) {
                    return;
                }

                panic!("Invalid APU address: {address:x}");
            }
        }
    }

    /// Register state the boot ROM leaves behind: master power on with the
    /// ch1 status flag set, boot-beep duty/envelope in ch1, default panning
    /// and volume (mooneye boot_hwio).
    pub fn set_boot_state(&mut self) {
        self.nr52.byte = 0x81;
        self.mixer.nr50_volume.byte = 0x77;
        self.mixer.nr51_panning.byte = 0xF3;
        self.write(0xFF11, 0x80); // NR11: duty 2 (the boot beep), length 0
        self.write(0xFF12, 0xF3); // NR12: initial volume 15, decreasing, pace 3
    }

    /// PCM12 ($FF76, CGB): current digital output of channels 1 (low nibble)
    /// and 2 (high nibble).
    #[inline(always)]
    pub fn read_pcm12(&self) -> u8 {
        self.ch1.get_sample(self.nr52) | (self.ch2.get_sample(self.nr52) << 4)
    }

    /// PCM34 ($FF77, CGB): current digital output of channels 3 (low nibble)
    /// and 4 (high nibble).
    #[inline(always)]
    pub fn read_pcm34(&self) -> u8 {
        self.ch3.get_sample(self.nr52) | (self.ch4.get_sample(self.nr52) << 4)
    }

    #[inline(always)]
    pub fn read(&self, address: u16) -> u8 {
        match address {
            CH1_START_ADDRESS..=CH1_END_ADDRESS => self.ch1.read(address),
            CH2_START_ADDRESS..=CH2_END_ADDRESS => self.ch2.read(address),
            CH3_START_ADDRESS..=CH3_END_ADDRESS => self.ch3.read(address),
            CH4_START_ADDRESS..=CH4_END_ADDRESS => self.ch4.read(address),
            AUDIO_MASTER_CONTROL_ADDRESS => self.nr52.read(),
            SOUND_PLANNING_ADDRESS => self.mixer.nr51_panning.byte,
            MASTER_VOLUME_ADDRESS => self.mixer.nr50_volume.byte,
            CH3_WAVE_RAM_START..=CH3_WAVE_RAM_END => self.ch3.wave_ram.read(address),
            _ => {
                if (AUDIO_START_ADDRESS..=AUDIO_END_ADDRESS).contains(&address) {
                    return 0xFF;
                }

                panic!("Invalid APU address: {address:x}");
            }
        }
    }

    /// The frame sequencer generates low frequency clocks for the modulation
    /// units. It is clocked at 512 Hz by the falling edge of the DIV-APU bit
    /// (DIV bit 4, bit 5 in double speed) — so a DIV write that resets the
    /// counter while the bit is set produces an extra, early step
    /// (same-suite div_write_trigger*).
    #[inline(always)]
    fn sequence_frame(&mut self, div_apu_bit: bool) {
        let falling_edge = self.prev_div_apu_bit && !div_apu_bit;
        self.prev_div_apu_bit = div_apu_bit;

        if !falling_edge || !self.nr52.is_audio_on() {
            return;
        }

        match self.skip_div_event {
            // The first edge after power-on with the DIV-APU bit set belongs
            // to the pre-power period and is swallowed entirely.
            SkipDivEvent::Skip => {
                self.skip_div_event = SkipDivEvent::Skipped;
                return;
            }
            // The next one runs, but the phase does not advance.
            SkipDivEvent::Skipped => self.skip_div_event = SkipDivEvent::None,
            SkipDivEvent::None => {
                self.frame_sequencer_step = self.frame_sequencer_step.wrapping_add(1) & 7;
            }
        }

        if self.frame_sequencer_step & 1 == 1 {
            tick_length_all(self);
        }

        if self.frame_sequencer_step & 3 == 3 {
            self.ch1.tick_sweep(&mut self.nr52);
        }

        if self.frame_sequencer_step & 7 == 0 {
            tick_envelope_some(self);
        }
    }
}

/// Phase   Length Ctr  Vol Env     Sweep
/// ---------------------------------------
/// odd     Clock       -           -
/// & 3==3  -           -           Clock
/// & 7==0  -           Clock       -
/// ---------------------------------------
/// Rate    256 Hz      64 Hz       128 Hz
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkipDivEvent {
    #[default]
    None,
    /// The next DIV-APU event is swallowed entirely.
    Skip,
    /// The event after a swallowed one runs without advancing the phase.
    Skipped,
}

#[inline(always)]
fn tick_length_all(apu: &mut Apu) {
    apu.ch1.tick_length(&mut apu.nr52);
    apu.ch2.tick_length(&mut apu.nr52);
    apu.ch3.tick_length(&mut apu.nr52);
    apu.ch4.tick_length(&mut apu.nr52);
}

#[inline(always)]
fn tick_envelope_some(apu: &mut Apu) {
    apu.ch1.tick_envelope();
    apu.ch2.tick_envelope();
    apu.ch4.tick_envelope();
}

/// FF26 — NR52: Audio master control
#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize)]
pub struct NR52 {
    byte: u8,
}

impl NR52 {
    #[inline]
    pub fn write(&mut self, value: u8) {
        let prev_enabled = self.is_audio_on();
        let new_enabled = get_bit_flag(value, 7);

        if !new_enabled && prev_enabled {
            // APU is turning off, clear all but Wave RAM
            self.byte = 0;
        } else if new_enabled {
            // Turn on APU
            self.byte |= 0b1000_0000;
        }
    }

    #[inline]
    pub fn read(&self) -> u8 {
        self.byte | 0b0111_0000 // Bits 4-6 always read as 1
    }

    #[inline]
    pub fn is_audio_on(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }

    #[inline]
    pub fn is_ch1_on(&self) -> bool {
        get_bit_flag(self.byte, Self::get_enable_bit_pos(ChannelType::CH1))
    }

    #[inline]
    pub fn is_ch2_on(&self) -> bool {
        get_bit_flag(self.byte, Self::get_enable_bit_pos(ChannelType::CH2))
    }

    #[inline]
    pub fn is_ch3_on(&self) -> bool {
        get_bit_flag(self.byte, Self::get_enable_bit_pos(ChannelType::CH3))
    }

    #[inline]
    pub fn is_ch4_on(&self) -> bool {
        get_bit_flag(self.byte, Self::get_enable_bit_pos(ChannelType::CH4))
    }

    /// Only the status of the channels’ generation circuits is reported
    #[inline]
    pub fn is_ch_on(&self, ch_type: ChannelType) -> bool {
        get_bit_flag(self.byte, Self::get_enable_bit_pos(ch_type))
    }

    #[inline]
    pub fn deactivate_ch(&mut self, ch_type: ChannelType) {
        set_bit(&mut self.byte, Self::get_enable_bit_pos(ch_type), false);
    }

    #[inline]
    pub fn deactivate_ch4(&mut self) {
        set_bit(
            &mut self.byte,
            Self::get_enable_bit_pos(ChannelType::CH4),
            false,
        );
    }

    #[inline]
    pub fn activate_ch1(&mut self) {
        set_bit(
            &mut self.byte,
            Self::get_enable_bit_pos(ChannelType::CH1),
            true,
        );
    }

    #[inline]
    pub fn activate_ch2(&mut self) {
        set_bit(
            &mut self.byte,
            Self::get_enable_bit_pos(ChannelType::CH2),
            true,
        );
    }

    #[inline]
    pub fn activate_ch3(&mut self) {
        set_bit(
            &mut self.byte,
            Self::get_enable_bit_pos(ChannelType::CH3),
            true,
        );
    }

    #[inline]
    pub fn activate_ch4(&mut self) {
        set_bit(
            &mut self.byte,
            Self::get_enable_bit_pos(ChannelType::CH4),
            true,
        );
    }

    #[inline]
    pub fn activate_ch(&mut self, ch_type: ChannelType) {
        set_bit(&mut self.byte, Self::get_enable_bit_pos(ch_type), true);
    }

    #[inline]
    pub fn on_dac_update(&mut self, dac_enabled: bool, ch_type: ChannelType) {
        // Disabling DAC always disables the channel
        if !dac_enabled {
            self.deactivate_ch(ch_type);
        }
    }

    #[inline]
    fn get_enable_bit_pos(ch_type: ChannelType) -> u8 {
        match ch_type {
            ChannelType::CH1 => 0,
            ChannelType::CH2 => 1,
            ChannelType::CH3 => 2,
            ChannelType::CH4 => 3,
        }
    }
}

/// FF25 — NR51:
/// Each channel can be panned hard left, center, hard right, or ignored entirely.
/// Setting a bit to 1 enables the channel to go into the selected output.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NR51 {
    pub byte: u8,
}

impl NR51 {
    pub fn ch4_left(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }
    pub fn ch3_left(&self) -> bool {
        get_bit_flag(self.byte, 6)
    }
    pub fn ch2_left(&self) -> bool {
        get_bit_flag(self.byte, 5)
    }
    pub fn ch1_left(&self) -> bool {
        get_bit_flag(self.byte, 4)
    }
    pub fn ch4_right(&self) -> bool {
        get_bit_flag(self.byte, 3)
    }
    pub fn ch3_right(&self) -> bool {
        get_bit_flag(self.byte, 2)
    }
    pub fn ch2_right(&self) -> bool {
        get_bit_flag(self.byte, 1)
    }
    pub fn ch1_right(&self) -> bool {
        get_bit_flag(self.byte, 0)
    }
}

/// FF24 — NR50: Master volume & VIN panning
/// A value of 0 is treated as a volume of 1 (very quiet), and a value of 7 is treated as a volume of 8 (no volume reduction). Importantly, the amplifier never mutes a non-silent input.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct NR50 {
    pub byte: u8,
}

impl NR50 {
    pub fn left_volume(&self) -> u8 {
        (self.byte >> 4) & 0b111 // Extract bits 6-4
    }

    pub fn right_volume(&self) -> u8 {
        self.byte & 0b111 // Extract bits 2-0
    }

    pub fn vin_left_enabled(&self) -> bool {
        self.byte & 0b1000_0000 != 0
    }

    pub fn vin_right_enabled(&self) -> bool {
        self.byte & 0b0000_1000 != 0
    }
}
