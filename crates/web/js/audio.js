// A minimal WebAudio playback queue.
//
// The emulator produces a burst of interleaved stereo samples (L, R, L, R, …)
// each frame. `push()` wraps each burst in an AudioBuffer and schedules it to
// play back-to-back, keeping a small lead over the audio clock and resyncing
// forward on underrun. No AudioWorklet / SharedArrayBuffer, so it works on plain
// static hosting (e.g. GitHub Pages) with no special COOP/COEP headers.
export class AudioScheduler {
  constructor(sampleRate) {
    this.sampleRate = sampleRate;
    this.ctx = null;
    this.nextTime = 0;
    this.lead = 0.03;      // target latency (~30ms) kept ahead of the audio clock
    this.maxQueue = 0.08;  // hard cap (~80ms): drop bursts beyond this so latency can't ratchet up
  }

  /** Create/resume the AudioContext. Must be called from a user gesture. */
  resume() {
    if (!this.ctx) {
      const AC = window.AudioContext || window.webkitAudioContext;
      try {
        this.ctx = new AC({ sampleRate: this.sampleRate });
      } catch (_) {
        this.ctx = new AC(); // browser refused the exact rate; it'll resample
      }
      this.nextTime = 0;
    }
    if (this.ctx.state === 'suspended') this.ctx.resume();
  }

  /** Schedule one burst of interleaved stereo f32 samples. No-op until resumed. */
  push(interleaved) {
    if (!this.ctx) return;
    const frames = interleaved.length >> 1;
    if (!frames) return;

    const now = this.ctx.currentTime;

    // Underrun: we fell behind (stall / first burst) → jump forward with a small lead.
    if (this.nextTime < now + 0.005) {
      this.nextTime = now + this.lead;
    }

    // Overrun: the queue is already deeper than maxQueue → drop this burst and let
    // it drain. `nextTime` never moves backward (that would overlap live sources),
    // so dropping is what actually bounds latency instead of letting it ratchet up.
    if (this.nextTime > now + this.maxQueue) {
      return;
    }

    const buf = this.ctx.createBuffer(2, frames, this.sampleRate);
    const l = buf.getChannelData(0);
    const r = buf.getChannelData(1);
    for (let i = 0; i < frames; i++) {
      l[i] = interleaved[i * 2];
      r[i] = interleaved[i * 2 + 1];
    }

    const src = this.ctx.createBufferSource();
    src.buffer = buf;
    src.connect(this.ctx.destination);
    src.start(this.nextTime);
    this.nextTime += buf.duration;
  }
}
