use crate::cpu::instructions::address_mode::AddressMode;
use crate::cpu::instructions::arithmetic::adc::AdcInstruction;
use crate::cpu::instructions::arithmetic::add::AddInstruction;
use crate::cpu::instructions::arithmetic::cp::CpInstruction;
use crate::cpu::instructions::arithmetic::dec::DecInstruction;
use crate::cpu::instructions::arithmetic::inc::IncInstruction;
use crate::cpu::instructions::arithmetic::sbc::SbcInstruction;
use crate::cpu::instructions::arithmetic::sub::SubInstruction;
use crate::cpu::instructions::bitwise::and::AndInstruction;
use crate::cpu::instructions::bitwise::cpl::CplInstruction;
use crate::cpu::instructions::bitwise::or::OrInstruction;
use crate::cpu::instructions::bitwise::xor::XorInstruction;
use crate::cpu::instructions::interrupt::di::DiInstruction;
use crate::cpu::instructions::interrupt::ei::EiInstruction;
use crate::cpu::instructions::interrupt::halt::HaltInstruction;
use crate::cpu::instructions::jump::call::CallInstruction;
use crate::cpu::instructions::jump::jp::JpInstruction;
use crate::cpu::instructions::jump::jr::JrInstruction;
use crate::cpu::instructions::jump::ret::RetInstruction;
use crate::cpu::instructions::jump::reti::RetiInstruction;
use crate::cpu::instructions::jump::rst::RstInstruction;
use crate::cpu::instructions::load::ld::LdInstruction;
use crate::cpu::instructions::load::ldh::LdhInstruction;
use crate::cpu::instructions::load::pop::PopInstruction;
use crate::cpu::instructions::load::push::PushInstruction;
use crate::cpu::instructions::misc::ccf::CcfInstruction;
use crate::cpu::instructions::misc::daa::DaaInstruction;
use crate::cpu::instructions::misc::nop::NopInstruction;
use crate::cpu::instructions::misc::prefix::PrefixInstruction;
use crate::cpu::instructions::misc::scf::ScfInstruction;
use crate::cpu::instructions::misc::stop::StopInstruction;
use crate::cpu::instructions::opcodes::INSTRUCTIONS_BY_OPCODES;
use crate::cpu::instructions::rotate::rla::RlaInstruction;
use crate::cpu::instructions::rotate::rlca::RlcaInstruction;
use crate::cpu::instructions::rotate::rra::RraInstruction;
use crate::cpu::instructions::rotate::rrca::RrcaInstruction;
use crate::cpu::instructions::{ConditionType, FetchedData};
use crate::cpu::Cpu;
use std::fmt::Display;

#[derive(Copy, Clone)]
pub struct InstructionArgs {
    pub cond_type: Option<ConditionType>,
    pub addr: u16,
    pub addr_mode: AddressMode,
}

impl InstructionArgs {
    pub const fn new(cond_type: Option<ConditionType>, addr: u16, addr_mode: AddressMode) -> Self {
        Self {
            cond_type,
            addr,
            addr_mode,
        }
    }

    pub const fn default(addr_mode: AddressMode) -> Self {
        Self {
            cond_type: None,
            addr: 0,
            addr_mode,
        }
    }
}

#[derive(Copy, Clone)]
pub struct InstructionWrapper {
    r#type: InstructionType,
    args: InstructionArgs,
    execute: fn(&mut Cpu, fetched_data: FetchedData, arg: InstructionArgs),
    fetch: fn(&mut Cpu) -> FetchedData,
}

impl InstructionWrapper {
    pub fn execute(self, cpu: &mut Cpu, fetched_data: FetchedData) {
        (self.execute)(cpu, fetched_data, self.args);
    }

    pub fn fetch(self, cpu: &mut Cpu) -> FetchedData {
        (self.fetch)(cpu)
    }
    
    pub fn get_address_mode(&self) -> AddressMode {
        self.args.addr_mode
    }
    
    pub fn get_type(&self) -> InstructionType {
        self.r#type
    }
    
    pub fn get_condition(&self) -> Option<ConditionType> {
        self.args.cond_type
    }
}

impl InstructionWrapper {
    pub const fn unknown(_opcode: u8) -> Self {
        Self::new(
            InstructionType::Unknown,
            InstructionArgs::default(AddressMode::IMP),
            |_, _, _| panic!("can't fetch for unknown instruction for opcode"),
            |_| panic!("can't fetch for unknown instruction"),
        )
    }

    pub const fn new(
        r#type: InstructionType,
        args: InstructionArgs,
        execute: fn(&mut Cpu, fetched_data: FetchedData, arg: InstructionArgs),
        fetch: fn(&mut Cpu) -> FetchedData,
    ) -> Self {
        Self {
            r#type,
            args,
            execute,
            fetch,
        }
    }
}

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
    Adc(AdcInstruction),
    Rst(RstInstruction),
    Prefix(PrefixInstruction),
    Sbc(SbcInstruction),
}

impl ExecutableInstruction for Instruction {
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        match self {
            Instruction::Unknown(opcode) => {
                panic!("Can't execute an unknown instruction {opcode:X}")
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
            Instruction::Rst(inst) => inst.execute(cpu, fetched_data),
            Instruction::Prefix(inst) => inst.execute(cpu, fetched_data),
            Instruction::Sbc(inst) => inst.execute(cpu, fetched_data),
        }
    }

