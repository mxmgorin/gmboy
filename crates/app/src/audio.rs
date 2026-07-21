use crate::config::AudioConfig;
use core::apu::{MAX_SAMPLE_RATE_SKEW, SAMPLING_FREQUENCY};
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::{AudioSubsystem, Sdl};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Lock-free single-producer single-consumer ring: the emu thread pushes
/// mixed samples, the SDL audio thread pops them from its callback. Indices
/// grow monotonically and are masked on access (capacity is a power of two).
struct SpscRing {
    buf: Box<[UnsafeCell<f32>]>,
    mask: usize,
    /// Write position, advanced only by the producer (emu thread).
    head: AtomicUsize,
    /// Read position, advanced only by the consumer (audio thread).
    tail: AtomicUsize,
}

// SAFETY: the Acquire/Release pairing on head/tail guarantees the producer
// only writes slots the consumer has released and vice versa; each side
// advances only its own index, so no slot is ever accessed from both threads
// at once.
unsafe impl Sync for SpscRing {}
unsafe impl Send for SpscRing {}

impl SpscRing {
    fn new(min_capacity: usize) -> Arc<Self> {
        let capacity = min_capacity.next_power_of_two();

        Arc::new(Self {
            buf: (0..capacity).map(|_| UnsafeCell::new(0.0)).collect(),
            mask: capacity - 1,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        })
    }

    fn len(&self) -> usize {
        self.head
            .load(Ordering::Acquire)
            .wrapping_sub(self.tail.load(Ordering::Acquire))
    }

    /// Producer only. Returns how many samples fit; the rest is dropped.
    fn push(&self, data: &[f32]) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);
        let free = self.buf.len() - head.wrapping_sub(tail);
        let n = data.len().min(free);

        for (i, &sample) in data[..n].iter().enumerate() {
            // SAFETY: slots [head, head+n) are free — the consumer reads
            // strictly below `head`, which we only advance after the writes.
            unsafe { *self.buf[head.wrapping_add(i) & self.mask].get() = sample };
        }

        self.head.store(head.wrapping_add(n), Ordering::Release);
        n
    }

    /// Consumer only. Returns how many samples were filled.
    fn pop_into(&self, out: &mut [f32]) -> usize {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        let n = out.len().min(head.wrapping_sub(tail));

        for (i, slot) in out[..n].iter_mut().enumerate() {
            // SAFETY: slots [tail, tail+n) are published — the producer
            // writes strictly at/above `head` and advanced it after writing.
            *slot = unsafe { *self.buf[tail.wrapping_add(i) & self.mask].get() };
        }

        self.tail.store(tail.wrapping_add(n), Ordering::Release);
        n
    }
}

/// The pull side: SDL calls this on its audio thread whenever the device
/// needs a chunk. An empty ring pads with silence (underrun).
struct RingSource {
    ring: Arc<SpscRing>,
}

impl AudioCallback for RingSource {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let n = self.ring.pop_into(out);
        out[n..].fill(0.0);
    }
}

pub struct AppAudio {
    ring: Arc<SpscRing>,
    /// Ring fill (in samples) the dynamic rate control converges to.
    target_fill: usize,
    /// Hard cap; drops only happen when rate control is saturated for a long
    /// stretch (e.g. unmuted turbo producing faster than real time).
    max_fill: usize,
    sample_rate: u32,
    _device: AudioDevice<RingSource>,
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

        let samples_per_ms = SAMPLING_FREQUENCY as usize * 2 / 1000;
        let target_fill = config.latency_ms as usize * samples_per_ms;
        let max_fill = target_fill * 2;
        // headroom above the cap so a full burst still fits while the
        // callback drains
        let ring = SpscRing::new(max_fill + config.buffer_size * 2);

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |_spec| RingSource {
                ring: ring.clone(),
            })
            .unwrap();
        device.resume();

        Self {
            ring,
            target_fill,
            max_fill,
            sample_rate: SAMPLING_FREQUENCY,
            _device: device,
            _audio_subsystem: audio_subsystem,
        }
    }

    pub fn queue(&mut self, output: &[f32]) {
        let mut fill = self.ring.len();

        // An empty ring means playback stopped (startup, menu pause, the
        // rewind sleep). Prefill with silence up to the target so it resumes
        // at steady-state latency instead of riding the underrun edge while
        // rate control slowly builds the fill back up.
        if fill == 0 {
            let silence = vec![0.0f32; self.target_fill];
            fill = self.ring.push(&silence);
        }

        // Dynamic rate control: the emulator paces itself against the wall
        // clock while the device drains at its own crystal's 44.1 kHz, so
        // without feedback the ring drifts toward empty (underrun) or the
        // cap (dropped samples) — both audible. Skewing the APU emission
        // rate toward the target fill absorbs the drift inaudibly.
        let deviation = (fill as f32 - self.target_fill as f32) / self.target_fill as f32;
        let skew = MAX_SAMPLE_RATE_SKEW as f32 * deviation.clamp(-1.0, 1.0);
        self.sample_rate = (SAMPLING_FREQUENCY as f32 - skew) as u32;

        if fill < self.max_fill {
            self.ring.push(output);
        }
    }

    /// Rate the APU should emit at, per the last `queue()` measurement.
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ring_roundtrip() {
        let ring = SpscRing::new(8);
        assert_eq!(ring.push(&[1.0, 2.0, 3.0]), 3);
        assert_eq!(ring.len(), 3);

        let mut out = [0.0f32; 2];
        assert_eq!(ring.pop_into(&mut out), 2);
        assert_eq!(out, [1.0, 2.0]);
        assert_eq!(ring.len(), 1);
    }

    #[test]
    fn test_ring_wraparound_and_overflow() {
        let ring = SpscRing::new(4); // capacity 4
        let mut out = [0.0f32; 4];

        for round in 0..10 {
            let base = round as f32 * 10.0;
            assert_eq!(ring.push(&[base, base + 1.0]), 2);
            assert_eq!(ring.pop_into(&mut out[..2]), 2);
            assert_eq!(&out[..2], &[base, base + 1.0]);
        }

        // overflow: only the free space is accepted
        assert_eq!(ring.push(&[1.0; 6]), 4);
        assert_eq!(ring.push(&[2.0]), 0);
    }
}
