use crate::bus::Bus;
use crate::ppu::Ppu;

pub const T_CYCLES_PER_M_CYCLE: usize = 4;

#[derive(Debug, Clone, Default)]
pub struct Clock {
    pub t_cycles: usize,
    pub ppu: Ppu,
}

impl Clock {
    pub fn m_cycles(&mut self, m_cycles: usize, bus: &mut Bus) {
        for _ in 0..m_cycles {
            self.t_cycles(T_CYCLES_PER_M_CYCLE, bus);
            self.dma_tick(bus);
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

    pub fn dma_tick(&mut self, bus: &mut Bus) {
        if !bus.dma.is_active {
            return;
        }

        if bus.dma.start_delay > 0 {
            bus.dma.start_delay -= 1;
            return;
        }

        let addr = (bus.dma.address as u16 * 0x100).wrapping_add(bus.dma.current_byte as u16);
        let value = bus.read(addr);
        bus.oam_ram.write(bus.dma.current_byte as u16, value);
        bus.dma.current_byte += bus.dma.current_byte.wrapping_add(1);
        bus.dma.is_active = bus.dma.current_byte < 0xA0; // 160
    }
}
