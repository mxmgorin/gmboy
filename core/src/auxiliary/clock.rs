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
    m_cycles: usize,
    pub bus: Bus,
    pub ppu: Ppu,
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            m_cycles: 0,
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
            m_cycles: 0,
        }
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.m_cycles = 0;
        self.time = Instant::now();
    }

    #[inline(always)]
    pub fn tick_m_cycles(&mut self, m_cycles: usize) {
        self.m_cycles = self.m_cycles.wrapping_add(m_cycles);

        for _ in 0..m_cycles {
            self.tick_t_cycles(T_CYCLES_PER_M_CYCLE);
            Dma::tick(&mut self.bus);
        }
    }

    #[inline(always)]
    pub fn get_m_cycles(&self) -> usize {
        self.m_cycles
    }

    #[inline(always)]
    pub fn get_t_cycles(&self) -> usize {
        self.m_cycles * T_CYCLES_PER_M_CYCLE
    }

    #[inline(always)]
    fn tick_t_cycles(&mut self, t_cycles: usize) {
        for _ in 0..t_cycles {
            self.bus.io.timer.tick(&mut self.bus.io.interrupts);
            self.ppu.tick(&mut self.bus);
            self.bus.io.apu.tick();
        }
    }
}
