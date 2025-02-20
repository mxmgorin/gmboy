use crate::cpu::interrupts::{InterruptType, Interrupts};
use crate::{get_bit_flag, get_bit_flag16};

pub const TIMER_DIV_ADDRESS: u16 = 0xFF04;
pub const TIMER_TIMA_ADDRESS: u16 = 0xFF05;
pub const TIMER_TMA_ADDRESS: u16 = 0xFF06;
pub const TIMER_TAC_ADDRESS: u16 = 0xFF07;
pub const TIMER_TAC_M_CYCLES: [usize; 4] = [256, 4, 16, 64];
pub const TIMER_TAC_UNUSED_MASK: u8 = 0b1111_1000;

// with 3 passes
// test mooneye::test_timer_rapid_toggle
// test mooneye::test_timer_tima_reload
// but fails
// test mooneye::tma_write_reloading
// test mooneye::tma_write_reloading

// with 4 passes
// test mooneye::tma_write_reloading
// test mooneye::tma_write_reloading
// but fails
// test mooneye::test_timer_rapid_toggle
// test mooneye::test_timer_tima_reload
const TIMA_RELOAD_DELAY_TICKS: usize = 4; // seems like must be 4 (1 M-cycle delay)
const TAC_ENABLE_BIT: u8 = 2;

// #1 During the strange cycle [A] you can prevent the IF flag from being set and prevent the TIMA from
// being reloaded from TMA by writing a value to TIMA. That new value will be the one that stays in
// the TIMA register after the instruction. Writing to DIV, TAC or other registers won't prevent the
// IF flag from being set or TIMA from being reloaded.

// #2 If you write to TIMA during the cycle that TMA is being loaded to it [B], the write will be
// ignored and TMA value will be written to TIMA instead.

// #3 If register IF is written during [B], the written value will overwrite the automatic flag set
// to '1'. If a '0' is written during this cycle, the interrupt won't happen.

// #4 If TMA is written the same cycle it is loaded to TIMA [B], TIMA is also loaded with that value.

#[derive(Debug, Clone, Default)]
pub struct FallingEdgeDetector {
    pub prev_result: bool,
}

impl FallingEdgeDetector {
    pub fn detect(&mut self, div: u16, tac: u8) -> bool {
        let clock_bit = get_bit_flag16(div, get_clock_bit_position(tac));
        let enable_bit = get_bit_flag(tac, TAC_ENABLE_BIT);
        let and_result = clock_bit && enable_bit;

        let is_falling_edge = self.prev_result && !and_result;
        self.prev_result = and_result;

        is_falling_edge
    }
}

fn get_clock_bit_position(tac: u8) -> u8 {
    match tac & 0b11 {
        // 0b00 (4096 Hz): div bit 9, increment every 256 M-cycles
        0b00 => 9,
        // 0b01 (262144 Hz): div bit 3, increment every 4 M-cycles
        0b01 => 3,
        // 0b10 (65536 Hz): div bit 5, increment every 16 M-cycles
        0b10 => 5,
        // 0b11 (16384 Hz): div bit 7, increment every 64 M-cycles
        0b11 => 7,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone)]
pub struct Timer {
    // registers
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    // additional info
    falling_edge_detector: FallingEdgeDetector,
    tima_overflow_tma_write: Option<u8>,
    tima_overflow_ticks: Option<usize>,
    disabling_glitch: bool,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            // This value depends on the model. For the original Game Boy (DMG) it is 0xABCC.
            div: 0xABCC,
            tima: 0,
            tma: 0,
            tac: 0,
            tima_overflow_tma_write: None,
            falling_edge_detector: Default::default(),
            tima_overflow_ticks: None,
            disabling_glitch: false,
        }
    }
}

impl Timer {
    pub fn tick(&mut self, interrupts: &mut Interrupts) {
        // TIMA overflowed during the last cycle
        if let Some(tima_overflow_ticks) = self.tima_overflow_ticks.as_mut() {
            if *tima_overflow_ticks == TIMA_RELOAD_DELAY_TICKS || self.disabling_glitch {
                self.tima = self.tma;
                interrupts.request_interrupt(InterruptType::Timer);

                // reset after overflow fully handled
                self.tima_overflow_ticks = None;
                self.disabling_glitch = false;
            } else {
                //println!("SKIP {}", tima_overflow_ticks);
                *tima_overflow_ticks += 1;
            }
        }

        self.div = self.div.wrapping_add(1);

        if self.falling_edge_detector.detect(self.div, self.tac) {
            self.inc_tima();
        }
    }

