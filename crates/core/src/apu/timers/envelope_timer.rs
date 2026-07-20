use crate::apu::registers::NRx2;
use serde::{Deserialize, Serialize};

/// SameBoy's envelope pipeline: the 3-bit countdown decrements on every 64 Hz
/// frame-sequencer event; once it expires, the next DIV-APU *rising* edge
/// (the secondary event) reloads it and arms the envelope clock, and the
/// following frame-sequencer event applies the ±1 volume step. Reaching 0/15
/// in the step direction locks the envelope until the next trigger.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EnvelopeTimer {
    volume: u8,
    #[serde(default)]
    countdown: u8,
    #[serde(default)]
    clock: bool,
    #[serde(default)]
    should_lock: bool,
    #[serde(default)]
    locked: bool,
}

impl EnvelopeTimer {
    /// 64 Hz frame-sequencer event (sequencer phase & 7 == 7): the countdown
    /// only runs while the clock isn't armed.
    #[inline]
    pub fn countdown_tick(&mut self) {
        if !self.clock {
            self.countdown = self.countdown.wrapping_sub(1) & 7;
        }
    }

    /// Secondary event (DIV-APU rising edge): an active channel with an
    /// expired countdown reloads it from the pace and arms the clock
    /// (pace 0 arms nothing).
    #[inline]
    pub fn arm(&mut self, nrx2: NRx2, ch_active: bool) {
        if !ch_active || self.countdown != 0 {
            return;
        }

        let pace = nrx2.sweep_pace();
        self.countdown = pace;
        self.set_clock(pace != 0, nrx2.envelope_dir_up());
    }

    /// Frame-sequencer event: an armed clock applies the volume step.
    #[inline]
    pub fn tick(&mut self, nrx2: NRx2) {
        if !self.clock {
            return;
        }

        self.set_clock(false, false);

        if self.locked || nrx2.sweep_pace() == 0 {
            return;
        }

        if nrx2.envelope_dir_up() {
            self.volume = self.volume.wrapping_add(1) & 0xF;
        } else {
            self.volume = self.volume.wrapping_sub(1) & 0xF;
        }
    }

    #[inline]
    fn set_clock(&mut self, value: bool, dir_up: bool) {
        if self.clock == value {
            return;
        }

        if value {
            self.clock = true;
            self.should_lock =
                (self.volume == 0xF && dir_up) || (self.volume == 0 && !dir_up);
        } else {
            self.clock = false;
            self.locked |= self.should_lock;
        }
    }

    /// Zombie mode: an NRx2 write while the channel is active mutates the
    /// volume through glitchy paths (CGB D/E single-pass variant, SameBoy's
    /// _nrx2_glitch).
    pub fn nrx2_glitch(&mut self, new: u8, old: u8) {
        if self.clock {
            self.countdown = new & 7;
        }

        let mut should_tick = (new & 7) != 0 && (old & 7) == 0 && !self.locked;
        let should_invert = (new & 8) != (old & 8);

        if (new & 0xF) == 8 && (old & 0xF) == 8 && !self.locked {
            should_tick = true;
        }

        if should_invert {
            if new & 8 != 0 {
                if old & 7 == 0 && !self.locked {
                    self.volume ^= 0xF;
                } else {
                    self.volume = 0xEu8.wrapping_sub(self.volume) & 0xF;
                }

                should_tick = false;
            } else {
                self.volume = 0x10u8.wrapping_sub(self.volume) & 0xF;
            }
        }

        if should_tick {
            if new & 8 != 0 {
                self.volume = self.volume.wrapping_add(1) & 0xF;
            } else {
                self.volume = self.volume.wrapping_sub(1) & 0xF;
            }
        } else if new & 7 == 0 && self.clock {
            self.set_clock(false, false);
        }
    }

    #[inline(always)]
    pub fn get_volume(&self) -> u8 {
        self.volume
    }

    #[inline(always)]
    pub fn reload(&mut self, nrx2: NRx2) {
        self.volume = nrx2.initial_volume();
        self.countdown = nrx2.sweep_pace();
        self.clock = false;
        self.should_lock = false;
        self.locked = false;
    }
}
