use crate::apu::APU_CLOCK_SPEED;
use crate::CPU_CLOCK_SPEED;

// The frame sequencer generates low frequency clocks for the modulation units. It is clocked by a 512 Hz timer.
// Step   Length Ctr  Vol Env     Sweep
// ---------------------------------------
// 0      Clock       -           -
// 1      -           -           -
// 2      Clock       -           Clock
// 3      -           -           -
// 4      Clock       -           -
// 5      -           -           -
// 6      Clock       -           Clock
// 7      -           Clock       -
// ---------------------------------------
// Rate   256 Hz      64 Hz       128 Hz

const CYCLES_DIV: u16 = (CPU_CLOCK_SPEED / APU_CLOCK_SPEED as u32) as u16;

pub struct FrameSequencer {
    counter: u16,
    step: u8,
}

impl FrameSequencer {
    // ticks every t-cycle
    pub fn tick(&mut self) {
        self.counter += 1;

        if self.counter >= CYCLES_DIV {
            match self.step {
                0 => {} // tick_length
                1 => {}
                2 => {} // tick length, sweep
                3 => {}
                4 => {} // tick_length
                5 => {}
                6 => {} // tick length, sweep
                7 => {} // tick envelope
                _ => unreachable!(),
            }

            self.counter -= CYCLES_DIV;

            self.step += 1;

            if self.step > 7 {
                self.step = 0;
            }
        }
    }
}
