use crate::auxiliary::dma::Dma;
use crate::bus::Bus;
use crate::ppu::Ppu;
use serde::{Deserialize, Serialize};
use std::time::Instant;

pub const T_CYCLES_PER_M_CYCLE: usize = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clock {
    #[serde(with = "crate::instant_serde")]
    pub time: Instant,
    pub t_cycles: usize,
    pub bus: Bus,
    pub ppu: Ppu,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            t_cycles: 0,
            bus: Default::default(),
            ppu: Default::default(),
        }
    }
}

impl Clock {
    pub fn new(ppu: Ppu, bus: Bus) -> Self {
        Self {
            time: Instant::now(),
            ppu,
            bus,
            t_cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        self.t_cycles = 0;
        self.time = Instant::now();
    }

    pub fn m_cycles(&mut self, m_cycles: usize) {
        for _ in 0..m_cycles {
            self.t_cycles(T_CYCLES_PER_M_CYCLE);
            Dma::tick(&mut self.bus);
        }
    }

    pub fn get_m_cycles(&self) -> usize {
        self.t_cycles / T_CYCLES_PER_M_CYCLE
    }

    #[inline(always)]
    fn t_cycles(&mut self, t_cycles: usize) {
        for _ in 0..t_cycles {
            self.t_cycles = self.t_cycles.wrapping_add(1);
            self.bus.io.timer.tick(&mut self.bus.io.interrupts);
            self.ppu.tick(&mut self.bus);
            self.bus.io.apu.tick();
        }
    }
}
