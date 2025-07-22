use core::apu::{AUDIO_BUFFER_SIZE, SAMPLING_FREQ};
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::{AudioSubsystem, Sdl};

pub struct AppAudio {
    device: AudioQueue<f32>,
    max_queue_size: u32,
    _audio_subsystem: AudioSubsystem,
}

impl AppAudio {
    pub fn new(sdl: &Sdl) -> Self {
        let audio_subsystem = sdl.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(SAMPLING_FREQ as i32),
            channels: Some(2),
            samples: Some(AUDIO_BUFFER_SIZE as u16),
        };

        // Avoid overfilling the SDL2 audio queue
        let bytes_per_sample = size_of::<f32>() as u32;
        let bytes_per_second = desired_spec.freq.unwrap_or_default() as u32
            * desired_spec.channels.unwrap_or_default() as u32
            * bytes_per_sample;
        let bytes_per_ms = bytes_per_second / 1000;
        let max_queue_size = 100 * bytes_per_ms; // e.g., limit to 100 ms latency

        // creates the queue that is going to be used to update the
        // audio stream with new values during the main loop
        let device = audio_subsystem.open_queue(None, &desired_spec).unwrap();
        device.resume();

        Self {
            device,
            max_queue_size,
            _audio_subsystem: audio_subsystem,
        }
    }

    pub fn queue(&mut self, output: &[f32]) {
        if self.device.size() < self.max_queue_size {
            if let Err(err) = self.device.queue_audio(output) {
                eprintln!("Failed queue_audio: {err}");
            };
        }
    }
}
