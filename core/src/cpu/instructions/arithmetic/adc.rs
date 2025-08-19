use crate::cpu::instructions::InstructionSpec;
use crate::cpu::instructions::{DataDestination, FetchedData};
use crate::cpu::Cpu;

impl Cpu {
    #[inline]
    pub fn execute_adc(&mut self, fetched_data: FetchedData, _spec: InstructionSpec) {
        let DataDestination::Register(_) = fetched_data.dest else {
            unreachable!();
        };

        let u: u16 = fetched_data.value;
        let a: u16 = self.registers.a as u16;
        let c: u16 = self.registers.flags.get_c() as u16;

        self.registers.a = ((a + u + c) & 0xFF) as u8;

        self.registers.flags.set_z(self.registers.a == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h((a & 0xF) + (u & 0xF) + c > 0xF);
        self.registers.flags.set_c(a + u + c > 0xFF);
    }
}