    fn get_address_mode(&self) -> AddressMode {
        match self {
            Instruction::Unknown(opcode) => {
                panic!("Can't get_address_mode for unknown instruction {opcode:X}")
            }
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
            Instruction::Rst(inst) => inst.get_address_mode(),
            Instruction::Prefix(inst) => inst.get_address_mode(),
            Instruction::Sbc(inst) => inst.get_address_mode(),
        }
    }
}

impl Instruction {
    pub fn get_type(&self) -> InstructionType {
        match self {
            Instruction::Unknown(opcode) => {
                panic!("Can't get_type for unknown instruction {opcode:X}")
            }
            Instruction::Nop(_) => InstructionType::Nop,
            Instruction::Inc(_) => InstructionType::Inc,
            Instruction::Dec(_) => InstructionType::Dec,
            Instruction::Ld(_) => InstructionType::Ld,
            Instruction::Jr(_) => InstructionType::Jr,
            Instruction::Daa(_) => InstructionType::Daa,
            Instruction::Cpl(_) => InstructionType::Cpl,
            Instruction::Ccf(_) => InstructionType::Ccf,
            Instruction::Halt(_) => InstructionType::Halt,
            Instruction::Xor(_) => InstructionType::Xor,
            Instruction::Di(_) => InstructionType::Di,
            Instruction::Jp(_) => InstructionType::Jp,
            Instruction::Ldh(_) => InstructionType::Ldh,
            Instruction::Call(_) => InstructionType::Call,
            Instruction::Rra(_) => InstructionType::Rra,
            Instruction::Rla(_) => InstructionType::RLA,
            Instruction::Rrca(_) => InstructionType::Rrca,
            Instruction::Rlca(_) => InstructionType::Rlca,
            Instruction::Or(_) => InstructionType::Or,
            Instruction::Ret(_) => InstructionType::Ret,
            Instruction::Reti(_) => InstructionType::Reti,
            Instruction::Ei(_) => InstructionType::Ei,
            Instruction::Scf(_) => InstructionType::Scf,
            Instruction::Stop(_) => InstructionType::Stop,
            Instruction::And(_) => InstructionType::And,
            Instruction::Push(_) => InstructionType::Push,
            Instruction::Pop(_) => InstructionType::Pop,
            Instruction::Cp(_) => InstructionType::Cp,
            Instruction::Add(_) => InstructionType::Add,
            Instruction::Sub(_) => InstructionType::Sub,
            Instruction::Adc(_) => InstructionType::Adc,
            Instruction::Rst(_) => InstructionType::Rst,
            Instruction::Prefix(_) => InstructionType::CB,
            Instruction::Sbc(_) => InstructionType::Sbc,
        }
    }

    pub fn get_by_opcode(opcode: u8) -> Option<&'static InstructionWrapper> {
        INSTRUCTIONS_BY_OPCODES.get(opcode as usize)
    }
   
}

/// Represents the various CPU registers in a Game Boy CPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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

    pub const fn get_all() -> &'static [RegisterType] {
        &[
            RegisterType::A,
            RegisterType::F,
            RegisterType::B,
            RegisterType::C,
            RegisterType::D,
            RegisterType::E,
            RegisterType::H,
            RegisterType::L,
            RegisterType::AF,
            RegisterType::BC,
            RegisterType::DE,
            RegisterType::HL,
            RegisterType::SP,
            RegisterType::PC,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InstructionType {
    Unknown,
    /// No Operation
    Nop,
    /// Load (LD) instruction
    Ld,
    /// Increment (INC) instruction
    Inc,
    /// Decrement (DEC) instruction
    Dec,
    /// Rotate Left Circular (RLCA) instruction
    Rlca,
    /// Add (ADD) instruction
    Add,
    /// Rotate Right Circular (RRCA) instruction
    Rrca,
    /// Stop execution
    Stop,
    /// Rotate Left (RLA) instruction
    RLA,
    /// Jump Relative (JR) instruction
    Jr,
    /// Rotate Right (RRA) instruction
    Rra,
    /// Decimal Adjust Accumulator (DAA) instruction
    Daa,
    /// Complement (CPL) instruction
    Cpl,
    /// Set Carry Flag (SCF) instruction
    Scf,
    /// Complement Carry Flag (CCF) instruction
    Ccf,
    /// Halt execution
    Halt,
    /// Add with Carry (ADC) instruction
    Adc,
    /// Subtract (SUB) instruction
    Sub,
    /// Subtract with Carry (SBC) instruction
    Sbc,
    /// Logical AND (AND) instruction
    And,
    /// Logical XOR (XOR) instruction
    Xor,
    /// Logical OR (OR) instruction
    Or,
    /// Compare (CP) instruction
    Cp,
    /// Pop value from stack (POP) instruction
    Pop,
    /// Jump (JP) instruction
    Jp,
    /// Push value to stack (PUSH) instruction
    Push,
    /// Return from function (RET) instruction
    Ret,
    /// CB prefix instruction (used for extended instructions)
    CB,
    /// Call function (CALL) instruction
    Call,
    /// Return from interrupt (RETI) instruction
    Reti,
    /// Load high byte (LDH) instruction
    Ldh,
    /// Jump to address in HL register (JPHL) instruction
    JPHL,
    /// Disable interrupts (DI) instruction
    Di,
    /// Enable interrupts (EI) instruction
    Ei,
    /// Restart (RST) instruction
    Rst,
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
    Prefix,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = format!("{:?} {:?}", self.get_type(), self.get_address_mode());
        write!(f, "{str:?}")
    }
}
