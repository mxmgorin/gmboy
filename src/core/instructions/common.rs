use crate::core::cpu::Cpu;
use crate::core::instructions::nop;

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub r#type: Option<InstructionType>,
    pub address_mode: Option<AddressMode>,
    pub register_1_type: Option<RegisterType>,
    pub register_2_type: Option<RegisterType>,
    pub condition_type: Option<ConditionType>,
    pub param: Option<u8>,
    pub execute_fn: fn(instruction: &Instruction, cpu: &mut Cpu),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterType {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

const NONE_INSTRUCTION: Instruction = Instruction {
    r#type: None,
    address_mode: None,
    register_1_type: None,
    register_2_type: None,
    condition_type: None,
    param: None,
    execute_fn: nop::execute,
};

const INSTRUCTIONS_LEN: usize = 0xFF;

const INSTRUCTIONS: [Instruction; INSTRUCTIONS_LEN] = {
    let mut instructions = [NONE_INSTRUCTION; INSTRUCTIONS_LEN];

    instructions[nop::OPCODE as usize] = nop::new();
    instructions[0x04] = Instruction {
        r#type: Some(InstructionType::INC),
        address_mode: Some(AddressMode::R),
        register_1_type: Some(RegisterType::B),
        register_2_type: None,
        condition_type: None,
        param: None,
    };
    instructions[0x05] = Instruction {
        r#type: Some(InstructionType::DEC),
        address_mode: Some(AddressMode::R),
        register_1_type: Some(RegisterType::B),
        register_2_type: None,
        condition_type: None,
        param: None,
    };
    instructions[0x0E] = Instruction {
        r#type: Some(InstructionType::LD),
        address_mode: Some(AddressMode::R_D8),
        register_1_type: Some(RegisterType::C),
        register_2_type: None,
        condition_type: None,
        param: None,
    };
    instructions[0xAF] = Instruction {
        r#type: Some(InstructionType::XOR),
        address_mode: Some(AddressMode::R),
        register_1_type: Some(RegisterType::A),
        register_2_type: None,
        condition_type: None,
        param: None,
    };
    instructions[0xC3] = Instruction {
        r#type: Some(InstructionType::JP),
        address_mode: Some(AddressMode::D16),
        register_1_type: None,
        register_2_type: None,
        condition_type: None,
        param: None,
    };
    instructions[0xF3] = Instruction {
        r#type: Some(InstructionType::DI),
        address_mode: None,
        register_1_type: None,
        register_2_type: None,
        condition_type: None,
        param: None,
    };

    // todo: Add more instructions here...

    instructions
};

pub fn get_instruction_by_opcode(opcode: u8) -> Option<&'static Instruction> {
    INSTRUCTIONS.get(opcode as usize)
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
    /// Non-zero
    Nz,
    /// Zero
    Z,
    /// Non-carry
    Nc,
    /// Carry
    C,
}

/// Represents the different address modes in the CPU's instruction set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum AddressMode {
    /// Immediate Addressing: The operand is directly specified in the instruction.
    IMP,
    /// Register with 16-bit immediate address: The operand is a 16-bit immediate value,
    /// and the instruction works with a register.
    R_D16,
    /// Register to Register: The operand is another register, and the instruction operates
    /// between two registers.
    R_R,
    /// Memory to Register: The operand is a memory location, and the instruction operates
    /// between memory and a register.
    MR_R,
    /// Register: The operand is a register.
    R,
    /// Register with 8-bit immediate value: The operand is an 8-bit immediate value,
    /// and the instruction operates with a register.
    R_D8,
    /// Register with Memory to Register: The instruction reads a value from memory and stores
    /// it into a register.
    R_MR,
    /// Register and HL increment: The instruction uses the `HL` register pair, increments it,
    /// and accesses memory using the updated value of `HL`.
    R_HLI,
    /// Register and HL decrement: The instruction uses the `HL` register pair, decrements it,
    /// and accesses memory using the updated value of `HL`.
    R_HLD,
    /// HL increment and Register: The instruction stores a value from a register to memory and
    /// increments the `HL` register pair.
    HLI_R,
    /// HL decrement and Register: The instruction stores a value from a register to memory and
    /// decrements the `HL` register pair.
    HLD_R,
    /// Register and 8-bit immediate address: The instruction uses a 8-bit immediate address and
    /// a register for memory access.
    R_A8,
    /// 8-bit address and Register: The instruction uses a memory address and a register to store
    /// a value from the register to memory.
    A8_R,
    /// HL and Special Register Pair: This mode uses the `HL` register and other special register pairs
    /// for specific operations.
    HL_SPR,
    /// 16-bit immediate data: The instruction involves a 16-bit immediate operand.
    D16,
    /// 8-bit immediate data: The instruction involves an 8-bit immediate operand.
    D8,
    /// 16-bit immediate data to Register: The instruction loads a 16-bit immediate operand to a register.
    D16_R,
    /// Memory Read and 8-bit immediate address: The instruction reads from memory using an 8-bit immediate address.
    MR_D8,
    /// Memory Read: The instruction performs a read operation from memory.
    MR,
    /// 16-bit Address and Register: The instruction works with a 16-bit memory address and a register.
    A16_R,
    /// Register and 16-bit Address: The instruction stores a value from a register to a 16-bit memory address.
    R_A16,
}