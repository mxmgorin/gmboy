use crate::auxiliary::dma::Dma;
use crate::bus::Bus;
use std::thread;
use std::time::{Duration, Instant};

pub const T_CYCLES_PER_M_CYCLE: usize = 4;

pub trait Tickable {
    /// Executes one T-Cycle
    fn tick(&mut self, bus: &mut Bus);
}

#[derive(Debug, Clone)]
pub struct Clock {
    pub start_time: Instant,
    pub t_cycles: usize,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            t_cycles: 0,
        }
    }
}

impl Clock {
    pub fn reset(&mut self) {
        self.t_cycles = 0;
        self.start_time = Instant::now();
    }

    pub fn m_cycles(&mut self, m_cycles: usize, bus: &mut Bus, ppu: &mut impl Tickable) {
        for _ in 0..m_cycles {
            self.t_cycles(T_CYCLES_PER_M_CYCLE, bus, ppu);
            Dma::tick(bus);
        }
    }

    pub fn get_m_cycles(&self) -> usize {
        self.t_cycles / T_CYCLES_PER_M_CYCLE
    }

    fn t_cycles(&mut self, t_cycles: usize, bus: &mut Bus, ppu: &mut impl Tickable) {
        for _ in 0..t_cycles {
            self.t_cycles = self.t_cycles.wrapping_add(1);
            bus.io.timer.tick(&mut bus.io.interrupts);
            ppu.tick(bus);
            bus.io.apu.tick();
        }
    }
}

const SLEEP_DURATION: Duration = Duration::from_millis(3);

pub fn sleep_spin(duration: Duration) {
    let start = Instant::now();

    // Sleep to avoid overshooting
    if duration > SLEEP_DURATION {
        thread::sleep(duration - SLEEP_DURATION);
    }

    // Spin the rest to get close to the target duration
    while start.elapsed() < duration {
        std::hint::spin_loop();
        //thread::yield_now();
    }
}
