use crate::core::cpu::instructions::call::CallInstruction;
use crate::core::cpu::instructions::ccf::CcfInstruction;
use crate::core::cpu::instructions::common::address_mode::AddressMode;
use crate::core::cpu::instructions::common::opcodes::INSTRUCTIONS_BY_OPCODES;
use crate::core::cpu::instructions::common::ConditionType;
use crate::core::cpu::instructions::cpl::CplInstruction;
use crate::core::cpu::instructions::daa::DaaInstruction;
use crate::core::cpu::instructions::dec::DecInstruction;
use crate::core::cpu::instructions::di::DiInstruction;
use crate::core::cpu::instructions::halt::HaltInstruction;
use crate::core::cpu::instructions::inc::IncInstruction;
use crate::core::cpu::instructions::jp::JpInstruction;
use crate::core::cpu::instructions::jr::JrInstruction;
use crate::core::cpu::instructions::ld::LdInstruction;
use crate::core::cpu::instructions::ldh::LdhInstruction;
use crate::core::cpu::instructions::nop::NopInstruction;
use crate::core::cpu::instructions::xor::XorInstruction;
use crate::core::cpu::stack::Stack;
use crate::core::cpu::{Cpu}; use crate::cpu::instructions::common::FetchedData;
use std::fmt::Display;
use crate::cpu::instructions::adc::AdcInstruction;
use crate::cpu::instructions::add::AddInstruction;
use crate::cpu::instructions::and::AndInstruction;
use crate::cpu::instructions::cp::CpInstruction;
use crate::cpu::instructions::ei::EiInstruction;
use crate::cpu::instructions::or::OrInstruction;
use crate::cpu::instructions::pop::PopInstruction;
use crate::cpu::instructions::push::PushInstruction;
use crate::cpu::instructions::ret::RetInstruction;
use crate::cpu::instructions::reti::RetiInstruction;
use crate::cpu::instructions::rla::RlaInstruction;
use crate::cpu::instructions::rlca::RlcaInstruction;
use crate::cpu::instructions::rra::RraInstruction;
use crate::cpu::instructions::rrca::RrcaInstruction;
use crate::cpu::instructions::scf::ScfInstruction;
use crate::cpu::instructions::stop::StopInstruction;
use crate::cpu::instructions::sub::SubInstruction;

pub trait ExecutableInstruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData);
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
    Rra(RraInstruction),
    Rla(RlaInstruction),
    Rrca(RrcaInstruction),
    Rlca(RlcaInstruction),
    Or(OrInstruction),
    Ret(RetInstruction),
    Reti(RetiInstruction),
    Ei(EiInstruction),
    Scf(ScfInstruction),
    Stop(StopInstruction),
    And(AndInstruction),
    Push(PushInstruction),
    Pop(PopInstruction),
    Cp(CpInstruction),
    Add(AddInstruction),
    Sub(SubInstruction),
    Adc(AdcInstruction)
}

