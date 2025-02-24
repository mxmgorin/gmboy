use crate::apu::ch1_2_square::SquareChannel;
use crate::apu::ch3_wave::WaveChannel;
use crate::apu::ch4_noise::NoiseChannel;
use crate::get_bit_flag;

pub const AUDIO_MASTER_CONTROL_ADDRESS: u16 = 0xFF26;
pub const SOUND_PLANNING_ADDRESS: u16 = 0xFF25;
pub const MASTER_VOLUME_ADDRESS: u16 = 0xFF24;

pub struct Apu {
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    /// NR52
    control: AudioMasterControl,
    /// NR51
    sound_panning: SoundPanning,
    /// NR50
    master_volume_vin_panning: MasterVolumeVinPanning,
}

impl Apu {}

/// FF26 — NR52: Audio master control
pub struct AudioMasterControl {
    pub byte: u8,
}

impl AudioMasterControl {
    /// (Read/Write) This controls whether the APU is powered on at all
    pub fn audio_on(&self) -> bool {
        get_bit_flag(self.byte, 7)
    }
    pub fn ch4_on(&self) -> bool {
        get_bit_flag(self.byte, 3)
    }
    pub fn ch3_on(&self) -> bool {
        get_bit_flag(self.byte, 2)
    }
    pub fn ch2_on(&self) -> bool {
        get_bit_flag(self.byte, 2)
    }
    pub fn ch1_on(&self) -> bool {
        get_bit_flag(self.byte, 0)
    }
}

/// FF25 — NR51:
/// Each channel can be panned hard left, center, hard right, or ignored entirely.
/// Setting a bit to 1 enables the channel to go into the selected output.
pub struct SoundPanning {
    pub byte: u8,
}

impl SoundPanning {
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
pub struct MasterVolumeVinPanning {
    pub byte: u8,
}

impl MasterVolumeVinPanning {
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
