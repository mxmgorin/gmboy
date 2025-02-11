pub mod address_mode;
mod arithmetic;
mod bitwise;
pub mod condition_type;
pub mod instruction;
mod interrupt;
mod jump;
mod load;
mod misc;
pub mod opcodes;
mod rotate;

pub use address_mode::*;
pub use arithmetic::dec::*;
pub use arithmetic::inc::*;
pub use bitwise::cpl::*;
pub use bitwise::or::*;
pub use bitwise::xor::*;
pub use condition_type::*;
pub use instruction::*;
pub use interrupt::di::*;
pub use interrupt::ei::*;
pub use interrupt::halt::*;
pub use jump::call::*;
pub use jump::jp::*;
pub use jump::jr::*;
pub use jump::ret::*;
pub use jump::reti::*;
pub use load::ld::*;
pub use load::ldh::*;
pub use misc::ccf::*;
pub use misc::daa::*;
pub use misc::nop::*;
pub use opcodes::*;
pub use rotate::rlca::*;
pub use rotate::rra::*;
pub use rotate::rrca::*;

#[cfg(test)]
mod tests {
    use crate::bus::Bus;
    use crate::cpu::instructions::{
        AddressMode, ConditionType, Instruction, RegisterType, INSTRUCTIONS_BY_OPCODES,
    };
    use crate::cpu::Cpu;
    use crate::emu::EmuCtx;

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
        let mut cpu = Cpu::new(Bus::with_bytes(vec![0; 100000]));
        cpu.set_pc(0);
        cpu.clock.t_cycles = 0;
        cpu.bus.write(0, opcode as u8);
        cpu.step(&mut EmuCtx::new()).unwrap();
        let expected = M_CYCLES_BY_OPCODES[opcode];
        let actual = cpu.clock.t_cycles / 4;

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_m_cycles_call() {
        let mut cpu = Cpu::new(Bus::with_bytes(vec![0; 100000]));
        for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
            let Instruction::Call(instr) = *instr else {
                continue;
            };

            cpu.set_pc(0);
            cpu.clock.t_cycles = 0;
            cpu.bus.write(0, opcode as u8);

            if let Some(condition_type) = instr.condition_type {
                assert_for_condition(&mut cpu, condition_type, 6, 3);
            } else {
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(6, cpu.clock.t_cycles / 4);
            };
        }
    }

    #[test]
    pub fn test_m_cycles_jp() {
        let mut cpu = Cpu::new(Bus::with_bytes(vec![0; 100000]));
        for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
            let Instruction::Jp(instr) = *instr else {
                continue;
            };

            cpu.set_pc(0);
            cpu.clock.t_cycles = 0;
            cpu.bus.write(0, opcode as u8);

            if let Some(condition_type) = instr.condition_type {
                assert_for_condition(&mut cpu, condition_type, 4, 3);
            } else if instr.address_mode == AddressMode::D16 {
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(4, cpu.clock.t_cycles / 4);
            } else if instr.address_mode == AddressMode::R(RegisterType::HL) {
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(1, cpu.clock.t_cycles / 4);
            };
        }
    }

    #[test]
    pub fn test_m_cycles_jr() {
        let mut cpu = Cpu::new(Bus::with_bytes(vec![0; 100000]));
        for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
            let Instruction::Jr(instr) = *instr else {
                continue;
            };

            cpu.set_pc(0);
            cpu.clock.t_cycles = 0;
            cpu.bus.write(0, opcode as u8);

            if let Some(condition_type) = instr.condition_type {
                assert_for_condition(&mut cpu, condition_type, 3, 2);
            } else {
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(M_CYCLES_BY_OPCODES[opcode], cpu.clock.get_m_cycles());
                // 3
            };
        }
    }

    #[test]
    pub fn test_m_cycles_ret() {
        let mut cpu = Cpu::new(Bus::with_bytes(vec![0; 100000]));
        for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
            let Instruction::Ret(instr) = *instr else {
                continue;
            };

            cpu.set_pc(0);
            cpu.clock.t_cycles = 0;
            cpu.bus.write(0, opcode as u8);

            if let Some(condition_type) = instr.condition_type {
                assert_for_condition(&mut cpu, condition_type, 5, 2);
            } else {
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(M_CYCLES_BY_OPCODES[opcode], cpu.clock.get_m_cycles());
                // 4
            };
        }
    }

    #[test]
    pub fn test_m_cycles() {
        let mut cpu = Cpu::new(Bus::with_bytes(vec![0; 100000]));

        for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
            match instr {
                Instruction::Jp(_) // has tests
                | Instruction::Jr(_) // has tests
                | Instruction::Ret(_) // has tests
                | Instruction::Call(_) // has tests
                | Instruction::Stop(_)
                | Instruction::Halt(_)
                | Instruction::Unknown(_) => continue,
                _ => {}
            }

            if 0xCB == opcode {
                continue; // todo: investigate
            }

            cpu.set_pc(0);
            cpu.clock.t_cycles = 0;
            cpu.bus.write(0, opcode as u8);
            cpu.step(&mut EmuCtx::new()).unwrap();
            let expected = M_CYCLES_BY_OPCODES[opcode];
            let actual = cpu.clock.t_cycles / 4;

            if actual != expected {
                let msg = format!(
                    "Invalid M-Cycles for 0x{:02X}: actual={}, expected={}",
                    opcode, actual, expected
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
                cpu.registers.flags.set(None, None, None, false.into());
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(m_cycles_set, cpu.clock.t_cycles / 4);

                cpu.set_pc(0);
                cpu.clock.t_cycles = 0;

                cpu.registers.flags.set(None, None, None, true.into());
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(m_cycles_not, cpu.clock.t_cycles / 4);
            }
            ConditionType::C => {
                cpu.registers.flags.set(None, None, None, false.into());
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(m_cycles_not, cpu.clock.t_cycles / 4);

                cpu.set_pc(0);
                cpu.clock.t_cycles = 0;

                cpu.registers.flags.set(None, None, None, true.into());
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(m_cycles_set, cpu.clock.t_cycles / 4);
            }
            ConditionType::NZ => {
                cpu.registers.flags.set(false.into(), None, None, None);
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(m_cycles_set, cpu.clock.t_cycles / 4);

                cpu.set_pc(0);
                cpu.clock.t_cycles = 0;

                cpu.registers.flags.set(true.into(), None, None, None);
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(m_cycles_not, cpu.clock.t_cycles / 4);
            }
            ConditionType::Z => {
                cpu.registers.flags.set(false.into(), None, None, None);
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(m_cycles_not, cpu.clock.t_cycles / 4);

                cpu.set_pc(0);
                cpu.clock.t_cycles = 0;

                cpu.registers.flags.set(true.into(), None, None, None);
                cpu.step(&mut EmuCtx::new()).unwrap();
                assert_eq!(m_cycles_set, cpu.clock.t_cycles / 4);
            }
        }
    }
}
