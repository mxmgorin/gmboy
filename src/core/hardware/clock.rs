use crate::bus::Bus;
use crate::cpu::interrupts::InterruptType;

const T_CYCLES_PER_M_CYCLE: usize = 4;

#[derive(Debug, Clone, Default)]
pub struct Clock {
    pub ticks: usize,
}

impl Clock {
    pub fn m_cycles(&mut self, m_cycles: usize, bus: &mut Bus) {
        for _ in 0..m_cycles {
            self.t_cycles(T_CYCLES_PER_M_CYCLE, bus);
            bus.dma_tick();
        }
    }

    fn t_cycles(&mut self, t_cycles: usize, bus: &mut Bus) {
        for _ in 0..t_cycles {
            self.ticks = self.ticks.wrapping_add(1);

            if bus.io.timer.tick() {
                bus.io.interrupts.request_interrupt(InterruptType::Timer);
                bus.ppu.tick();
            }
        }
    }
}
