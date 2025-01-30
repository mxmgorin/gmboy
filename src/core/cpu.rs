use crate::core::bus::Bus;
use crate::core::util::reverse_u16;

#[derive(Debug, Clone)]
pub struct Cpu {
    bus: Bus,
    registers: CpuRegisters,
    halted: bool,
    mem_dest: u16,
    fetched_data: u16,
    dest_is_mem: bool,
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        Self {
            bus,
            registers: CpuRegisters::new(),
            halted: false,
            mem_dest: 0,
            fetched_data: 0,
            dest_is_mem: false,
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.halted {
            return Ok(());
        }

        let opcode = self.fetch_opcode();

        let Some(instruction) = get_instruction_by_opcode(opcode) else {
            return Err(format!("Unknown instruction opcode: 0x{opcode:X}",));
        };

        self.fetch_data(instruction);
        self.execute(instruction)?;

        Ok(())
    }

    fn execute(&mut self, instruction: &CpuInstruction) -> Result<(), String> {
        if cfg!(debug_assertions) {
            println!("Executing: {:?}", instruction);
        }

        Ok(())
    }

    fn fetch_opcode(&mut self) -> u8 {
        let opcode = self.bus.read(self.registers.pc);
        self.registers.pc += 1;

        opcode
    }

    fn fetch_data(&mut self, instruction: &CpuInstruction) {
        match instruction.address_mode {
            AddressMode::IMP => (),
            AddressMode::R => {
                self.fetched_data = self
                    .read_register(instruction.register_1_type.expect("must be set for R type"));
            }
            _ => eprintln!(
                "Unimplemented Addressing Mode: {:?}",
                instruction.address_mode
            ),
        }
    }

    fn read_register(&self, register_type: RegisterType) -> u16 {
        match register_type {
            RegisterType::A => self.registers.a as u16,
            RegisterType::F => self.registers.f as u16,
            RegisterType::B => self.registers.b as u16,
            RegisterType::C => self.registers.c as u16,
            RegisterType::D => self.registers.d as u16,
            RegisterType::E => self.registers.e as u16,
            RegisterType::H => self.registers.h as u16,
            RegisterType::L => self.registers.l as u16,
            RegisterType::AF => {
                reverse_u16(((self.registers.a as u16) << 8) | (self.registers.f as u16))
            }
            RegisterType::BC => {
                reverse_u16(((self.registers.b as u16) << 8) | (self.registers.c as u16))
            }
            RegisterType::DE => {
                reverse_u16(((self.registers.d as u16) << 8) | (self.registers.e as u16))
            }
            RegisterType::HL => {
                reverse_u16(((self.registers.h as u16) << 8) | (self.registers.l as u16))
            }
            RegisterType::PC => self.registers.pc,
            RegisterType::SP => self.registers.sp,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CpuInstruction {
    pub r#type: Option<InstructionType>,
    pub address_mode: AddressMode,
    pub register_1_type: Option<RegisterType>,
    pub register_2_type: Option<RegisterType>,
    pub condition_type: Option<ConditionType>,
    pub param: Option<u8>,
}

const NONE_INSTRUCTION: CpuInstruction = CpuInstruction {
    r#type: None,
    address_mode: AddressMode::IMP,
    register_1_type: None,
    register_2_type: None,
    condition_type: None,
    param: None,
};
const INSTRUCTIONS_LEN: usize = 0xFF;
const CPU_INSTRUCTIONS: [CpuInstruction; INSTRUCTIONS_LEN] = {
    let mut instructions = [NONE_INSTRUCTION; INSTRUCTIONS_LEN];

    instructions[0x00] = CpuInstruction {
        r#type: Some(InstructionType::NOP),
        address_mode: AddressMode::IMP,
        register_1_type: None,
        register_2_type: None,
        condition_type: None,
        param: None,
    };
    instructions[0xC3] = CpuInstruction {
        r#type: Some(InstructionType::JP),
        address_mode: AddressMode::D16,
        register_1_type: None,
        register_2_type: None,
        condition_type: None,
        param: None,
    };

    // todo: Add more instructions here...

    instructions
};

fn get_instruction_by_opcode(opcode: u8) -> Option<&'static CpuInstruction> {
    CPU_INSTRUCTIONS.get(opcode as usize)
}

#[derive(Debug, Clone)]
pub struct CpuRegisters {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl CpuRegisters {
    pub fn new() -> Self {
        Self {
            a: 0x01,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0x100,
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionType {
    NOP,
    Ld,
    Inc,
    Dec,
    Rlca,
    Add,
    Rrca,
    Stop,
    Rla,
    Jr,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,
    Halt,
    Adc,
    Sub,
    Sbc,
    And,
    Xor,
    Or,
    Cp,
    Pop,
    JP,
    Push,
    Ret,
    Cb,
    Call,
    Reti,
    Ldh,
    Jphl,
    Di,
    Ei,
    Rst,
    Err,
    Rlc,
    Rrc,
    Rl,
    Rr,
    Sla,
    Sra,
    Swap,
    Srl,
    Bit,
    Res,
    Set,
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
