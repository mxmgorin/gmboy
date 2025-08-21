use crate::cpu::instructions::{DataDestination, DataSource};
use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    pub fn fetch_execute_ld_lh_spi8(&mut self) {
        self.fetch_lh_spi8();
        let h_flag = (self.registers.sp & 0xF) + (self.step_ctx.fetched_data.value & 0xF) >= 0x10;
        let c_flag =
            (self.registers.sp & 0xFF) + (self.step_ctx.fetched_data.value & 0xFF) >= 0x100;

        self.registers.flags.set_z(false);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(h_flag);
        self.registers.flags.set_c(c_flag);

        let offset_e = self.step_ctx.fetched_data.value as i8; // truncate to 8 bits (+8e)

        self.registers.set_register(
            RegisterType::HL,
            self.registers.sp.wrapping_add(offset_e as u16),
        );

        self.clock.m_cycles(1);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_d8<const R1: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        self.fetch_r_d8::<R1>();
        self.registers
            .set_register(r1, self.step_ctx.fetched_data.value);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_d16<const R1: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        self.fetch_r_d16::<R1>();
        self.registers
            .set_register(r1, self.step_ctx.fetched_data.value);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_r::<R1, R2>();
        self.ld_r_r::<R1, R2>();
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_mr<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr::<R1, R2>();
        self.ld_r_r::<R1, R2>();
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_mr_dec<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_r_mr_dec::<R1, R2>();
        self.ld_r_r::<R1, R2>();
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_a16<const R1: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        self.fetch_r_a16::<R1>();
        self.registers
            .set_register(r1, self.step_ctx.fetched_data.value);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_mr_r<const R1: u8, const R2: u8>(&mut self) {
        self.fetch_mr_r::<R1, R2>();
        let DataDestination::Memory(addr) = self.step_ctx.fetched_data.dest else {
            unreachable!()
        };

        self.ld_addr_r::<R2>(addr);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_a16_r<const R2: u8>(&mut self) {
        self.fetch_a16_r::<R2>();
        let DataDestination::Memory(addr) = self.step_ctx.fetched_data.dest else {
            unreachable!()
        };

        self.ld_addr_r::<R2>(addr);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_hli_r<const R2: u8>(&mut self) {
        self.fetch_hli_r::<R2>();
        let DataDestination::Memory(addr) = self.step_ctx.fetched_data.dest else {
            unreachable!()
        };

        self.ld_addr_r::<R2>(addr);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_r_hli<const R1: u8>(&mut self) {
        const R2: u8 = RegisterType::HL as u8;
        self.fetch_r_hli::<R1>();
        self.ld_r_r::<R1, R2>();
    }

    #[inline(always)]
    pub fn fetch_execute_ld_hld_r<const R2: u8>(&mut self) {
        self.fetch_hld_r::<R2>();
        let DataDestination::Memory(addr) = self.step_ctx.fetched_data.dest else {
            unreachable!()
        };

        self.ld_addr_r::<R2>(addr);
    }

    #[inline(always)]
    pub fn fetch_execute_ld_mr_d8<const R1: u8>(&mut self) {
        self.fetch_mr_d8::<R1>();
        let DataDestination::Memory(addr) = self.step_ctx.fetched_data.dest else {
            unreachable!()
        };

        self.write_to_memory(addr, self.step_ctx.fetched_data.value as u8);
    }

    #[inline(always)]
    pub fn execute_ld(&mut self) {
        match self.step_ctx.fetched_data.dest {
            DataDestination::Register(r) => {
                if let DataSource::Register(src_r) = self.step_ctx.fetched_data.source {
                    if r.is_16bit() && src_r.is_16bit() {
                        self.clock.m_cycles(1);
                    }
                }

                self.registers
                    .set_register(r, self.step_ctx.fetched_data.value);
            }
            DataDestination::Memory(addr) => match self.step_ctx.fetched_data.source {
                DataSource::Memory(_) => unreachable!(),
                DataSource::Register(r) | DataSource::MemoryRegister(r, _) => {
                    if r.is_16bit() {
                        self.write_to_memory(
                            addr + 1,
                            ((self.step_ctx.fetched_data.value >> 8) & 0xFF) as u8,
                        );
                        self.write_to_memory(addr, (self.step_ctx.fetched_data.value & 0xFF) as u8);
                    } else {
                        self.write_to_memory(addr, self.step_ctx.fetched_data.value as u8);
                    }
                }
                DataSource::Immediate => {
                    self.write_to_memory(addr, self.step_ctx.fetched_data.value as u8);
                }
            },
        }
    }

    #[inline(always)]
    fn ld_addr_r<const R2: u8>(&mut self, addr: u16) {
        let r2 = RegisterType::from_u8(R2);

        if r2.is_16bit() {
            self.write_to_memory(
                addr + 1,
                ((self.step_ctx.fetched_data.value >> 8) & 0xFF) as u8,
            );
            self.write_to_memory(addr, (self.step_ctx.fetched_data.value & 0xFF) as u8);
        } else {
            self.write_to_memory(addr, self.step_ctx.fetched_data.value as u8);
        }
    }

    #[inline(always)]
    fn ld_r_r<const R1: u8, const R2: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        let r2 = RegisterType::from_u8(R1);

        if r1.is_16bit() && r2.is_16bit() {
            self.clock.m_cycles(1);
        }

        self.registers
            .set_register(r1, self.step_ctx.fetched_data.value);
    }
}
