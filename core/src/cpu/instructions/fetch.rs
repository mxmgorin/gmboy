use crate::cpu::{Cpu, RegisterType};
use serde::{Deserialize, Serialize};

impl Cpu {
    #[inline(always)]
    pub fn fetch_r<const R: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R);

        self.step_ctx.fetched_data = FetchedData {
            value: self.registers.read_register(r1),
            source: DataSource::Register(r1),
            dest: DataDestination::Register(r1),
        };
    }

    #[inline(always)]
    pub fn fetch_r_r<const R1: u8, const R2: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        let r2 = RegisterType::from_u8(R2);

        self.step_ctx.fetched_data = FetchedData {
            value: self.registers.read_register(r2),
            source: DataSource::Register(r2),
            dest: DataDestination::Register(r1),
        };
    }

    #[inline(always)]
    pub fn fetch_r_d8<const R: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R);

        self.step_ctx.fetched_data = FetchedData {
            value: self.read_pc() as u16,
            source: DataSource::Immediate,
            dest: DataDestination::Register(r1),
        };
    }

    #[inline(always)]
    pub fn fetch_d16(&mut self) {
        self.step_ctx.fetched_data = FetchedData {
            dest: DataDestination::Memory(0),
            value: self.read_pc16(),
            source: DataSource::Immediate,
        };
    }

    #[inline(always)]
    pub fn fetch_r_d16<const R: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R);

        self.step_ctx.fetched_data = FetchedData {
            value: self.read_pc16(),
            source: DataSource::Immediate,
            dest: DataDestination::Register(r1),
        };
    }

    #[inline(always)]
    pub fn fetch_r_mr<const R1: u8, const R2: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        let r2 = RegisterType::from_u8(R2);
        let addr = self.registers.read_register(r2);

        self.step_ctx.fetched_data = FetchedData {
            value: self.read_memory(addr),
            source: DataSource::MemoryRegister(r2, addr),
            dest: DataDestination::Register(r1),
        };
    }

    #[inline(always)]
    pub fn fetch_r_hmr_a_c(&mut self) {
        let r1 = RegisterType::A;
        let r2 = RegisterType::C;
        let addr = self.registers.read_register(r2);
        let addr = 0xFF00 | addr;

        self.step_ctx.fetched_data = FetchedData {
            value: self.read_memory(addr),
            source: DataSource::MemoryRegister(r2, addr),
            dest: DataDestination::Register(r1),
        };
    }

    #[inline(always)]
    pub fn fetch_mr_r<const R1: u8, const R2: u8>(&mut self) {
        let r1 = RegisterType::from_u8(R1);
        let r2 = RegisterType::from_u8(R2);

        self.step_ctx.fetched_data = FetchedData {
            value: self.registers.read_register(r2),
            source: DataSource::Register(r2),
            dest: DataDestination::Memory(self.registers.read_register(r1)),
        };
    }

    #[inline(always)]
    pub fn fetch_r_hli_a(&mut self) {
        let r1 = RegisterType::A;

        let addr = self.registers.read_register(RegisterType::HL);
        self.step_ctx.fetched_data = FetchedData {
            value: self.read_memory(addr),
            source: DataSource::MemoryRegister(RegisterType::HL, addr),
            dest: DataDestination::Register(r1),
        };

        self.registers.set_hl(addr.wrapping_add(1));
    }

    #[inline(always)]
    pub fn fetch_r_hld_a(&mut self) {
        let r1 = RegisterType::A;
        let addr = self.registers.read_register(RegisterType::HL);
        self.step_ctx.fetched_data = FetchedData {
            value: self.read_memory(addr),
            source: DataSource::MemoryRegister(RegisterType::HL, addr),
            dest: DataDestination::Register(r1),
        };

        self.registers.set_hl(addr.wrapping_sub(1));
    }

    #[inline(always)]
    pub fn fetch_hli_r_a(&mut self) {
        let r2 = RegisterType::A;
        let addr = self.registers.read_register(RegisterType::HL);
        self.step_ctx.fetched_data = FetchedData {
            value: self.registers.read_register(r2),
            source: DataSource::Register(r2),
            dest: DataDestination::Memory(addr),
        };

        self.registers.set_hl(addr.wrapping_add(1));
    }

    #[inline(always)]
    pub fn fetch_hld_r_a(&mut self) {
        let r2 = RegisterType::A;
        let addr = self.registers.read_register(RegisterType::HL);
        self.step_ctx.fetched_data = FetchedData {
            value: self.registers.read_register(r2),
            source: DataSource::Register(r2),
            dest: DataDestination::Memory(addr),
        };

        self.registers.set_hl(addr.wrapping_sub(1));
    }

    #[inline(always)]
    pub fn fetch_r_ha8_a(&mut self) {
        let r1 = RegisterType::A;
        let addr = self.read_pc() as u16;
        let addr = 0xFF00 | addr;

        self.step_ctx.fetched_data = FetchedData {
            value: self.read_memory(addr),
            source: DataSource::Memory(addr),
            dest: DataDestination::Register(r1),
        }
    }

    #[inline(always)]
    pub fn fetch_a8_r_a(&mut self) {
        let r2 = RegisterType::A;
        let addr = self.read_pc() as u16;

        self.step_ctx.fetched_data = FetchedData {
            value: self.registers.read_register(r2),
            source: DataSource::Register(r2),
            dest: DataDestination::Memory(addr),
        }
    }

    #[inline(always)]
    pub fn fetch_lh_spi8(&mut self) {
        self.step_ctx.fetched_data = FetchedData {
            value: self.read_pc() as u16,
            source: DataSource::Immediate,
            dest: DataDestination::Register(RegisterType::HL),
        }
    }

    #[inline(always)]
    pub fn fetch_d8(&mut self) {
        self.step_ctx.fetched_data = FetchedData {
            dest: DataDestination::Memory(0),
            value: self.read_pc() as u16,
            source: DataSource::Immediate,
        }
    }

    #[inline(always)]
    pub fn fetch_a16_r<const R2: u8>(&mut self) {
        let r2 = RegisterType::from_u8(R2);
        let addr = self.read_pc16();

        self.step_ctx.fetched_data = FetchedData {
            value: self.registers.read_register(r2),
            source: DataSource::Register(r2),
            dest: DataDestination::Memory(addr),
        }
    }

    #[inline(always)]
    pub fn fetch_mr_d8_hl(&mut self) {
        let r1 = RegisterType::HL;

        self.step_ctx.fetched_data = FetchedData {
            value: self.read_pc() as u16,
            source: DataSource::Immediate,
            dest: DataDestination::Memory(self.registers.read_register(r1)),
        }
    }

    #[inline(always)]
    pub fn fetch_mr_hl(&mut self) {
        let r1 = RegisterType::HL;
        let addr = self.registers.read_register(r1);

        self.step_ctx.fetched_data = FetchedData {
            value: self.read_memory(addr),
            source: DataSource::MemoryRegister(r1, addr),
            dest: DataDestination::Memory(addr),
        }
    }

    #[inline(always)]
    pub fn fetch_r_a16_a(&mut self) {
        let r1 = RegisterType::A;
        let addr = self.read_pc16();

        self.step_ctx.fetched_data = FetchedData {
            value: self.read_memory(addr),
            source: DataSource::Memory(addr),
            dest: DataDestination::Register(r1),
        }
    }

    #[inline(always)]
    pub const fn fetch_impl(&mut self) {}

    /// temp method used for fetch_execute instructions
    #[inline(always)]
    pub const fn fetch_d(&mut self) {}
}

