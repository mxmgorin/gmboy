pub mod fetch;
mod arithmetic;
mod bitwise;
pub mod condition;
pub mod instruction;
mod interrupt;
mod jump;
mod load;
mod misc;
pub mod opcodes;
mod rotate;
pub use fetch::*;
pub use condition::*;
pub use instruction::*;
pub use opcodes::*;


#[cfg(test)]
mod tests {
    use crate::auxiliary::clock::Clock;
    use crate::auxiliary::io::Io;
    use crate::bus::Bus;
    use crate::cpu::instructions::{
        AddressMode
    };
    use crate::cpu::{Cpu, RegisterType};
    use crate::cpu::instructions::condition::ConditionType;
    use crate::cpu::instructions::instruction::Mnemonic;
    use crate::cpu::instructions::opcodes::INSTRUCTIONS_BY_OPCODES;
    use crate::ppu::Ppu;

    const M_CYCLES_BY_OPCODES: [usize; 0x100] = [
        1, 3, 2, 2, 1, 1, 2, 1, 5, 2, 2, 2, 1, 1, 2, 1, 0, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1,
        2, 1, 2, 3, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 2, 1, 2, 3, 2, 2, 3, 3, 3, 1, 2, 2, 2, 2,
        1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1,
        1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 2, 2, 2, 2, 0, 2,
        1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1,
        2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1,
        1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 3, 3, 4, 3, 4, 2, 4, 2, 4, 3, 0, 3, 6, 2, 4, 2, 3,
        3, 0, 3, 4, 2, 4, 2, 4, 3, 0, 3, 0, 2, 4, 3, 3, 2, 0, 0, 4, 2, 4, 4, 1, 4, 0, 0, 0, 2, 4,
        3, 3, 2, 1, 0, 4, 2, 4, 3, 2, 4, 1, 0, 0, 2, 4,
    ];

    #[test]
    pub fn test_m_cycles_ldh_f0() {
        let opcode = 0xF0;
        let clock = Clock::new(
            Ppu::default(),
            Bus::with_bytes(vec![0; 100000], Io::default()),
        );
        let mut cpu = Cpu::new(clock);
        cpu.registers.pc = 0;
        cpu.clock.bus.write(0, opcode as u8);
        cpu.step(None);

        assert_eq!(M_CYCLES_BY_OPCODES[opcode], cpu.clock.get_m_cycles());
    }

