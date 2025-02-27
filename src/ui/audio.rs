use crate::apu::{Apu, AUDIO_BUFFER_SIZE, SAMPLING_FREQUENCY};
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::{AudioSubsystem, Sdl};

pub struct GameAudio {
    device: AudioQueue<f32>,
    _audio_subsystem: AudioSubsystem,
}

impl GameAudio {
    pub fn new(sdl: &Sdl) -> Self {
        let audio_subsystem = sdl.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(SAMPLING_FREQUENCY as i32),
            channels: Some(2),
            samples: Some(AUDIO_BUFFER_SIZE as u16),
        };

        // creates the queue that is going to be used to update the
        // audio stream with new values during the main loop
        let device = audio_subsystem.open_queue(None, &desired_spec).unwrap();
        device.resume();

        Self {
            device,

            _audio_subsystem: audio_subsystem,
        }
    }

    pub fn play(&mut self, apu: &mut Apu) -> Result<(), String> {
        if apu.is_buffer_ready() {
            self.device.queue_audio(apu.take_buffer())?;
        }

        Ok(())
    }
}
