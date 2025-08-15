use crate::cpu::instructions::instruction::RegisterType;
use crate::cpu::Cpu;

impl AddressMode {
    pub fn is_hl_spi8(self) -> bool {
        self == AddressMode::LH_SPi8
    }
}

impl Cpu {
    pub fn fetch_data_mode(&mut self, address_mode: AddressMode) -> FetchedData {
        let mut fetched_data = FetchedData::empty();

        match address_mode {
            AddressMode::IMP => (),
            AddressMode::R(r1) => {
                fetched_data.value = self.registers.read_register(r1);
                fetched_data.source = DataSource::Register(r1);
                fetched_data.dest = DataDestination::Register(r1);
            }
            AddressMode::R_R(r1, r2) => {
                fetched_data.value = self.registers.read_register(r2);
                fetched_data.source = DataSource::Register(r2);
                fetched_data.dest = DataDestination::Register(r1);
            }
            AddressMode::R_D8(r1) => {
                fetched_data.value = self.fetch_data() as u16;
                fetched_data.source = DataSource::Immediate;
                fetched_data.dest = DataDestination::Register(r1);
            }
            AddressMode::D16 => {
                fetched_data.value = self.fetch_data16();
                fetched_data.source = DataSource::Immediate;
            }
            AddressMode::R_D16(r1) => {
                fetched_data.value = self.fetch_data16();
                fetched_data.source = DataSource::Immediate;
                fetched_data.dest = DataDestination::Register(r1);
            }
            AddressMode::R_MR(r1, r2) => {
                let addr = self.registers.read_register(r2);

                fetched_data.value = self.read_memory(addr);
                fetched_data.source = DataSource::MemoryRegister(r2, addr);
                fetched_data.dest = DataDestination::Register(r1);
            }
            AddressMode::R_HMR(r1, r2) => {
                let addr = self.registers.read_register(r2);
                let addr = 0xFF00 | addr;

                fetched_data.value = self.read_memory(addr);
                fetched_data.source = DataSource::MemoryRegister(r2, addr);
                fetched_data.dest = DataDestination::Register(r1);
            }
            AddressMode::MR_R(r1, r2) => {
                fetched_data.value = self.registers.read_register(r2);
                fetched_data.source = DataSource::Register(r2);
                fetched_data.dest = DataDestination::Memory(self.registers.read_register(r1));
            }
            AddressMode::R_HLI(r1) => {
                let addr = self.registers.read_register(RegisterType::HL);

                fetched_data.value = self.read_memory(addr);
                fetched_data.source = DataSource::MemoryRegister(RegisterType::HL, addr);
                fetched_data.dest = DataDestination::Register(r1);

                self.registers.set_hl(addr.wrapping_add(1));
            }
            AddressMode::R_HLD(r1) => {
                let addr = self.registers.read_register(RegisterType::HL);

                fetched_data.value = self.read_memory(addr);
                fetched_data.source = DataSource::MemoryRegister(RegisterType::HL, addr);
                fetched_data.dest = DataDestination::Register(r1);

                self.registers.set_hl(addr.wrapping_sub(1));
            }
            AddressMode::HLI_R(r2) => {
                let addr = self.registers.read_register(RegisterType::HL);

                fetched_data.value = self.registers.read_register(r2);
                fetched_data.source = DataSource::Register(r2);
                fetched_data.dest = DataDestination::Memory(addr);

                self.registers.set_hl(addr.wrapping_add(1));
            }
            AddressMode::HLD_R(r2) => {
                let addr = self.registers.read_register(RegisterType::HL);

                fetched_data.value = self.registers.read_register(r2);
                fetched_data.source = DataSource::Register(r2);
                fetched_data.dest = DataDestination::Memory(addr);

                self.registers.set_hl(addr.wrapping_sub(1));
            }
            AddressMode::R_A8(r1) => {
                let addr = self.fetch_data() as u16;

                fetched_data.value = self.read_memory(addr);
                fetched_data.source = DataSource::Memory(addr);
                fetched_data.dest = DataDestination::Register(r1);
            }
            AddressMode::R_HA8(r1) => {
                let addr = self.fetch_data() as u16;
                let addr = 0xFF00 | addr;

                fetched_data.value = self.read_memory(addr);
                fetched_data.source = DataSource::Memory(addr);
                fetched_data.dest = DataDestination::Register(r1);
            }
            AddressMode::A8_R(r2) => {
                let addr = self.fetch_data() as u16;

                fetched_data.value = self.registers.read_register(r2);
                fetched_data.source = DataSource::Register(r2);
                fetched_data.dest = DataDestination::Memory(addr);
            }
            AddressMode::LH_SPi8 => {
                fetched_data.value = self.fetch_data() as u16;
                fetched_data.source = DataSource::Immediate;
                fetched_data.dest = DataDestination::Register(RegisterType::HL);
            }
            AddressMode::D8 => {
                fetched_data.value = self.fetch_data() as u16;
                fetched_data.source = DataSource::Immediate;
            }
            AddressMode::A16_R(r2) => {
                let addr = self.fetch_data16();

                fetched_data.value = self.registers.read_register(r2);
                fetched_data.source = DataSource::Register(r2);
                fetched_data.dest = DataDestination::Memory(addr);
            }
            AddressMode::MR_D8(r1) => {
                fetched_data.value = self.fetch_data() as u16;
                fetched_data.source = DataSource::Immediate;
                fetched_data.dest = DataDestination::Memory(self.registers.read_register(r1));
            }
            AddressMode::MR(r1) => {
                let addr = self.registers.read_register(r1);

                fetched_data.value = self.read_memory(addr);
                fetched_data.source = DataSource::MemoryRegister(r1, addr);
                fetched_data.dest = DataDestination::Memory(addr);
            }
            AddressMode::R_A16(r1) => {
                let addr = self.fetch_data16();

                fetched_data.value = self.read_memory(addr);
                fetched_data.source = DataSource::Memory(addr);
                fetched_data.dest = DataDestination::Register(r1);
            }
        }

        fetched_data
    }
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

#[derive(Debug, Clone)]
pub struct FetchedData {
    pub dest: DataDestination,
    pub source: DataSource,
    pub value: u16,
}

impl FetchedData {
    pub fn empty() -> FetchedData {
        Self {
            dest: DataDestination::Memory(0),
            source: DataSource::Immediate,
            value: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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
    use crate::auxiliary::clock::{Clock};
    use crate::auxiliary::io::Io;
    use crate::bus::Bus;
    use crate::cart::Cart;
    use crate::cpu::instructions::{AddressMode, RegisterType};
    use crate::cpu::Cpu;
    use crate::LittleEndianBytes;
    use crate::ppu::Ppu;

    #[test]
    fn test_fetch_imp() {
        let cart = Cart::new(vec![0u8; 1000].into_boxed_slice()).unwrap();
        let clock = Clock::new(Ppu::default(), Bus::new(cart, Io::default()));
        let mut cpu = Cpu::new(clock);
        let mode = AddressMode::IMP;

        let data = cpu.fetch_data_mode(mode);

        assert_eq!(data.value, 0);
    }

    #[test]
    fn test_fetch_r() {
        let cart = Cart::new(vec![0u8; 1000].into_boxed_slice()).unwrap();
        let clock = Clock::new(Ppu::default(), Bus::new(cart, Io::default()));
        let mut cpu = Cpu::new(clock);
        for reg_type in RegisterType::get_all().iter().cloned() {
            cpu.registers.set_register(reg_type, 23);
            let mode = AddressMode::R(reg_type);

            let data = cpu.fetch_data_mode(mode);

            assert_eq!(data.value, cpu.registers.read_register(reg_type));
            assert_eq!(data.dest.get_addr(), None);
        }
    }

    #[test]
    fn test_fetch_r_r() {
        let cart = Cart::new(vec![0u8; 1000].into_boxed_slice()).unwrap();
        let clock = Clock::new(Ppu::default(), Bus::new(cart, Io::default()));
        let mut cpu = Cpu::new(clock);
        for reg_type in RegisterType::get_all().iter().cloned() {
            cpu.registers.set_register(reg_type, 23);
            let mode = AddressMode::R_R(RegisterType::BC, reg_type);

            let data = cpu.fetch_data_mode(mode);

            assert_eq!(data.value, cpu.registers.read_register(reg_type));
            assert_eq!(data.dest.get_addr(), None);
        }
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
        let mode = AddressMode::R_D8(RegisterType::A);

        let data = cpu.fetch_data_mode(mode);

        assert_eq!(data.value as u8, value);
        assert_eq!(data.dest.get_addr(), None);
        assert_eq!(cpu.registers.pc, pc as u16 + 1);
        //assert_eq!(self.clock.t_cycles, 4);
    }

    #[test]
    fn test_r_hli() {
        let mode = AddressMode::R_HLI(RegisterType::A);
        let mut bytes = vec![0u8; 40000];
        let h_val = 0x40;
        let l_val = 0x00;
        let hl_val: u16 = LittleEndianBytes {
            low_byte: l_val,
            high_byte: h_val,
        }
        .into();
        let addr_value = 123;
        bytes[hl_val as usize] = addr_value;

        let clock = Clock::new(Ppu::default(), Bus::with_bytes(bytes, Io::default()));
        let mut cpu = Cpu::new(clock);
        cpu.registers.h = h_val;
        cpu.registers.l = l_val;

        let data = cpu.fetch_data_mode(mode);

        assert_eq!(data.value, addr_value as u16);
        assert_eq!(data.dest.get_addr(), None);
        assert_eq!(cpu.registers.get_hl(), hl_val.wrapping_add(1));
    }

    #[test]
    fn test_r_hld() {
        let mode = AddressMode::R_HLD(RegisterType::A);
        let mut bytes = vec![0u8; 40000];
        let h_val = 0x40;
        let l_val = 0x00;
        let hl_val: u16 = LittleEndianBytes {
            low_byte: l_val,
            high_byte: h_val,
        }
        .into();
        let addr_value = 123;
        bytes[hl_val as usize] = addr_value;
        let cart = Cart::new(bytes.into_boxed_slice()).unwrap();
        let clock = Clock::new(Ppu::default(), Bus::new(cart, Io::default()));
        let mut cpu = Cpu::new(clock);
        cpu.registers.h = h_val;
        cpu.registers.l = l_val;

        let data = cpu.fetch_data_mode(mode);

        assert_eq!(data.value, addr_value as u16);
        assert_eq!(data.dest.get_addr(), None);
        assert_eq!(cpu.registers.get_hl(), hl_val.wrapping_sub(1));
    }
}
