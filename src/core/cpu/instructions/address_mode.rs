use crate::core::cpu::instructions::instruction::RegisterType;
use crate::core::cpu::Cpu;

#[derive(Debug, Clone, Default)]
pub struct FetchedData {
    pub value: u16,
    pub dest_addr: Option<u16>,
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
    /// Register and 8-bit data.
    /// Fetches value from PC.
    ///
    /// Cycles: 1.
    R_D8(RegisterType),
    /// Register and Memory address Register: Fetches address from second register.
    ///
    /// Cycles: 1.
    R_MR(RegisterType, RegisterType),
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
    /// 8-bit address and Register: Fetches value from second register.
    ///
    /// Cycles: 1.
    A8_R(RegisterType),
    /// HL and SP: HL,(SP+8e): Fetches PC value.
    ///
    /// Cycles: 1.
    HL_SPe8,
    /// 16-bit data: Fetches 16-bit value from memory by PC.
    ///
    /// Cycles: 2.
    D16,
    /// 8-bit data: Fetches 8-bit value from memory by PC.
    ///
    /// Cycles: 1.
    D8,
    /// 16-bit data to Register.
    ///
    /// Cycles: 2.
    D16_R(RegisterType),
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

impl AddressMode {
    pub fn fetch_data(cpu: &mut Cpu, address_mode: AddressMode) -> FetchedData {
        let mut fetched_data = FetchedData::default();

        match address_mode {
            AddressMode::IMP => (),
            AddressMode::R(r1) => {
                fetched_data.value = cpu.registers.read_register(r1);
            }
            AddressMode::R_R(_r1, r2) => {
                fetched_data.value = cpu.registers.read_register(r2);
            }
            AddressMode::R_D8(_r1) => {
                fetched_data.value = cpu.fetch_data() as u16;
            }
            AddressMode::D16 | AddressMode::R_D16(_) => {
                fetched_data.value = cpu.fetch_data16();
            }
            AddressMode::R_MR(_r1, r2) => {
                let mut addr = cpu.registers.read_register(r2);

                if r2 == RegisterType::C {
                    addr |= 0xFF00;
                }

                fetched_data.value = cpu.bus.read(addr) as u16;
                cpu.update_cycles(1);
            }
            AddressMode::MR_R(r1, r2) => {
                fetched_data.value = cpu.registers.read_register(r2);
                let mut addr = cpu.registers.read_register(r1);

                if r1 == RegisterType::C {
                    addr |= 0xFF00;
                }

                fetched_data.dest_addr = Some(addr);
            }
            AddressMode::R_HLI(_r1) => {
                let hl_val = cpu.registers.read_register(RegisterType::HL);
                fetched_data.value = cpu.bus.read(hl_val) as u16;
                cpu.update_cycles(1);
                cpu.registers.set_hl(hl_val.wrapping_add(1));
            }
            AddressMode::R_HLD(_r1) => {
                let hl = RegisterType::HL;
                let hl_val = cpu.registers.read_register(hl);
                fetched_data.value = cpu.bus.read(hl_val) as u16;
                cpu.update_cycles(1);
                cpu.registers.set_hl(hl_val.wrapping_sub(1));
            }
            AddressMode::HLI_R(r2) => {
                fetched_data.value = cpu.registers.read_register(r2);

                let hl_val = cpu.registers.read_register(RegisterType::HL);
                fetched_data.dest_addr = Some(hl_val);
                cpu.registers.set_hl(hl_val.wrapping_add(1));
            }
            AddressMode::HLD_R(r2) => {
                fetched_data.value = cpu.registers.read_register(r2);

                let hl_val = cpu.registers.read_register(RegisterType::HL);
                fetched_data.dest_addr = Some(hl_val);
                cpu.registers.set_hl(hl_val.wrapping_sub(1));
            }
            AddressMode::R_A8(_r1) => {
                fetched_data.value = cpu.fetch_data() as u16;
            }
            AddressMode::A8_R(r2) => {
                let value = cpu.fetch_data() as u16;
                fetched_data.dest_addr = Some(value | 0xFF00);
                fetched_data.value = cpu.registers.read_register(r2);
            }
            AddressMode::HL_SPe8 => {
                fetched_data.value = cpu.fetch_data() as u16;
            }
            AddressMode::D8 => {
                fetched_data.value = cpu.fetch_data() as u16;
            }
            AddressMode::D16_R(r2) | AddressMode::A16_R(r2) => {
                let addr = cpu.fetch_data16();
                fetched_data.dest_addr = Some(addr);
                fetched_data.value = cpu.registers.read_register(r2);
            }
            AddressMode::MR_D8(r1) => {
                fetched_data.value = cpu.fetch_data() as u16;
                fetched_data.dest_addr = Some(cpu.registers.read_register(r1));
            }
            AddressMode::MR(r1) => {
                let r_addr = cpu.registers.read_register(r1);
                fetched_data.dest_addr = Some(r_addr);
                fetched_data.value = cpu.bus.read(r_addr) as u16;
                cpu.update_cycles(1);
            }
            AddressMode::R_A16(_r1) => {
                let addr = cpu.fetch_data16();
                fetched_data.value = cpu.bus.read(addr) as u16;
                cpu.update_cycles(1);
            }
        }

        fetched_data
    }
}

#[cfg(test)]
mod tests {
    use crate::cart::Cart;
    use crate::core::bus::ram::Ram;
    use crate::core::bus::Bus;
    use crate::cpu::instructions::{AddressMode, RegisterType};
    use crate::cpu::Cpu;
    use crate::util::LittleEndianBytes;

