use crate::core::cpu::instructions::common::instruction::{Instruction, RegisterType};
use crate::core::cpu::instructions::common::ExecutableInstruction;
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
    /// Implied: The operand is specified in the instruction itcpu.
    IMP,
    /// Register: The operand is a register.
    R(RegisterType),
    /// Register and 16-bit Data: The instruction stores direct value into register.
    R_D16(RegisterType),
    /// Register to Register: The operand is another register, and the instruction operates
    /// between two registers.
    R_R(RegisterType, RegisterType),
    /// Indirect (memory address) Register and Register: The instruction read value from register
    /// and stores it into memory address
    MR_R(RegisterType, RegisterType),
    /// Register and 8-bit data: The operand is an 8-bit immediate value,
    /// and the instruction operates with a register.
    R_D8(RegisterType),
    /// Register and Indirect (memory address) Register: The instruction reads a value from memory and stores
    /// it into a register.
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
    /// Register and 8-bit immediate address
    R_A8(RegisterType),
    /// 8-bit address and Register: The instruction uses a memory address and a register to store
    /// a value from the register to memory.
    A8_R(RegisterType),
    /// Read PC value
    HL_SPR(RegisterType, RegisterType),
    /// 16-bit immediate data: The instruction involves a 16-bit immediate operand.
    D16,
    /// 8-bit immediate data: The instruction involves an 8-bit immediate operand.
    D8,
    /// 16-bit immediate data to Register
    D16_R(RegisterType),
    /// Memory Read and 8-bit immediate data
    MR_D8(RegisterType),
    /// Memory Read: The instruction performs a read operation from memory.
    MR(RegisterType),
    /// 16-bit Address and Register
    A16_R(RegisterType),
    /// Register and 16-bit Address
    R_A16(RegisterType),
}

impl AddressMode {
    pub fn fetch_data(cpu: &mut Cpu, instruction: &Instruction) -> FetchedData {
        let mut fetched_data = FetchedData::default();

        match instruction.get_address_mode() {
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
