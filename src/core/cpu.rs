use crate::core::bus::Bus;

#[derive(Debug, Clone)]
pub struct Cpu {
    bus: Bus,
    registers: CpuRegisters,
    current_opcode: u8,
    current_instruction: Option<&'static CpuInstruction>,
    halted: bool,
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        Self {
            bus,
            registers: CpuRegisters::new(),
            current_opcode: 0,
            current_instruction: None,
            halted: false,
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.halted {
            return Ok(());
        }

        self.fetch_instruction();
        //self.fetch_data();
        self.execute()?;

        Err("cpu step not implemented yet".to_string())
    }

    fn execute(&mut self) -> Result<(), String> {
        let Some(current_instruction) = self.current_instruction.take() else {
            return Err(format!(
                "Unknown instruction opcode: {}",
                self.current_opcode
            ));
        };

        Ok(())
    }

    fn fetch_instruction(&mut self) {
        self.current_opcode = self.bus.read(self.registers.pc);
        self.registers.pc += 1;
        self.current_instruction = get_instruction_by_opcode(self.current_opcode);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CpuInstruction {
    pub r#type: Option<InstructionType>,
    pub address_mode: Option<AddressMode>,
    pub register_1_type: Option<RegisterType>,
    pub register_2_type: Option<RegisterType>,
    pub condition_type: Option<ConditionType>,
    pub param: Option<u8>,
}

const CPU_INSTRUCTIONS: [CpuInstruction; 0x100] = {
    let mut instructions = [CpuInstruction {
        r#type: None,
        address_mode: None,
        register_1_type: None,
        register_2_type: None,
        condition_type: None,
        param: None,
    }; 0x100];

    instructions[0x00] = CpuInstruction {
        r#type: Some(InstructionType::Nop),
        address_mode: Some(AddressMode::Imp),
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
    None,
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