/// Represents the different address modes in the CPU's instruction set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum AddressMode {
    /// Implied: The operand is specified in the instruction itself
    ///
    /// Cycles: 0.
    IMP,
    /// Register: Fetches value of register.
    ///
    /// Cycles: 0.
    R(RegisterType),
    /// Register and 16-bit Data: Fetches the 16-bit data by PC.
    ///
    /// Cycles: 2.
    R_D16(RegisterType),
    /// Register to Register: Fetches the data from second register.
    ///
    /// Cycles: 0.
    R_R(RegisterType, RegisterType),
    /// Memory address Register and Register: Fetches the data from second register and memory address from first register.
    ///
    /// Cycles: 0.
    MR_R(RegisterType, RegisterType),
    /// Register and 8-bit data: Fetches value from PC.
    ///
    /// Cycles: 1.
    R_D8(RegisterType),
    /// Register and Memory address Register: Fetches address from second register.
    ///
    /// Cycles: 1.
    R_MR(RegisterType, RegisterType),
    /// Register and High Memory address Register: Fetches address from second register.
    ///
    /// Cycles: 1.
    R_HMR(RegisterType, RegisterType),
    /// Register and HL increment.
    ///
    /// Cycles: 1.
    R_HLI(RegisterType),
    /// Register and HL decrement.
    ///
    /// Cycles: 1.
    R_HLD(RegisterType),
    /// HL increment and Register.
    ///
    /// Cycles: 0.
    HLI_R(RegisterType),
    /// HL decrement and Register.
    ///
    /// Cycles: 0.
    HLD_R(RegisterType),
    /// Register and 8-bit address: Fetches value from 8-bit address.
    ///
    /// Cycles: 1.
    R_A8(RegisterType),
    /// Register and 8-bit high address: Fetches value from 8-bit address.
    ///
    /// Cycles: 1.
    R_HA8(RegisterType),
    /// 8-bit address and Register: Fetches value from second register.
    ///
    /// Cycles: 1.
    A8_R(RegisterType),
    /// HL and SP: HL,(SP+8e): Fetches PC value.
    ///
    /// Cycles: 1.
    LH_SPi8,
    /// 16-bit data: Fetches 16-bit value from memory by PC.
    ///
    /// Cycles: 2.
    D16,
    /// 8-bit data: Fetches 8-bit value from memory by PC.
    ///
    /// Cycles: 1.
    D8,
    /// Memory Address Register and 8-bit data: Fetches 8-bit value from memory by PC and memory address from register.
    ///
    /// Cycles: 1.
    MR_D8(RegisterType),
    /// Memory Address Register: Fetches memory address from register and data by that address.
    ///
    /// Cycles: 1.
    MR(RegisterType),
    /// 16-bit Address and Register: Fetches value from register and memory address by PC.
    ///
    /// Cycles: 2.
    A16_R(RegisterType),
    /// Register and 16-bit Address: Fetches value from register and memory address by PC.
    ///
    /// Cycles: 1.
    R_A16(RegisterType),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FetchedData {
    pub dest: DataDestination,
    pub source: DataSource,
    pub value: u16,
}

