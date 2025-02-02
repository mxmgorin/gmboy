use crate::core::cpu::{Cpu, Registers};
use crate::core::instructions::call::CallInstruction;
use crate::core::instructions::ccf::CcfInstruction;
use crate::core::instructions::cpl::CplInstruction;
use crate::core::instructions::daa::DaaInstruction;
use crate::core::instructions::dec::DecInstruction;
use crate::core::instructions::di::DiInstruction;
use crate::core::instructions::halt::HaltInstruction;
use crate::core::instructions::inc::IncInstruction;
use crate::core::instructions::jp::JpInstruction;
use crate::core::instructions::jr::JrInstruction;
use crate::core::instructions::ld::LdInstruction;
use crate::core::instructions::ldh::LdhInstruction;
use crate::core::instructions::nop::NopInstruction;
use crate::core::instructions::table::INSTRUCTIONS_BY_OPCODES;
use crate::core::instructions::xor::XorInstruction;
use crate::core::stack::Stack;
use std::fmt::Display;

pub trait ExecutableInstruction {
    fn execute(&self, cpu: &mut Cpu);
    fn get_address_mode(&self) -> AddressMode;
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Unknown(u8),
    Nop(NopInstruction),
    Inc(IncInstruction),
    Dec(DecInstruction),
    Ld(LdInstruction),
    Jr(JrInstruction),
    Daa(DaaInstruction),
    Cpl(CplInstruction),
    Ccf(CcfInstruction),
    Halt(HaltInstruction),
    Xor(XorInstruction),
    Di(DiInstruction),
    Jp(JpInstruction),
    Ldh(LdhInstruction),
    Call(CallInstruction),
}

impl Instruction {
    fn get_type(&self) -> InstructionType {
        match self {
            Instruction::Unknown(opcode) => {
                panic!("Can't get_type for unknown instruction {:X}", opcode)
            }
            Instruction::Nop(_inst) => InstructionType::NOP,
            Instruction::Inc(_inst) => InstructionType::INC,
            Instruction::Dec(_inst) => InstructionType::DEC,
            Instruction::Ld(_inst) => InstructionType::LD,
            Instruction::Jr(_inst) => InstructionType::JR,
            Instruction::Daa(_inst) => InstructionType::DAA,
            Instruction::Cpl(_inst) => InstructionType::CPL,
            Instruction::Ccf(_inst) => InstructionType::CCF,
            Instruction::Halt(_inst) => InstructionType::HALT,
            Instruction::Xor(_inst) => InstructionType::XOR,
            Instruction::Di(_inst) => InstructionType::DI,
            Instruction::Jp(_inst) => InstructionType::JP,
            Instruction::Ldh(_inst) => InstructionType::LDH,
            Instruction::Call(_inst) => InstructionType::CALL,
        }
    }

