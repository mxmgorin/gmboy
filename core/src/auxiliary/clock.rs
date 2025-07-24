use crate::auxiliary::dma::Dma;
use crate::bus::Bus;
use std::time::Instant;

pub const T_CYCLES_PER_M_CYCLE: usize = 4;

pub trait Tickable {
    /// Executes one T-Cycle
    fn tick(&mut self, bus: &mut Bus);
}

#[derive(Debug, Clone)]
pub struct Clock {
    pub time: Instant,
    pub t_cycles: usize,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            t_cycles: 0,
        }
    }
}

impl Clock {
    pub fn reset(&mut self) {
        self.t_cycles = 0;
        self.time = Instant::now();
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