    fn inc_tima(&mut self) {
        let (tima, tima_overflow) = self.tima.overflowing_add(1);
        self.write_tima(tima);

        if tima_overflow && self.tima_overflow_ticks.is_none() {
            // Timer interrupt is delayed 4 ticks from the TIMA overflow.
            // The TMA reload to TIMA is also delayed for 1 tick.
            // After overflowing TIMA, the value in TIMA is 00, not TMA.
            self.tima = 0x00;
            self.tima_overflow_ticks = Some(0);
        }
    }

    fn get_clock_bit_position(&self) -> u8 {
        match self.tac & 0b11 {
            // 0b00 (4096 Hz): div bit 9, increment every 256 M-cycles
            0b00 => 9,
            // 0b01 (262144 Hz): div bit 3, increment every 4 M-cycles
            0b01 => 3,
            // 0b10 (65536 Hz): div bit 5, increment every 16 M-cycles
            0b10 => 5,
            // 0b11 (16384 Hz): div bit 7, increment every 64 M-cycles
            0b11 => 7,
            _ => unreachable!(),
        }
    }

    fn is_enabled(&self) -> bool {
        // If bit 2 of TAC is set to 0 then the timer is disabled
        self.tac & (1 << TAC_ENABLE_BIT) != 0
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            TIMER_DIV_ADDRESS => self.reset_div(),
            TIMER_TIMA_ADDRESS => self.write_tima(value),
            TIMER_TMA_ADDRESS => {
                if self.tima_overflow_ticks.is_some() {
                    self.tima_overflow_tma_write = Some(value);
                }

                self.tma = value;
            }
            TIMER_TAC_ADDRESS => self.write_tac(value),
            _ => panic!("Invalid Timer address: {:02X}", address),
        }
    }

    fn write_tima(&mut self, value: u8) {
        if let Some(overflow_ticks) = self.tima_overflow_ticks {
            if overflow_ticks == TIMA_RELOAD_DELAY_TICKS {
                // case #2: the same tick on which the reload occurs - ignore write
                self.tima = self.tma;
                return;
            } else {
                // case #1: write during 4-ticks delay - abort handling
                self.tima_overflow_ticks = None;
            }
        }

        self.tima = value;
    }

    pub fn reset_div(&mut self) {
        self.div = 0;

        // - When writing to DIV register the TIMA register can be increased if the counter has reached half
        // the clocks it needs to increase because the selected bit by the multiplexer will go from 1 to 0 (which
        // is a falling edge, that will be detected by the falling edge detector).
        //if self.is_enabled() && self.is_falling_edge(self.prev_div) {
        //    self.inc_tima();
        //}
    }

    pub fn write_tac(&mut self, value: u8) {
        let old_is_enabled = self.is_enabled();
        let old_clock_bit = self.get_clock_bit_position();

        self.tac = value;

        let new_is_enabled = self.is_enabled();

        // - When disabling the timer, if the corresponding bit in the system counter is set to 1, the falling edge
        // detector will see a change from 1 to 0, so TIMA will increase. This means that whenever half the
        // clocks of the count are reached, TIMA will increase when disabling the timer.
        // Correctly emulated by detect_falling_edge

        // fix rapid_toggle
        self.disabling_glitch =
            (self.div & (1 << old_clock_bit)) != 0 && old_is_enabled && !new_is_enabled;

        if !self.disabling_glitch {
            // - When changing TAC register value, if the old selected bit by the multiplexer was 0, the new one is
            // 1, and the new enable bit of TAC is set to 1, it will increase TIMA.
            let enabling_glitch = (self.div & (1 << old_clock_bit)) == 0
                && (self.div & (1 << self.get_clock_bit_position())) != 0
                && new_is_enabled;

            if enabling_glitch {
                self.inc_tima();
            }
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            TIMER_DIV_ADDRESS => (self.div >> 8) as u8, // most significant byte in a 16-bit long number
            TIMER_TIMA_ADDRESS => {
                // fix for tima_reload
                if let Some(overflow_ticks) = self.tima_overflow_ticks {
                    if overflow_ticks == TIMA_RELOAD_DELAY_TICKS {
                        return self.tma;
                    }
                }

                self.tima
            }
            TIMER_TMA_ADDRESS => self.tma,
            TIMER_TAC_ADDRESS => self.tac | TIMER_TAC_UNUSED_MASK,
            _ => panic!("Invalid Timer address: {:02X}", address),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::auxiliary::timer::Timer;
    use crate::cpu::interrupts::Interrupts;

    #[test]
    pub fn test_timer_tima_01() {
        let mut timer = Timer {
            tac: 0b101,
            div: 0,

            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();
        let mut prev_tima = 0;
        let cycles = 16;

        for i in 1..=500 {
            timer.tick(&mut interrupts);

            if prev_tima != timer.tima {
                assert_eq!(i % cycles, 0);
            }

            if i == cycles {
                assert_eq!(timer.tima, (cycles / i) as u8);
            }

            prev_tima = timer.tima;
        }
    }

    #[test]
    pub fn test_timer_tima_10() {
        let mut timer = Timer {
            tac: 0b110,
            div: 0,

            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();
        let mut prev_tima = 0;
        let cycles = 64;

        for i in 1..=1000_usize {
            timer.tick(&mut interrupts);

            if prev_tima != timer.tima {
                assert_eq!(i % cycles, 0);
            }

            if i == cycles {
                assert_eq!(timer.tima, (cycles / i) as u8);
            }

            prev_tima = timer.tima;
        }
    }

    #[test]
    pub fn test_timer_tima_11() {
        let mut timer = Timer {
            tac: 0b111,
            div: 0,

            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();
        let mut prev_tima = 0;
        let cycles = 256;

        for i in 1..=10000_usize {
            timer.tick(&mut interrupts);

            if prev_tima != timer.tima {
                assert_eq!(i % cycles, 0);
            }

            if i == cycles {
                assert_eq!(timer.tima, (cycles / i) as u8);
            }

            prev_tima = timer.tima;
        }
    }

    #[test]
    pub fn test_timer_tima_00() {
        let mut timer = Timer {
            tac: 0b100,
            div: 0,

            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();
        let mut prev_tima = 0;
        let cycles = 1024;

        for i in 1..=100000_usize {
            timer.tick(&mut interrupts);

            if prev_tima != timer.tima {
                assert_eq!(i % cycles, 0);
            }

            if i == cycles {
                assert_eq!(timer.tima, (cycles / i) as u8);
            }

            prev_tima = timer.tima;
        }
    }

    #[test]
    pub fn test_timer_tima_00_trigger() {
        let mut timer = Timer {
            tac: 0b100,
            div: 0,

            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();

        for _ in 1..=512 {
            timer.tick(&mut interrupts);
        }

        timer.reset_div();
        timer.tick(&mut interrupts);

        assert_eq!(1, timer.tima);
    }

    #[test]
    pub fn test_timer_tima_01_trigger() {
        let mut timer = Timer {
            tac: 0b101,
            div: 0,

            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();

        for _ in 1..=8 {
            timer.tick(&mut interrupts);
        }

        timer.reset_div();
        timer.tick(&mut interrupts);

        assert_eq!(1, timer.tima);
    }

    #[test]
    pub fn test_timer_tima_10_trigger() {
        let mut timer = Timer {
            tac: 0b110,
            div: 0,

            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();

        for _ in 1..=32 {
            timer.tick(&mut interrupts);
        }

        timer.reset_div();
        timer.tick(&mut interrupts);

        assert_eq!(1, timer.tima);
    }

    #[test]
    pub fn test_timer_tima_11_trigger() {
        let mut timer = Timer {
            tac: 0b111,
            div: 0,

            ..Timer::default()
        };
        let mut interrupts = Interrupts::default();

        for _ in 1..=128 {
            timer.tick(&mut interrupts);
        }

        timer.reset_div();
        timer.tick(&mut interrupts);

        assert_eq!(1, timer.tima);
    }
}
