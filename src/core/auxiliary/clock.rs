use crate::bus::Bus;

pub const T_CYCLES_PER_M_CYCLE: usize = 4;

#[derive(Debug, Clone, Default)]
pub struct Clock {
    pub t_cycles: usize,
}

impl Clock {
    pub fn m_cycles(&mut self, m_cycles: usize, bus: &mut Bus) {
        for _ in 0..m_cycles {
            self.t_cycles(T_CYCLES_PER_M_CYCLE, bus);
            bus.dma_tick();
        }
    }
    
    pub fn get_m_cycles(&self) -> usize {
        self.t_cycles / T_CYCLES_PER_M_CYCLE
    }

    fn t_cycles(&mut self, t_cycles: usize, bus: &mut Bus) {
        for _ in 0..t_cycles {
            self.t_cycles = self.t_cycles.wrapping_add(1);

            bus.io.timer.tick(&mut bus.io.interrupts);
            bus.ppu.tick(&mut bus.io);
        }
    }
}