impl ExecutableInstruction for Instruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        match self {
            Instruction::Unknown(opcode) => {
                panic!("Can't execute an unknown instruction {:X}", opcode)
            }
            Instruction::Nop(inst) => inst.execute(cpu, fetched_data),
            Instruction::Inc(inst) => inst.execute(cpu, fetched_data),
            Instruction::Dec(inst) => inst.execute(cpu, fetched_data),
            Instruction::Ld(inst) => inst.execute(cpu, fetched_data),
            Instruction::Jr(inst) => inst.execute(cpu, fetched_data),
            Instruction::Daa(inst) => inst.execute(cpu, fetched_data),
            Instruction::Cpl(inst) => inst.execute(cpu, fetched_data),
            Instruction::Ccf(inst) => inst.execute(cpu, fetched_data),
            Instruction::Halt(inst) => inst.execute(cpu, fetched_data),
            Instruction::Xor(inst) => inst.execute(cpu, fetched_data),
            Instruction::Di(inst) => inst.execute(cpu, fetched_data),
            Instruction::Jp(inst) => inst.execute(cpu, fetched_data),
            Instruction::Ldh(inst) => inst.execute(cpu, fetched_data),
            Instruction::Call(inst) => inst.execute(cpu, fetched_data),
            Instruction::Rra(inst) => inst.execute(cpu, fetched_data),
            Instruction::Rla(inst) => inst.execute(cpu, fetched_data),
            Instruction::Rrca(inst) => inst.execute(cpu, fetched_data),
            Instruction::Rlca(inst) => inst.execute(cpu, fetched_data),
            Instruction::Or(inst) => inst.execute(cpu, fetched_data),
            Instruction::Ret(inst) => inst.execute(cpu, fetched_data),
            Instruction::Reti(inst) => inst.execute(cpu, fetched_data),
            Instruction::Ei(inst) => inst.execute(cpu, fetched_data),
            Instruction::Scf(inst) => inst.execute(cpu, fetched_data),
            Instruction::Stop(inst) => inst.execute(cpu, fetched_data),
            Instruction::And(inst) => inst.execute(cpu, fetched_data),
            Instruction::Push(inst) => inst.execute(cpu, fetched_data),
            Instruction::Pop(inst) => inst.execute(cpu, fetched_data),
            Instruction::Cp(inst) => inst.execute(cpu, fetched_data),
            Instruction::Add(inst) => inst.execute(cpu, fetched_data),
            Instruction::Sub(inst) => inst.execute(cpu, fetched_data),
            Instruction::Adc(inst) => inst.execute(cpu, fetched_data),
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
            Instruction::Rra(inst) => inst.get_address_mode(),
            Instruction::Rla(inst) => inst.get_address_mode(),
            Instruction::Rrca(inst) => inst.get_address_mode(),
            Instruction::Rlca(inst) => inst.get_address_mode(),
            Instruction::Or(inst) => inst.get_address_mode(),
            Instruction::Ret(inst) => inst.get_address_mode(),
            Instruction::Reti(inst) => inst.get_address_mode(),
            Instruction::Ei(inst) => inst.get_address_mode(),
            Instruction::Scf(inst) => inst.get_address_mode(),
            Instruction::Stop(inst) => inst.get_address_mode(),
            Instruction::And(inst) => inst.get_address_mode(),
            Instruction::Push(inst) => inst.get_address_mode(),
            Instruction::Pop(inst) => inst.get_address_mode(),
            Instruction::Cp(inst) => inst.get_address_mode(),
            Instruction::Add(inst) => inst.get_address_mode(),
            Instruction::Sub(inst) => inst.get_address_mode(),
            Instruction::Adc(inst) => inst.get_address_mode(),
        }
    }
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
            Instruction::Rra(_) => InstructionType::RRA,
            Instruction::Rla(_) => InstructionType::RLA,
            Instruction::Rrca(_) => InstructionType::RRCA,
            Instruction::Rlca(_) => InstructionType::RLCA,
            Instruction::Or(_) => InstructionType::OR,
            Instruction::Ret(_) => InstructionType::RET,
            Instruction::Reti(_) => InstructionType::RETI,
            Instruction::Ei(_) => InstructionType::EI,
            Instruction::Scf(_) => InstructionType::SCF,
            Instruction::Stop(_) => InstructionType::STOP,
            Instruction::And(_) => InstructionType::AND,
            Instruction::Push(_) => InstructionType::PUSH,
            Instruction::Pop(_) => InstructionType::POP,
            Instruction::Cp(_) => InstructionType::CP,
            Instruction::Add(_) => InstructionType::ADD,
            Instruction::Sub(_) => InstructionType::SUB,
            Instruction::Adc(_) => InstructionType::ADC,
        }
    }

    pub fn get_by_opcode(opcode: u8) -> Option<&'static Instruction> {
        INSTRUCTIONS_BY_OPCODES.get(opcode as usize)
    }

    pub fn goto_addr(cpu: &mut Cpu, cond: Option<ConditionType>, addr: u16, push_pc: bool) {
        if ConditionType::check_cond(&cpu.registers, cond) {
            if push_pc {
                cpu.update_cycles(2);
                let pc = cpu.registers.pc;
                Stack::push16(&mut cpu.registers, &mut cpu.bus, pc);
            }

            cpu.registers.pc = addr;
            cpu.update_cycles(1);
        }
    }

    pub fn to_asm_string(&self, cpu: &Cpu, fetched_data: &FetchedData) -> String {
        match self.get_address_mode() {
            AddressMode::IMP => format!("{:?}", self.get_type()),
            AddressMode::R_D16(r1) | AddressMode::R_A16(r1) => {
                format!("{:?} {:?},${:04X}", self.get_type(), r1, fetched_data.value)
            }
            AddressMode::R(r1) => {
                format!("{:?} {:?}", self.get_type(), r1)
            }
            AddressMode::R_R(r1, r2) => {
                format!("{:?} {:?},{:?}", self.get_type(), r1, r2)
            }
            AddressMode::MR_R(r1, r2) => {
                format!("{:?} {:?},{:?}", self.get_type(), r1, r2)
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
                    fetched_data.value & 0xFF
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
                    fetched_data.value & 0xFF
                )
            }
            AddressMode::D8 => {
                format!("{:?} ${:02X}", self.get_type(), fetched_data.value & 0xFF)
            }
            AddressMode::D16 => {
                format!("{:?} ${:04X}", self.get_type(), fetched_data.value)
            }
            AddressMode::MR_D8(r1) => {
                format!(
                    "{:?} {:?}),${:02X}",
                    self.get_type(),
                    r1,
                    fetched_data.value & 0xFF
                )
            }
            AddressMode::A16_R(r2) => {
                format!(
                    "{:?} (${:04X}),{:?}",
                    self.get_type(),
                    fetched_data.value,
                    r2
                )
            }
            _ => {
                panic!("INVALID address mode: {:?}", self.get_address_mode());
            }
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
