use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction};
use crate::core::cpu::Cpu;
use crate::cpu::instructions::common::FetchedData;

#[derive(Debug, Clone, Copy)]
pub struct XorInstruction {
    pub address_mode: AddressMode,
}

impl ExecutableInstruction for XorInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        cpu.registers.a ^= (fetched_data.value & 0xFF) as u8;
        cpu.registers.set_flags(
            ((cpu.registers.a == 0) as i8).into(),
            0.into(),
            0.into(),
            0.into(),
        );
    }

    fn get_address_mode(&self) -> AddressMode {
        self.address_mode
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cart::Cart;
    use crate::core::bus::ram::Ram;
    use crate::core::bus::Bus;
    use crate::cpu::Cpu;

    #[test]
    fn test_xor_zero_result() {
        let mut cpu = Cpu::new(Bus::new(Cart::new(vec![0u8; 1000]).unwrap(), Ram::new()));
        cpu.registers.a = 0b10101010;

        let fetched_data = FetchedData {
            value: 0b10101010,
            dest_addr: None,
        };

        let instruction = XorInstruction {
            address_mode: AddressMode::IMP,
        };

        instruction.execute(&mut cpu, fetched_data);

        assert_eq!(cpu.registers.a, 0);
        assert!(cpu.registers.get_flag_z()); // Zero flag should be set
        assert!(!cpu.registers.get_flag_n()); // N flag should be cleared
        assert!(!cpu.registers.get_flag_h()); // H flag should be cleared
        assert!(!cpu.registers.get_flag_c()); // C flag should be cleared
    }

    #[test]
    fn test_xor_non_zero_result() {
        let mut cpu = Cpu::new(Bus::new(Cart::new(vec![0u8; 1000]).unwrap(), Ram::new()));
        cpu.registers.a = 0b10101010; // Initial value in the A register

        let fetched_data = FetchedData {
            value: 0b01010101,
            dest_addr: None,
        }; // XOR with a different value

        let instruction = XorInstruction {
            address_mode: AddressMode::IMP, // Implied mode
        };

        instruction.execute(&mut cpu, fetched_data);

        assert_eq!(cpu.registers.a, 0b11111111); // Result should be 0xFF
        assert!(!cpu.registers.get_flag_z()); // Zero flag should be cleared
        assert!(!cpu.registers.get_flag_n()); // N flag should be cleared
        assert!(!cpu.registers.get_flag_h()); // H flag should be cleared
        assert!(!cpu.registers.get_flag_c()); // C flag should be cleared
    }

    #[test]
    fn test_xor_with_zero() {
        let mut cpu = Cpu::new(Bus::new(Cart::new(vec![0u8; 1000]).unwrap(), Ram::new()));
        cpu.registers.a = 0b10101010; // Initial value in the A register

        let fetched_data = FetchedData {
            value: 0,
            dest_addr: None,
        }; // XOR with 0

        let instruction = XorInstruction {
            address_mode: AddressMode::IMP, // Implied mode
        };

        instruction.execute(&mut cpu, fetched_data);

        assert_eq!(cpu.registers.a, 0b10101010); // Result should remain unchanged
        assert!(!cpu.registers.get_flag_z()); // Zero flag should be cleared
        assert!(!cpu.registers.get_flag_n()); // N flag should be cleared
        assert!(!cpu.registers.get_flag_h()); // H flag should be cleared
        assert!(!cpu.registers.get_flag_c()); // C flag should be cleared
    }

    #[test]
    fn test_xor_with_maximum_value() {
        let mut cpu = Cpu::new(Bus::new(Cart::new(vec![0u8; 1000]).unwrap(), Ram::new()));
        cpu.registers.a = 0xFF; // Initial value in the A register

        let fetched_data = FetchedData {
            value: 0xFF,
            dest_addr: None,
        }; // XOR with maximum value (255)

        let instruction = XorInstruction {
            address_mode: AddressMode::IMP, // Implied mode
        };

        instruction.execute(&mut cpu, fetched_data);

        assert_eq!(cpu.registers.a, 0); // Result should be 0
        assert!(cpu.registers.get_flag_z()); // Zero flag should be set
        assert!(!cpu.registers.get_flag_n()); // N flag should be cleared
        assert!(!cpu.registers.get_flag_h()); // H flag should be cleared
        assert!(!cpu.registers.get_flag_c()); // C flag should be cleared
    }

    #[test]
    fn test_xor_with_minimum_value() {
        let mut cpu = Cpu::new(Bus::new(Cart::new(vec![0u8; 1000]).unwrap(), Ram::new()));
        cpu.registers.a = 0x00; // Initial value in the A register

        let fetched_data = FetchedData {
            value: 0xFF,
            dest_addr: None,
        }; // XOR with maximum value (255)

        let instruction = XorInstruction {
            address_mode: AddressMode::IMP, // Implied mode
        };

        instruction.execute(&mut cpu, fetched_data);

        assert_eq!(cpu.registers.a, 0xFF); // Result should be maximum value
        assert!(!cpu.registers.get_flag_z()); // Zero flag should be cleared
        assert!(!cpu.registers.get_flag_n()); // N flag should be cleared
        assert!(!cpu.registers.get_flag_h()); // H flag should be cleared
        assert!(!cpu.registers.get_flag_c()); // C flag should be cleared
    }

    #[test]
    fn test_xor_with_random_values() {
        let mut cpu = Cpu::new(Bus::new(Cart::new(vec![0u8; 1000]).unwrap(), Ram::new()));
        cpu.registers.a = 0b11001010; // Initial value in the A register

        let fetched_data = FetchedData {
            value: 0b10110101,
            dest_addr: None,
        }; // Random value

        let instruction = XorInstruction {
            address_mode: AddressMode::IMP, // Implied mode
        };

        instruction.execute(&mut cpu, fetched_data);

        assert_eq!(cpu.registers.a, 0b01111111); // Expected XOR result
        assert!(!cpu.registers.get_flag_z()); // Zero flag should be cleared
        assert!(!cpu.registers.get_flag_n()); // N flag should be cleared
        assert!(!cpu.registers.get_flag_h()); // H flag should be cleared
        assert!(!cpu.registers.get_flag_c()); // C flag should be cleared
    }

    #[test]
    fn test_xor_zero_with_non_zero_value() {
        let mut cpu = Cpu::new(Bus::new(Cart::new(vec![0u8; 1000]).unwrap(), Ram::new()));
        cpu.registers.a = 0; // Initial value in the A register

        let fetched_data = FetchedData {
            value: 0b11110000,
            dest_addr: None,
        }; // XOR with non-zero value

        let instruction = XorInstruction {
            address_mode: AddressMode::IMP, // Implied mode
        };

        instruction.execute(&mut cpu, fetched_data);

        assert_eq!(cpu.registers.a, 0b11110000); // Result should be the fetched value
        assert!(!cpu.registers.get_flag_z()); // Zero flag should be cleared
        assert!(!cpu.registers.get_flag_n()); // N flag should be cleared
        assert!(!cpu.registers.get_flag_h()); // H flag should be cleared
        assert!(!cpu.registers.get_flag_c()); // C flag should be cleared
    }

    #[test]
    fn test_xor_flags_unaffected_by_value_bits() {
        let mut cpu = Cpu::new(Bus::new(Cart::new(vec![0u8; 1000]).unwrap(), Ram::new()));
        cpu.registers.a = 0b11111111; // Initial value in the A register

        let fetched_data = FetchedData {
            value: 0b00000000,
            dest_addr: None,
        }; // XOR with zero

        let instruction = XorInstruction {
            address_mode: AddressMode::IMP, // Implied mode
        };

        instruction.execute(&mut cpu, fetched_data);

        assert_eq!(cpu.registers.a, 0b11111111); // Result should remain unchanged
        assert!(!cpu.registers.get_flag_z()); // Zero flag should be cleared
        assert!(!cpu.registers.get_flag_n()); // N flag should be cleared
        assert!(!cpu.registers.get_flag_h()); // H flag should be cleared
        assert!(!cpu.registers.get_flag_c()); // C flag should be cleared
    }
}
