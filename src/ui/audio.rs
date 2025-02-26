use sdl2::audio::{AudioDevice, AudioSpecDesired};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use crate::apu::SAMPLING_FREQUENCY;

pub fn create_audio_device(
    sdl_context: &sdl2::Sdl,
) -> Result<(AudioDevice<BufferedAudioCallback>, Arc<Mutex<VecDeque<u8>>>), String> {
    let audio_subsystem = sdl_context.audio()?;
    let desired_spec = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32 - 1000),
        channels: Some(2),
        samples: Some(1024),
    };

    let audio_buffer = Arc::new(Mutex::new(VecDeque::with_capacity(1024)));
    let audio_buffer_clone = audio_buffer.clone();
    let audio_device =
        audio_subsystem.open_playback(None, &desired_spec, move |_spec| BufferedAudioCallback {
            buffer: audio_buffer_clone,
        })?;

    audio_device.resume();

    Ok((audio_device, audio_buffer))
}

pub struct BufferedAudioCallback {
    buffer: Arc<Mutex<VecDeque<u8>>>,
}

impl sdl2::audio::AudioCallback for BufferedAudioCallback {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        let mut buffer = self.buffer.lock().unwrap();

        for out_sample in out.iter_mut() {
            if let Some(sample) =  buffer.pop_front() {
                *out_sample = sample;
            } else {
                //println!("no sample")
            }
        }
    }
}
