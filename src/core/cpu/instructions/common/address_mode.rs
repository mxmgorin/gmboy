use crate::core::cpu::instructions::common::instruction::RegisterType;
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
    IMP,
    /// Register
    /// Fetches value of register
    R(RegisterType),
    /// Register and 16-bit Data
    /// Fetches the 16-bit data by PC
    R_D16(RegisterType),
    /// Register to Register
    /// Fetches the data from second register
    R_R(RegisterType, RegisterType),
    /// Memory address Register and Register
    /// Fetches the data from second register and memory address from first register
    MR_R(RegisterType, RegisterType),
    /// Register and 8-bit data
    /// Fetches value from PC
    R_D8(RegisterType),
    /// Register and Memory address Register
    /// Fetches address from second register
    R_MR(RegisterType, RegisterType),
    /// Register and HL increment: The instruction uses the `HL` register pair, increments it,
    /// and accesses memory using the updated value of `HL`.
    R_HLI(RegisterType, RegisterType),
    /// Register and HL decrement: The instruction uses the `HL` register pair, decrements it,
    /// and accesses memory using the updated value of `HL`.
    R_HLD(RegisterType, RegisterType),
    /// HL increment and Register: The instruction stores a value from a register to memory and
    /// increments the `HL` register pair.
    HLI_R(RegisterType, RegisterType),
    /// HL decrement and Register: The instruction stores a value from a register to memory and
    /// decrements the `HL` register pair.
    HLD_R(RegisterType, RegisterType),
    /// Register and 8-bit address
    /// Fetches value from 8-bit address
    R_A8(RegisterType),
    /// 8-bit address and Register
    /// Fetches value from second register
    A8_R(RegisterType),
    /// Special case for LD Hl,(SP+8e)
    /// Fetches PC value
    HL_SPR(RegisterType, RegisterType),
    /// 16-bit data
    /// Fetches 16-bit value from memory by PC
    D16,
    /// 8-bit data
    /// Fetches 8-bit value from memory by PC
    D8,
    /// 16-bit data to Register
    D16_R(RegisterType),
    /// Memory Address Register and 8-bit data
    /// Fetches 8-bit value from memory by PC and memory address from register
    MR_D8(RegisterType),
    /// Memory Address Register
    /// Fetches memory address from register and data by that address
    MR(RegisterType),
    /// 16-bit Address and Register
    /// Fetches value from register and memory address by PC
    A16_R(RegisterType),
    /// Register and 16-bit Address
    /// Fetches value from register and memory address by PC
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
                fetched_data.value = cpu.bus.read(cpu.registers.pc) as u16;
                cpu.update_cycles(1);
                cpu.registers.pc += 1;
            }
            AddressMode::D16 | AddressMode::R_D16(_) => {
                let lo = cpu.bus.read(cpu.registers.pc);
                cpu.update_cycles(1);
                let hi = cpu.bus.read(cpu.registers.pc + 1);
                cpu.update_cycles(1);
                fetched_data.value = (hi as u16) << 8 | (lo as u16);
                cpu.registers.pc += 2;
            }
            AddressMode::R_MR(_r1, r2) => {
                let mut addr = cpu.registers.read_register(r2);

                if r2 == RegisterType::C {
                    // LDH A,[C]
                    // Copy the byte at address $FF00+C into register A.
                    // This is sometimes written as ‘LD A,[$FF00+C]’.
                    addr |= 0xFF0;
                }

                fetched_data.value = cpu.bus.read(addr) as u16;
                cpu.update_cycles(1);
            }
            AddressMode::MR_R(r1, r2) => {
                fetched_data.value = cpu.registers.read_register(r2);
                let mut addr = cpu.registers.read_register(r1);

                if r1 == RegisterType::C {
                    // LDH [C],A
                    // Copy the value in register A into the byte at address $FF00+C.
                    // This is sometimes written as ‘LD [$FF00+C],A’.
                    addr |= 0xFF00;
                }

                fetched_data.dest_addr = Some(addr);
            }
            AddressMode::R_HLI(_r1, r2) => {
                fetched_data.value = cpu.bus.read(cpu.registers.read_register(r2)) as u16;
                cpu.update_cycles(1);
                cpu.registers.set_register(
                    RegisterType::HL,
                    cpu.registers.read_register(RegisterType::HL) + 1,
                );
            }
            AddressMode::R_HLD(_r1, r2) => {
                fetched_data.value = cpu.bus.read(cpu.registers.read_register(r2)) as u16;
                cpu.update_cycles(1);
                cpu.registers.set_register(
                    RegisterType::HL,
                    cpu.registers
                        .read_register(RegisterType::HL)
                        .wrapping_sub(1),
                );
            }
            AddressMode::HLI_R(r1, r2) => {
                fetched_data.value = cpu.registers.read_register(r2);
                fetched_data.dest_addr = Some(cpu.registers.read_register(r1));
                cpu.registers.set_register(
                    RegisterType::HL,
                    cpu.registers.read_register(RegisterType::HL) + 1,
                );
            }
            AddressMode::HLD_R(r1, r2) => {
                fetched_data.value = cpu.registers.read_register(r2);
                fetched_data.dest_addr = Some(cpu.registers.read_register(r1));
                cpu.registers.set_register(
                    RegisterType::HL,
                    cpu.registers.read_register(RegisterType::HL) - 1,
                );
            }
            AddressMode::R_A8(_r1) => {
                fetched_data.value = cpu.bus.read(cpu.registers.pc) as u16;
                cpu.update_cycles(1);
                cpu.registers.pc += 1;
            }
            AddressMode::A8_R(_r1) => {
                fetched_data.dest_addr = Some(cpu.bus.read(cpu.registers.pc) as u16 | 0xFF00);
                cpu.update_cycles(1);
                cpu.registers.pc += 1;
            }
            AddressMode::HL_SPR(_r1, _r2) => {
                fetched_data.value = cpu.bus.read(cpu.registers.pc) as u16;
                cpu.update_cycles(1);
                cpu.registers.pc += 1;
            }
            AddressMode::D8 => {
                fetched_data.value = cpu.bus.read(cpu.registers.pc) as u16;
                cpu.update_cycles(1);
                cpu.registers.pc += 1;
            }
            AddressMode::D16_R(r1) | AddressMode::A16_R(r1) => {
                let lo = cpu.bus.read(cpu.registers.pc) as u16;
                cpu.update_cycles(1);

                let hi = cpu.bus.read(cpu.registers.pc + 1) as u16;
                cpu.update_cycles(1);

                fetched_data.dest_addr = Some(lo | (hi << 8));
                cpu.registers.pc += 2;
                fetched_data.value = cpu.registers.read_register(r1);
            }
            AddressMode::MR_D8(r1) => {
                fetched_data.value = cpu.bus.read(cpu.registers.pc) as u16;
                cpu.update_cycles(1);
                cpu.registers.pc += 1;
                fetched_data.dest_addr = Some(cpu.registers.read_register(r1));
            }
            AddressMode::MR(r1) => {
                fetched_data.dest_addr = Some(cpu.registers.read_register(r1));
                fetched_data.value = cpu.bus.read(cpu.registers.read_register(r1)) as u16;
            }
            AddressMode::R_A16(_r1) => {
                let lo = cpu.bus.read(cpu.registers.pc) as u16;
                cpu.update_cycles(1);

                let hi = cpu.bus.read(cpu.registers.pc + 1) as u16;
                cpu.update_cycles(1);

                let addr = lo | (hi << 8);
                cpu.registers.pc += 2;
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
    use crate::cpu::instructions::common::{AddressMode, RegisterType};
    use crate::cpu::Cpu;

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

            assert_eq!(data.value as u8, cpu.registers.a);
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

            assert_eq!(data.value as u8, cpu.registers.a);
            assert_eq!(data.dest_addr, None);
        }
    }

    #[test]
    fn test_fetch_r_d8() {
        let pc = 4;
        let value = 25;
        let mut bytes = vec![0u8; 1000];
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
}