impl Default for FetchedData {
    fn default() -> Self {
        Self::empty()
    }
}

impl FetchedData {
    pub const fn empty() -> FetchedData {
        Self {
            dest: DataDestination::Memory(0),
            source: DataSource::Immediate,
            value: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DataDestination {
    Register(RegisterType),
    Memory(u16),
}

impl DataDestination {
    pub fn get_addr(self) -> Option<u16> {
        match self {
            DataDestination::Register(_) => None,
            DataDestination::Memory(addr) => Some(addr),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DataSource {
    MemoryRegister(RegisterType, u16),
    Register(RegisterType),
    Memory(u16),
    Immediate,
}

impl DataSource {
    pub fn get_addr(self) -> Option<u16> {
        match self {
            DataSource::MemoryRegister(_, addr) => Some(addr),
            DataSource::Register(_) => None,
            DataSource::Memory(addr) => Some(addr),
            DataSource::Immediate => None,
        }
    }

    pub fn get_register(self) -> Option<RegisterType> {
        match self {
            DataSource::MemoryRegister(r, _) => Some(r),
            DataSource::Register(r) => Some(r),
            DataSource::Memory(_) => None,
            DataSource::Immediate => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::auxiliary::clock::Clock;
    use crate::auxiliary::io::Io;
    use crate::bus::Bus;
    use crate::cart::Cart;
    use crate::cpu::{Cpu, RegisterType};
    use crate::ppu::Ppu;

    #[test]
    fn test_fetch_r() {
        let cart = Cart::new(vec![0u8; 1000].into_boxed_slice()).unwrap();
        let clock = Clock::new(Ppu::default(), Bus::new(cart, Io::default()));
        let mut cpu = Cpu::new(clock);
        const REG_TYPE: RegisterType = RegisterType::B;
        cpu.registers.set_register(REG_TYPE, 23);

        cpu.fetch_r::<{ REG_TYPE as u8 }>();

        assert_eq!(cpu.step_ctx.fetched_data.value, cpu.registers.read_register(REG_TYPE));
        assert_eq!(cpu.step_ctx.fetched_data.dest.get_addr(), None);
    }

    #[test]
    fn test_fetch_r_r() {
        let cart = Cart::new(vec![0u8; 1000].into_boxed_slice()).unwrap();
        let clock = Clock::new(Ppu::default(), Bus::new(cart, Io::default()));
        let mut cpu = Cpu::new(clock);
        const R1: RegisterType = RegisterType::BC;
        const R2: RegisterType = RegisterType::A;
        cpu.registers.set_register(R2, 23);

        cpu.fetch_r_r::<{ R1 as u8 }, { R2 as u8 }>();

        assert_eq!(cpu.step_ctx.fetched_data.value, cpu.registers.read_register(R2));
        assert_eq!(cpu.step_ctx.fetched_data.dest.get_addr(), None);
    }

    #[test]
    fn test_fetch_r_d8() {
        let pc = 4;
        let value = 25;
        let mut bytes = vec![0u8; 8000].into_boxed_slice();
        bytes[pc] = value;
        let cart = Cart::new(bytes).unwrap();
        let clock = Clock::new(Ppu::default(), Bus::new(cart, Io::default()));
        let mut cpu = Cpu::new(clock);
        cpu.registers.pc = pc as u16;
        const REG_TYPE: RegisterType = RegisterType::A;

        cpu.fetch_r_d8::<{ REG_TYPE as u8 }>();

        assert_eq!(cpu.step_ctx.fetched_data.value as u8, value);
        assert_eq!(cpu.step_ctx.fetched_data.dest.get_addr(), None);
        assert_eq!(cpu.registers.pc, pc as u16 + 1);
        //assert_eq!(self.clock.t_cycles, 4);
    }

    #[test]
    fn test_r_hli() {
        let mut bytes = vec![0u8; 40000];
        let h_val = 0x40;
        let l_val = 0x00;
        let hl_val = u16::from_le_bytes([l_val, h_val]);
        let addr_value = 123;
        bytes[hl_val as usize] = addr_value;

        let clock = Clock::new(Ppu::default(), Bus::with_bytes(bytes, Io::default()));
        let mut cpu = Cpu::new(clock);
        cpu.registers.h = h_val;
        cpu.registers.l = l_val;

        cpu.fetch_r_hli_a();

        assert_eq!(cpu.step_ctx.fetched_data.value, addr_value as u16);
        assert_eq!(cpu.step_ctx.fetched_data.dest.get_addr(), None);
        assert_eq!(cpu.registers.get_hl(), hl_val.wrapping_add(1));
    }

    #[test]
    fn test_r_hld() {
        let mut bytes = vec![0u8; 40000];
        let h_val = 0x40;
        let l_val = 0x00;
        let hl_val = u16::from_le_bytes([l_val, h_val]);
        let addr_value = 123;
        bytes[hl_val as usize] = addr_value;
        let cart = Cart::new(bytes.into_boxed_slice()).unwrap();
        let clock = Clock::new(Ppu::default(), Bus::new(cart, Io::default()));
        let mut cpu = Cpu::new(clock);
        cpu.registers.h = h_val;
        cpu.registers.l = l_val;

        cpu.fetch_r_hld_a();

        assert_eq!(cpu.step_ctx.fetched_data.value, addr_value as u16);
        assert_eq!(cpu.step_ctx.fetched_data.dest.get_addr(), None);
        assert_eq!(cpu.registers.get_hl(), hl_val.wrapping_sub(1));
    }
}
