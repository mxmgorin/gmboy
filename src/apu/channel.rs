// Square 1: Sweep -> Timer -> Duty -> Length Counter -> Envelope -> Mixer
// 
// Square 2:          Timer -> Duty -> Length Counter -> Envelope -> Mixer
// 
// Wave:              Timer -> Wave -> Length Counter -> Volume   -> Mixer
// 
// Noise:             Timer -> LFSR -> Length Counter -> Envelope -> Mixer

#[derive(Clone, Debug)]
pub enum ChannelType {
    CH1,
    CH2,
    CH3,
    CH4,
}

impl ChannelType {
    pub fn get_enable_bit_pos(&self) -> u8 {
        match self {
            ChannelType::CH1 => 0,
            ChannelType::CH2 => 1,
            ChannelType::CH3 => 2,
            ChannelType::CH4 => 3,
        }
    }
}