    pub fn get_by_opcode(opcode: u8) -> Option<&'static Instruction> {
        INSTRUCTIONS_BY_OPCODES.get(opcode as usize)
    }

    pub fn check_cond(registers: &Registers, cond: Option<ConditionType>) -> bool {
        let Some(cond) = cond else {
            return true;
        };

        match cond {
            ConditionType::C => registers.get_flag_c(),
            ConditionType::NC => !registers.get_flag_c(),
            ConditionType::Z => registers.get_flag_z(),
            ConditionType::NZ => !registers.get_flag_z(),
        }
    }

    pub fn goto_addr(cpu: &mut Cpu, cond: Option<ConditionType>, addr: u16, push_pc: bool) {
        if Instruction::check_cond(&cpu.registers, cond) {
            if push_pc {
                //emu_cycles(2);
                let pc = cpu.registers.pc;
                Stack::push16(&mut cpu.registers, &mut cpu.bus,pc);
            }

            cpu.registers.pc = addr;
            //emu_cycles(1);
        }
    }

    pub fn to_asm_string(&self, cpu: &Cpu) -> String {
        match self.get_address_mode() {
            AddressMode::IMP => format!("{:?}", self.get_type()),
            AddressMode::R_D16(r1) | AddressMode::R_A16(r1) => {
                format!("{:?} {:?},${:04X}", self.get_type(), r1, cpu.fetched_data)
            }
            AddressMode::R(r1) => {
                format!("{:?} {:?}", self.get_type(), r1)
            }
            AddressMode::R_R(r1, r2) => {
                format!("{:?} {:?},{:?}", self.get_type(), r1, r2)
            }
            AddressMode::MR_R(r1, r2) => {
                format!("{:?} ({:?},{:?}", self.get_type(), r1, r2)
            }
            AddressMode::MR(r1) => {
                format!("{:?} ({:?})", self.get_type(), r1)
            }
            AddressMode::R_MR(r1, r2) => {
                format!("{:?} {:?},({:?})", self.get_type(), r1, r2)
            }
            AddressMode::R_D8(r1) | AddressMode::R_A8(r1) => {
                format!(
                    "{:?} {:?},${:02X}",
                    self.get_type(),
                    r1,
                    cpu.fetched_data & 0xFF
                )
            }
            AddressMode::R_HLI(r1, r2) => {
                format!("{:?} {:?},({:?}+)", self.get_type(), r1, r2)
            }
            AddressMode::R_HLD(r1, r2) => {
                format!("{:?} {:?},({:?}-)", self.get_type(), r1, r2)
            }
            AddressMode::HLI_R(r1, r2) => {
                format!("{:?} ({:?}+),{:?}", self.get_type(), r1, r2)
            }
            AddressMode::HLD_R(r1, r2) => {
                format!("{:?} ({:?}-),{:?}", self.get_type(), r1, r2)
            }
            AddressMode::A8_R(r2) => {
                format!(
                    "{:?} ${:02X},{:?}",
                    self.get_type(),
                    cpu.bus.read(cpu.registers.pc - 1),
                    r2
                )
            }
            AddressMode::HL_SPR(r1, _r2) => {
                format!(
                    "{:?} ({:?},SP+{:?})",
                    self.get_type(),
                    r1,
                    cpu.fetched_data & 0xFF
                )
            }
            AddressMode::D8 => {
                format!("{:?} ${:02X}", self.get_type(), cpu.fetched_data & 0xFF)
            }
            AddressMode::D16 => {
                format!("{:?} ${:04X}", self.get_type(), cpu.fetched_data)
            }
            AddressMode::MR_D8(r1) => {
                format!(
                    "{:?} ({:?}),${:02X}",
                    self.get_type(),
                    r1,
                    cpu.fetched_data & 0xFF
                )
            }
            AddressMode::A16_R(r2) => {
                format!("{:?} (${:04X}),{:?}", self.get_type(), cpu.fetched_data, r2)
            }
            _ => {
                panic!("INVALID address mode: {:?}", self.get_address_mode());
            }
        }
    }
}

impl ExecutableInstruction for Instruction {
    fn execute(&self, cpu: &mut Cpu) {
        match self {
            Instruction::Unknown(opcode) => {
                panic!("Can't execute an unknown instruction {:X}", opcode)
            }
            Instruction::Nop(inst) => inst.execute(cpu),
            Instruction::Inc(inst) => inst.execute(cpu),
            Instruction::Dec(inst) => inst.execute(cpu),
            Instruction::Ld(inst) => inst.execute(cpu),
            Instruction::Jr(inst) => inst.execute(cpu),
            Instruction::Daa(inst) => inst.execute(cpu),
            Instruction::Cpl(inst) => inst.execute(cpu),
            Instruction::Ccf(inst) => inst.execute(cpu),
            Instruction::Halt(inst) => inst.execute(cpu),
            Instruction::Xor(inst) => inst.execute(cpu),
            Instruction::Di(inst) => inst.execute(cpu),
            Instruction::Jp(inst) => inst.execute(cpu),
            Instruction::Ldh(inst) => inst.execute(cpu),
            Instruction::Call(inst) => inst.execute(cpu),
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        match self {
            Instruction::Unknown(opcode) => panic!(
                "Can't get_address_mode for unknown instruction {:X}",
                opcode
            ),
            Instruction::Nop(inst) => inst.get_address_mode(),
            Instruction::Inc(inst) => inst.get_address_mode(),
            Instruction::Dec(inst) => inst.get_address_mode(),
            Instruction::Ld(inst) => inst.get_address_mode(),
            Instruction::Jr(inst) => inst.get_address_mode(),
            Instruction::Daa(inst) => inst.get_address_mode(),
            Instruction::Cpl(inst) => inst.get_address_mode(),
            Instruction::Ccf(inst) => inst.get_address_mode(),
            Instruction::Halt(inst) => inst.get_address_mode(),
            Instruction::Xor(inst) => inst.get_address_mode(),
            Instruction::Di(inst) => inst.get_address_mode(),
            Instruction::Jp(inst) => inst.get_address_mode(),
            Instruction::Ldh(inst) => inst.get_address_mode(),
            Instruction::Call(inst) => inst.get_address_mode(),
        }
    }
}

/// Represents the various CPU registers in a Game Boy CPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterType {
    /// Accumulator register, used for arithmetic and logic operations.
    A,
    /// Flags register, holds condition flags (Z, N, H, C).
    F,
    /// General-purpose register B.
    B,
    /// General-purpose register C.
    C,
    /// General-purpose register D.
    D,
    /// General-purpose register E.
    E,
    /// High byte of the HL register pair.
    H,
    /// Low byte of the HL register pair.
    L,
    /// Register pair combining A and F (used for specific operations).
    AF,
    /// Register pair combining B and C (used for addressing or data storage).
    BC,
    /// Register pair combining D and E (used for addressing or data storage).
    DE,
    /// Register pair combining H and L (often used as a memory address pointer).
    HL,
    /// Stack pointer, points to the top of the stack.
    SP,
    /// Program counter, points to the next instruction to be executed.
    PC,
}

