use crate::apu::ch3_wave::WaveChannel;
use crate::apu::{APU_CLOCK_SPEED, NR52};
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

#[derive(Debug, Clone, Default)]
pub struct FrameSequencer {
    counter: u16,
    step: u8,
}

impl FrameSequencer {
    // ticks every t-cycle
    pub fn tick(&mut self, master_ctrl: &mut NR52, ch3: &mut WaveChannel) {
        self.counter += 1;

        if self.counter >= CYCLES_DIV {
            match self.step {
                0 => ch3.tick_length(master_ctrl), // tick_length
                1 => {}
                2 => ch3.tick_length(master_ctrl), // tick length, sweep
                3 => {}
                4 => ch3.tick_length(master_ctrl), // tick_length
                5 => {}
                6 => ch3.tick_length(master_ctrl), // tick length, sweep
                7 => {}                            // tick envelope
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
