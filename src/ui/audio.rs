use std::time::Duration;
use crate::apu::{Apu, AUDIO_BUFFER_SIZE, SAMPLING_FREQ};
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::{AudioSubsystem, Sdl};
use crate::auxiliary::clock::spin_wait;

pub struct GameAudio {
    device: AudioQueue<f32>,
    _audio_subsystem: AudioSubsystem,
}

impl GameAudio {
    pub fn new(sdl: &Sdl) -> Self {
        let audio_subsystem = sdl.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(SAMPLING_FREQ as i32),
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

    pub fn update(&mut self, apu: &mut Apu) -> Result<(), String> {
        if (self.device.size() as usize) > AUDIO_BUFFER_SIZE * 10 {
            spin_wait(Duration::from_millis(10));
        }
        
        if apu.output_ready() {
            self.device.queue_audio(apu.take_output())?;
        }

        Ok(())
    }
}