    #[test]
    fn test_fetch_imp() {
        let cart = Cart::new(vec![0u8; 1000]).unwrap();
        let mut cpu = Cpu::new(Bus::new(cart, Ram::new()));
        let mode = AddressMode::IMP;

        let data = AddressMode::fetch_data(&mut cpu, mode);

        assert_eq!(data.value, 0);
        assert_eq!(data.dest_addr, None);
    }

    #[test]
    fn test_fetch_r() {
        let cart = Cart::new(vec![0u8; 1000]).unwrap();
        let mut cpu = Cpu::new(Bus::new(cart, Ram::new()));

        for reg_type in RegisterType::get_all().iter().cloned() {
            cpu.registers.set_register(reg_type, 23);
            let mode = AddressMode::R(reg_type);

            let data = AddressMode::fetch_data(&mut cpu, mode);

            assert_eq!(data.value, cpu.registers.read_register(reg_type));
            assert_eq!(data.dest_addr, None);
        }
    }

    #[test]
    fn test_fetch_r_r() {
        let cart = Cart::new(vec![0u8; 1000]).unwrap();
        let mut cpu = Cpu::new(Bus::new(cart, Ram::new()));

        for reg_type in RegisterType::get_all().iter().cloned() {
            cpu.registers.set_register(reg_type, 23);
            let mode = AddressMode::R_R(RegisterType::BC, reg_type);

            let data = AddressMode::fetch_data(&mut cpu, mode);

            assert_eq!(data.value, cpu.registers.read_register(reg_type));
            assert_eq!(data.dest_addr, None);
        }
    }

    #[test]
    fn test_fetch_r_d8() {
        let pc = 4;
        let value = 25;
        let mut bytes = vec![0u8; 8000];
        bytes[pc] = value;
        let cart = Cart::new(bytes).unwrap();
        let mut cpu = Cpu::new(Bus::new(cart, Ram::new()));
        cpu.registers.pc = pc as u16;
        let mode = AddressMode::R_D8(RegisterType::A);

        let data = AddressMode::fetch_data(&mut cpu, mode);

        assert_eq!(data.value as u8, value);
        assert_eq!(data.dest_addr, None);
        assert_eq!(cpu.registers.pc, pc as u16 + 1);
        assert_eq!(cpu.ticks, 4);
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

        let mut cpu = Cpu::new(Bus::new(Cart::new(bytes).unwrap(), Ram::new()));
        cpu.registers.h = h_val;
        cpu.registers.l = l_val;

        let data = AddressMode::fetch_data(&mut cpu, mode);

        assert_eq!(data.value, addr_value as u16);
        assert_eq!(data.dest_addr, None);
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

        let mut cpu = Cpu::new(Bus::new(Cart::new(bytes).unwrap(), Ram::new()));
        cpu.registers.h = h_val;
        cpu.registers.l = l_val;

        let data = AddressMode::fetch_data(&mut cpu, mode);

        assert_eq!(data.value, addr_value as u16);
        assert_eq!(data.dest_addr, None);
        assert_eq!(cpu.registers.get_hl(), hl_val.wrapping_sub(1));
    }
}
