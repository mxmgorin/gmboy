use crate::config::AudioConfig;
use core::apu::{MAX_SAMPLE_RATE_SKEW, SAMPLING_FREQUENCY};
use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::{AudioSubsystem, Sdl};

/// Queue fill the dynamic rate control converges to. Half of it is jitter
/// headroom (the emulator pushes in per-frame bursts), half is drain headroom.
const TARGET_LATENCY_MS: u32 = 50;
/// Hard cap; drops only happen when rate control is saturated for a long
/// stretch (e.g. unmuted turbo producing faster than real time).
const MAX_LATENCY_MS: u32 = 100;

pub struct AppAudio {
    device: AudioQueue<f32>,
    max_queue_size: u32,
    target_queue_size: u32,
    sample_rate: u32,
    _audio_subsystem: AudioSubsystem,
}

impl AppAudio {
    pub fn new(sdl: &Sdl, config: &AudioConfig) -> Self {
        let audio_subsystem = sdl.audio().unwrap();
        let desired_spec = AudioSpecDesired {
            freq: Some(SAMPLING_FREQUENCY as i32),
            channels: Some(2),
            samples: Some(config.buffer_size as u16),
        };

        let bytes_per_sample = size_of::<f32>() as u32;
        let bytes_per_second = desired_spec.freq.unwrap_or_default() as u32
            * desired_spec.channels.unwrap_or_default() as u32
            * bytes_per_sample;
        let bytes_per_ms = bytes_per_second / 1000;

        // creates the queue that is going to be used to update the
        // audio stream with new values during the main loop
        let device = audio_subsystem.open_queue(None, &desired_spec).unwrap();
        device.resume();

        Self {
            device,
            max_queue_size: MAX_LATENCY_MS * bytes_per_ms,
            target_queue_size: TARGET_LATENCY_MS * bytes_per_ms,
            sample_rate: SAMPLING_FREQUENCY,
            _audio_subsystem: audio_subsystem,
        }
    }

    pub fn queue(&mut self, output: &[f32]) {
        let mut fill = self.device.size();

        // An empty queue means playback stopped (startup, menu pause, the
        // rewind sleep). Prefill with silence up to the target so it resumes
        // at steady-state latency instead of riding the underrun edge while
        // rate control slowly builds the fill back up.
        if fill == 0 {
            let silence = vec![0.0f32; self.target_queue_size as usize / size_of::<f32>()];
            if let Err(err) = self.device.queue_audio(&silence) {
                log::error!("Failed queue_audio: {err}");
            }
            fill = self.device.size();
        }

        // Dynamic rate control: the emulator paces itself against the wall
        // clock while the device drains at its own crystal's 44.1 kHz, so
        // without feedback the queue drifts toward empty (underrun) or the
        // cap (dropped buffers) — both audible. Skewing the APU emission
        // rate toward the target fill absorbs the drift inaudibly.
        let deviation =
            (fill as f32 - self.target_queue_size as f32) / self.target_queue_size as f32;
        let skew = MAX_SAMPLE_RATE_SKEW as f32 * deviation.clamp(-1.0, 1.0);
        self.sample_rate = (SAMPLING_FREQUENCY as f32 - skew) as u32;

        if fill < self.max_queue_size {
            if let Err(err) = self.device.queue_audio(output) {
                log::error!("Failed queue_audio: {err}");
            };
        }
    }

    /// Rate the APU should emit at, per the last `queue()` measurement.
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}
