use crate::core::bus::Bus;
use crate::core::util::reverse_u16;

#[derive(Debug, Clone)]
pub struct Cpu {
    bus: Bus,
    registers: CpuRegisters,
    current_opcode: u8,
    current_instruction: Option<&'static CpuInstruction>,
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
            current_opcode: 0,
            current_instruction: None,
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

        self.fetch_instruction();
        self.fetch_data();
        self.execute()?;

        Ok(())
    }

    fn execute(&mut self) -> Result<(), String> {
        let Some(current_instruction) = self.current_instruction.take() else {
            return Err(format!(
                "Unknown instruction opcode: {}",
                self.current_opcode
            ));
        };

        if cfg!(debug_assertions) {
            println!("Executing: {:?}", current_instruction);
        }

        Err("cpu execute not implemented yet".into())
    }

    fn fetch_instruction(&mut self) {
        self.current_opcode = self.bus.read(self.registers.pc);
        self.registers.pc += 1;
        self.current_instruction = get_instruction_by_opcode(self.current_opcode);
    }

    fn fetch_data(&mut self) {
        let Some(current_instruction) = self.current_instruction else {
            return;
        };

        match current_instruction.address_mode {
            AddressMode::Imp => (),
            AddressMode::R => {
                self.fetched_data = self.read_register(
                    current_instruction
                        .register_1_type
                        .expect("must be set for R type"),
                );
            }
            _ => eprintln!("Not implemented instruction: {:?}", current_instruction),
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
    address_mode: AddressMode::Imp,
    register_1_type: None,
    register_2_type: None,
    condition_type: None,
    param: None,
};
const INSTRUCTIONS_LEN: usize = 1;
const CPU_INSTRUCTIONS: [CpuInstruction; INSTRUCTIONS_LEN] = {
    let mut instructions = [NONE_INSTRUCTION; INSTRUCTIONS_LEN];

    instructions[0x00] = CpuInstruction {
        r#type: Some(InstructionType::Nop),
        address_mode: AddressMode::Imp,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressMode {
    Imp,
    R_D16,
    R_R,
    MR_R,
    R,
    R_D8,
    R_MR,
    R_HLI,
    R_HLD,
    HLI_R,
    HLD_R,
    R_A8,
    A8_R,
    HL_SPR,
    D16,
    D8,
    D16_R,
    MR_D8,
    MR,
    A16_R,
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
    None,
    Nop,
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
    Jp,
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
    None,
    /// Non-zero
    Nz,
    /// Zero
    Z,
    /// Non-carry
    Nc,
    /// Carry
    C,
}
