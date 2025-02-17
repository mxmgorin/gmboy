use crate::auxiliary::dma::Dma;
use crate::bus::Bus;
use crate::ppu::Ppu;

pub const T_CYCLES_PER_M_CYCLE: usize = 4;

#[derive(Debug, Clone, Default)]
pub struct Clock {
    pub t_cycles: usize,
    pub ppu: Ppu,
}

impl Clock {
    pub fn new(ppu: Ppu) -> Self {
        Self { t_cycles: 0, ppu }
    }

    pub fn m_cycles(&mut self, m_cycles: usize, bus: &mut Bus) {
        for _ in 0..m_cycles {
            self.t_cycles(T_CYCLES_PER_M_CYCLE, bus);
            Dma::tick(bus);
        }
    }

    pub fn get_m_cycles(&self) -> usize {
        self.t_cycles / T_CYCLES_PER_M_CYCLE
    }

    fn t_cycles(&mut self, t_cycles: usize, bus: &mut Bus) {
        for _ in 0..t_cycles {
            self.t_cycles = self.t_cycles.wrapping_add(1);

            bus.io.timer.tick(&mut bus.io.interrupts);
            self.ppu.tick(bus);
        }
    }
}