impl RegisterType {
    pub fn is_16bit(&self) -> bool {
        match self {
            RegisterType::A
            | RegisterType::F
            | RegisterType::B
            | RegisterType::C
            | RegisterType::D
            | RegisterType::E
            | RegisterType::H
            | RegisterType::L => false,
            RegisterType::AF
            | RegisterType::BC
            | RegisterType::DE
            | RegisterType::HL
            | RegisterType::SP
            | RegisterType::PC => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionType {
    /// No Operation
    NOP,
    /// Load (LD) instruction
    LD,
    /// Increment (INC) instruction
    INC,
    /// Decrement (DEC) instruction
    DEC,
    /// Rotate Left Circular (RLCA) instruction
    RLCA,
    /// Add (ADD) instruction
    ADD,
    /// Rotate Right Circular (RRCA) instruction
    RRCA,
    /// Stop execution
    STOP,
    /// Rotate Left (RLA) instruction
    RLA,
    /// Jump Relative (JR) instruction
    JR,
    /// Rotate Right (RRA) instruction
    RRA,
    /// Decimal Adjust Accumulator (DAA) instruction
    DAA,
    /// Complement (CPL) instruction
    CPL,
    /// Set Carry Flag (SCF) instruction
    SCF,
    /// Complement Carry Flag (CCF) instruction
    CCF,
    /// Halt execution
    HALT,
    /// Add with Carry (ADC) instruction
    ADC,
    /// Subtract (SUB) instruction
    SUB,
    /// Subtract with Carry (SBC) instruction
    SBC,
    /// Logical AND (AND) instruction
    AND,
    /// Logical XOR (XOR) instruction
    XOR,
    /// Logical OR (OR) instruction
    OR,
    /// Compare (CP) instruction
    CP,
    /// Pop value from stack (POP) instruction
    POP,
    /// Jump (JP) instruction
    JP,
    /// Push value to stack (PUSH) instruction
    PUSH,
    /// Return from function (RET) instruction
    RET,
    /// CB prefix instruction (used for extended instructions)
    CB,
    /// Call function (CALL) instruction
    CALL,
    /// Return from interrupt (RETI) instruction
    RETI,
    /// Load high byte (LDH) instruction
    LDH,
    /// Jump to address in HL register (JPHL) instruction
    JPHL,
    /// Disable interrupts (DI) instruction
    DI,
    /// Enable interrupts (EI) instruction
    EI,
    /// Restart (RST) instruction
    RST,
    /// Error instruction
    ERR,
    /// Rotate Left Circular (RLC) instruction
    RLC,
    /// Rotate Right Circular (RRC) instruction
    RRC,
    /// Rotate Left (RL) instruction
    RL,
    /// Rotate Right (RR) instruction
    RR,
    /// Shift Left Arithmetic (SLA) instruction
    SLA,
    /// Shift Right Arithmetic (SRA) instruction
    SRA,
    /// Swap nibbles (SWAP) instruction
    SWAP,
    /// Shift Right Logical (SRL) instruction
    SRL,
    /// Test bit in register (BIT) instruction
    BIT,
    /// Reset bit in register (RES) instruction
    RES,
    /// Set bit in register (SET) instruction
    SET,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionType {
    /// Non-zero: Execute if Z is not set.
    NZ,
    /// Zero: Execute if Z is set.
    Z,
    /// Non-carry: Execute if C is not set.
    NC,
    /// Carry: Execute if C is set.
    C,
}

/// Represents the different address modes in the CPU's instruction set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum AddressMode {
    /// Implied: The operand is specified in the instruction itself.
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
    /// HL and Special Register Pair: This mode uses the `HL` register and other special register pairs
    /// for specific operations.
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

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!("{:?} {:?}", self.get_type(), self.get_address_mode());
        write!(f, "{:?}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_instruction() {
        let inst = Instruction::Ld(LdInstruction {
            address_mode: AddressMode::R_D16(RegisterType::BC),
        });

        println!("{:?}", inst);
    }
}
