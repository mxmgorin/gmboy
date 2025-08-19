use crate::cpu::instructions::fetch::AddressMode;
use crate::cpu::instructions::opcodes::INSTRUCTIONS_BY_OPCODES;
use crate::cpu::instructions::{ConditionType, FetchedData};
use crate::cpu::Cpu;

#[derive(Copy, Clone)]
pub struct InstructionSpec {
    pub cond_type: Option<ConditionType>,
    pub addr: u16,
    pub addr_mode: AddressMode,
}

impl InstructionSpec {
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
pub struct Instruction {
    mnemonic: Mnemonic,
    spec: InstructionSpec,
    execute: fn(&mut Cpu, fetched_data: FetchedData, spec: InstructionSpec),
    fetch: fn(&mut Cpu) -> FetchedData,
}

impl Instruction {
    pub const fn get_by_opcode(opcode: u8) -> &'static Instruction {
        &INSTRUCTIONS_BY_OPCODES[opcode as usize]
    }

    pub fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        (self.execute)(cpu, fetched_data, self.spec);
    }

    pub fn fetch(&self, cpu: &mut Cpu) -> FetchedData {
        (self.fetch)(cpu)
    }

    pub fn get_address_mode(&self) -> AddressMode {
        self.spec.addr_mode
    }

    pub fn get_mnemonic(&self) -> Mnemonic {
        self.mnemonic
    }

    pub fn get_condition(&self) -> Option<ConditionType> {
        self.spec.cond_type
    }

    pub const fn unknown(_opcode: u8) -> Self {
        Self::new(
            Mnemonic::Unknown,
            InstructionSpec::default(AddressMode::IMP),
            |_, _, _| panic!("can't fetch for unknown instruction for opcode"),
            |_| panic!("can't fetch for unknown instruction"),
        )
    }

    pub const fn new(
        mnemonic: Mnemonic,
        args: InstructionSpec,
        execute: fn(&mut Cpu, fetched_data: FetchedData, arg: InstructionSpec),
        fetch: fn(&mut Cpu) -> FetchedData,
    ) -> Self {
        Self {
            mnemonic,
            spec: args,
            execute,
            fetch,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Mnemonic {
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
