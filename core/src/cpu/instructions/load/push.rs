use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn fetch_execute_push<const R1: u8>(&mut self) {
        self.clock.tick_m_cycles(1);

        let hi = (self.registers.get_register::<R1>() >> 8) & 0xFF;
        self.push(hi as u8);

        let lo = self.registers.get_register::<R1>() & 0xFF;
        self.push(lo as u8);
    }
}
