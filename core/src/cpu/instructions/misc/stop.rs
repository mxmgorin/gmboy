use crate::cpu::Cpu;

impl Cpu {
    #[inline(always)]
    pub fn execute_stop(&mut self) {
        match self.clock.bus.io.ppu.lcd.model {
            crate::emu::config::GbModel::Dmg => {},
            crate::emu::config::GbModel::Cgb => {
                self.clock.bus.io.cgb_speed.toggle();
                self.stop_m_cycles = 2050;
            }
        }
    }
}