    #[test]
    pub fn test_m_cycles_call() {
        let clock = Clock::new(
            Ppu::default(),
            Bus::with_bytes(vec![0; 100000], Io::default()),
        );
        let mut cpu = Cpu::new(clock);
        for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
            if Mnemonic::Call != instr.get_mnemonic() {
                continue;
            }

            cpu.registers.pc = 0;
            cpu.clock.reset();
            cpu.clock.bus.write(0, opcode as u8);

            if let Some(condition_type) = instr.get_condition() {
                assert_for_condition(&mut cpu, condition_type, 6, M_CYCLES_BY_OPCODES[opcode]);
            } else {
                cpu.step(None);
                // 6
                assert_eq!(M_CYCLES_BY_OPCODES[opcode], cpu.clock.get_m_cycles());
            };
        }
    }

    #[test]
    pub fn test_m_cycles_jp() {
        let clock = Clock::new(
            Ppu::default(),
            Bus::with_bytes(vec![0; 100000], Io::default()),
        );
        let mut cpu = Cpu::new(clock);
        for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
            if Mnemonic::Jp != instr.get_mnemonic() {
                continue;
            }

            cpu.registers.pc = 0;
            cpu.clock.reset();
            cpu.clock.bus.write(0, opcode as u8);

            if let Some(condition_type) = instr.get_condition() {
                assert_for_condition(&mut cpu, condition_type, 4, M_CYCLES_BY_OPCODES[opcode]);
            } else if instr.get_address_mode() == AddressMode::D16 {
                cpu.step(None);
                // 4
                assert_eq!(M_CYCLES_BY_OPCODES[opcode], cpu.clock.get_m_cycles());
            } else if instr.get_address_mode() == AddressMode::R(RegisterType::HL) {
                cpu.step(None);
                // 1
                assert_eq!(M_CYCLES_BY_OPCODES[opcode], cpu.clock.get_m_cycles());
            };
        }
    }

    #[test]
    pub fn test_m_cycles_jr() {
        let clock = Clock::new(
            Ppu::default(),
            Bus::with_bytes(vec![0; 100000], Io::default()),
        );
        let mut cpu = Cpu::new(clock);
        for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
            if Mnemonic::Jr != instr.get_mnemonic() {
                continue;
            }

            cpu.registers.pc = 0;
            cpu.clock.reset();
            cpu.clock.bus.write(0, opcode as u8);

            if let Some(condition_type) = instr.get_condition() {
                assert_for_condition(&mut cpu, condition_type, 3, 2);
            } else {
                cpu.step(None);
                // 3
                assert_eq!(M_CYCLES_BY_OPCODES[opcode], cpu.clock.get_m_cycles());
            };
        }
    }

    #[test]
    pub fn test_m_cycles_ret() {
        let clock = Clock::new(
            Ppu::default(),
            Bus::with_bytes(vec![0; 100000], Io::default()),
        );
        let mut cpu = Cpu::new(clock);
        for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
            if Mnemonic::Ret != instr.get_mnemonic() {
                continue;
            }

            cpu.registers.pc = 0;
            cpu.clock.reset();
            cpu.clock.bus.write(0, opcode as u8);

            if let Some(condition_type) = instr.get_condition() {
                assert_for_condition(&mut cpu, condition_type, 5, 2);
            } else {
                cpu.step(None);
                // 4
                assert_eq!(M_CYCLES_BY_OPCODES[opcode], cpu.clock.get_m_cycles());
            };
        }
    }

    #[test]
    pub fn test_m_cycles() {
        let clock = Clock::new(
            Ppu::default(),
            Bus::with_bytes(vec![0; 100000], Io::default()),
        );
        let mut cpu = Cpu::new(clock);
        for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
            match instr.get_mnemonic() {
                Mnemonic::Jp // has tests
                | Mnemonic::Jr // has tests
                | Mnemonic::Ret // has tests
                | Mnemonic::Call // has tests
                | Mnemonic::Stop // has 0 in matrix, invalid?
                | Mnemonic::Halt // has 0 in matrix, invalid? 
                | Mnemonic::Unknown => continue,
                _ => {}
            }

            if 0xCB == opcode {
                continue; // todo: investigate
            }

            cpu.registers.pc = 0;
            cpu.clock.reset();
            cpu.clock.bus.write(0, opcode as u8);
            cpu.step(None);
            let expected = M_CYCLES_BY_OPCODES[opcode];
            let actual = cpu.clock.get_m_cycles();

            if actual != expected {
                let msg = format!(
                    "Invalid M-Cycles for 0x{opcode:02X}: actual={actual}, expected={expected}"
                );
                panic!("{}", msg);
            }
        }
    }

    pub fn assert_for_condition(
        cpu: &mut Cpu,
        condition_type: ConditionType,
        m_cycles_set: usize,
        m_cycles_not: usize,
    ) {
        match condition_type {
            ConditionType::NC => {
                cpu.registers.flags.set_c(false);
                cpu.step(None);
                assert_eq!(m_cycles_set, cpu.clock.get_m_cycles());

                cpu.registers.pc = 0;
                cpu.clock.reset();

                cpu.registers.flags.set_c(true);
                cpu.step(None);
                assert_eq!(m_cycles_not, cpu.clock.get_m_cycles());
            }
            ConditionType::C => {
                cpu.registers.flags.set_c(false);
                cpu.step(None);
                assert_eq!(m_cycles_not, cpu.clock.get_m_cycles());

                cpu.registers.pc = 0;
                cpu.clock.reset();

                cpu.registers.flags.set_c(true);
                cpu.step(None);
                assert_eq!(m_cycles_set, cpu.clock.get_m_cycles());
            }
            ConditionType::NZ => {
                cpu.registers.flags.set_z(false);
                cpu.step(None);
                assert_eq!(m_cycles_set, cpu.clock.get_m_cycles());

                cpu.registers.pc = 0;
                cpu.clock.reset();

                cpu.registers.flags.set_z(true);
                cpu.step(None);
                assert_eq!(m_cycles_not, cpu.clock.get_m_cycles());
            }
            ConditionType::Z => {
                cpu.registers.flags.set_z(false);
                cpu.step(None);
                assert_eq!(m_cycles_not, cpu.clock.get_m_cycles());

                cpu.registers.pc = 0;
                cpu.clock.reset();

                cpu.registers.flags.set_z(true);
                cpu.step(None);
                assert_eq!(m_cycles_set, cpu.clock.get_m_cycles());
            }
        }
    }
}
