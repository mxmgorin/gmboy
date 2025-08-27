use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_add_sp_e8(&mut self) {
        let lhs = self.registers.sp;
        let rhs = self.read_pc();

        self.clock.tick_m_cycles(2);
        self.registers.sp = self.registers.sp.wrapping_add(rhs as i8 as u16);
        self.registers.flags.op_add_sp_e8(lhs, rhs as u16);
    }

    #[inline(always)]
    pub fn fetch_execute_add_r_r<const R1: u8, const R2: u8>(&mut self) {
        let val = self.registers.get_register::<R2>();
        self.execute_add::<R1>(val);
    }

    #[inline(always)]
    pub fn fetch_execute_add_r_mr<const R1: u8, const R2: u8>(&mut self) {
        let val = self.read_mr::<R2>();
        self.execute_add::<R1>(val as u16);
    }

    #[inline(always)]
    pub fn fetch_execute_add_r_d8<const R1: u8>(&mut self) {
        let val = self.read_pc();
        self.execute_add::<R1>(val as u16);
    }

    #[inline(always)]
    pub fn execute_add<const R1: u8>(&mut self, value: u16) {
        if RegisterType::from_u8(R1).is_16bit() {
            // but not for SP
            let lhs = self.registers.get_register::<R1>();
            let rhs = value;
            let result = lhs.wrapping_add(rhs);
            self.registers.set_register::<R1>(result);
            self.clock.tick_m_cycles(1);
            self.registers.flags.op_add16(lhs, rhs);
        } else {
            let lhs = self.registers.get_register8::<R1>();
            let rhs = value as u8;
            let result = lhs.wrapping_add(rhs);
            self.registers.set_register8::<R1>(result);
            self.registers.flags.op_add8(lhs, rhs, 0, result);
        }
    }
}
