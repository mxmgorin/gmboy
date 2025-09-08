use crate::cpu::fetch::AddressMode;
use crate::cpu::instructions::opcode::INSTRUCTIONS;
use crate::cpu::instructions::JumpCondition;

#[derive(Copy, Clone)]
pub struct Instruction {
    pub cond_type: Option<JumpCondition>,
    pub operand_addr: u16,
    pub addr_mode: AddressMode,
    pub mnemonic: Mnemonic,
}

impl Instruction {
    #[inline(always)]
    pub const fn get_by_opcode(opcode: u8) -> &'static Instruction {
        &INSTRUCTIONS[opcode as usize]
    }

    pub fn get_address_mode(&self) -> AddressMode {
        self.addr_mode
    }

    pub fn get_mnemonic(&self) -> Mnemonic {
        self.mnemonic
    }

    pub fn get_condition(&self) -> Option<JumpCondition> {
        self.cond_type
    }

    pub const fn unknown(_opcode: u8) -> Self {
        Self::new(Mnemonic::Unknown, None, 0, AddressMode::IMP)
    }

    pub const fn new(
        mnemonic: Mnemonic,
        cond_type: Option<JumpCondition>,
        addr: u16,
        addr_mode: AddressMode,
    ) -> Self {
        Self {
            cond_type,
            operand_addr: addr,
            addr_mode,
            mnemonic,
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
