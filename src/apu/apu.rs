use crate::apu::ch1_2_square::SquareChannel;
use crate::apu::ch3_wave::WaveChannel;
use crate::apu::ch4_noise::NoiseChannel;
use crate::apu::channel::ChannelType;
use crate::{get_bit_flag, set_bit};
use crate::apu::frame_sequencer::FrameSequencer;

pub const APU_CLOCK_SPEED: u16 = 512;

pub const AUDIO_MASTER_CONTROL_ADDRESS: u16 = 0xFF26;
pub const SOUND_PLANNING_ADDRESS: u16 = 0xFF25;
pub const MASTER_VOLUME_ADDRESS: u16 = 0xFF24;

pub struct Apu {
    // internal mem
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    audio_master_control: NR52,
    sound_panning: NR51,
    master_volume: NR50,
    // others
    counter: u16,
    frame_sequencer: FrameSequencer,
    
}

impl Apu {
    pub fn tick(&mut self) {
        self.frame_sequencer.tick();
        self.counter = self.counter.wrapping_add(1);
    }
}

/// FF26 — NR52: Audio master control
pub struct NR52 {
    pub byte: u8,
}

impl NR52 {
    /// (Read/Write) This controls whether the APU is powered on at all
    pub fn is_audio_on(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }

    pub fn is_ch_on(&self, ch_type: ChannelType) -> bool {
        get_bit_flag(self.byte, ch_type.get_enable_bit_pos())
    }

    pub fn is_ch4_on(&self) -> bool {
        get_bit_flag(self.byte, ChannelType::CH4.get_enable_bit_pos())
    }

    pub fn is_ch3_on(&self) -> bool {
        get_bit_flag(self.byte, ChannelType::CH3.get_enable_bit_pos())
    }

    pub fn is_ch2_on(&self) -> bool {
        get_bit_flag(self.byte, ChannelType::CH2.get_enable_bit_pos())
    }

    pub fn is_ch1_on(&self) -> bool {
        get_bit_flag(self.byte, ChannelType::CH1.get_enable_bit_pos())
    }

    pub fn disable_ch(&mut self, ch_type: ChannelType) {
        set_bit(&mut self.byte, ch_type.get_enable_bit_pos(), false);
    }
}

/// FF25 — NR51:
/// Each channel can be panned hard left, center, hard right, or ignored entirely.
/// Setting a bit to 1 enables the channel to go into the selected output.
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
pub struct NR50 {
    pub byte: u8,
}

impl NR50 {
    pub fn vin_left(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }

    pub fn left_volume(&self) -> u8 {
        self.byte & 0b0111_0000
    }

    pub fn vin_right(&self) -> bool {
        get_bit_flag(self.byte, 3)
    }

    pub fn right_volume(&self) -> u8 {
        self.byte & 0b0000_0111
    }
}
