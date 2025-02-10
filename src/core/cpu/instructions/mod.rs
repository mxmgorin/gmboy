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
    use crate::cpu::instructions::{Instruction, INSTRUCTIONS_BY_OPCODES};
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
        let mut cpu = Cpu::new(Bus::flat_mem(vec![0; 100000]));
        cpu.set_pc(0);
        cpu.clock.ticks = 0;
        cpu.bus.write(0, opcode as u8);
        cpu.step(&mut EmuCtx::new()).unwrap();
        let expected = M_CYCLES_BY_OPCODES[opcode];
        let actual = cpu.clock.ticks / 4;

        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_m_cycles() {
        let mut cpu = Cpu::new(Bus::flat_mem(vec![0; 100000]));

        for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
            if let Instruction::Unknown(_) = instr {
                continue;
            }

            if let Instruction::Stop(_) = instr {
                continue; // is incorrect in matrix?
            }

            if let Instruction::Halt(_) = instr {
                continue; // is incorrect in matrix?
            }

            if let Instruction::Jp(_) = instr {
                continue; // todo: handle branching
            }

            if let Instruction::Jr(_) = instr {
                continue; // todo: handle branching
            }

            if let Instruction::Ret(_) = instr {
                continue; // todo: handle branching
            }

            if let Instruction::Call(_) = instr {
                continue; // todo: handle branching
            }

            if 0xCB == opcode {
                continue; // todo: investigate
            }

            cpu.set_pc(0);
            cpu.clock.ticks = 0;
            cpu.bus.write(0, opcode as u8);
            cpu.step(&mut EmuCtx::new()).unwrap();
            let expected = M_CYCLES_BY_OPCODES[opcode];
            let actual = cpu.clock.ticks / 4;

            if actual != expected {
                let msg = format!(
                    "Invalid M-Cycles for 0x{:02X}: actual={}, expected={}",
                    opcode, actual, expected
                );
                panic!("{}", msg);
            }
        }
    }
}
