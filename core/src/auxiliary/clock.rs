use crate::bus::Bus;
use crate::{auxiliary::dma::Dma, cpu::CPU_CLOCK_SPEED};
use serde::{Deserialize, Serialize};
use std::time::Instant;

const T_CYCLES_PER_M_CYCLE: usize = 4;
const NANOS_PER_SECOND: u32 = 1_000_000_000;
const T_CYCLE_DURATION_NANOS: f64 = NANOS_PER_SECOND as f64 / CPU_CLOCK_SPEED as f64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clock {
    #[serde(with = "crate::instant_serde")]
    pub time: Instant,
    pub bus: Bus,
    m_cycles: usize,
}

impl Default for Clock {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl Clock {
    pub fn new(bus: Bus) -> Self {
        Self {
            time: Instant::now(),
            bus,
            m_cycles: 0,
        }
    }

    #[inline(always)]
    pub fn calc_elapsed_nanos(&self) -> f64 {
        self.get_t_cycles() as f64 * self.get_t_cycle_duration_nanos()
    }

    #[inline(always)]
    pub fn reset(&mut self) {
        self.m_cycles = 0;
        self.time = Instant::now();
    }

    #[inline(always)]
    pub fn tick_m_cycles(&mut self, m_cycles: usize) {
        for _ in 0..m_cycles {
            self.m_cycles = self.m_cycles.wrapping_add(1);
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

            if self.bus.io.cgb_speed.double_speed && self.m_cycles % 2 != 0 {
                continue;
            }

            self.bus.io.ppu.tick(&mut self.bus.io.interrupts);
            self.bus.io.apu.tick();
        }
    }

    fn get_t_cycle_duration_nanos(&self) -> f64 {
        if self.bus.io.cgb_speed.double_speed {
            return T_CYCLE_DURATION_NANOS / 2.0;
        }

        T_CYCLE_DURATION_NANOS
    }
}
