use crate::auxiliary::clock::{Clock};
use crate::cpu::instructions::{ConditionType, Instruction, InstructionWrapper};
use crate::cpu::instructions::{FetchedData, RegisterType};
use crate::cpu::Registers;
use serde::{Deserialize, Serialize};

pub const CPU_CLOCK_SPEED: u32 = 4194304;

pub struct DebugCtx {
    pub pc: u16,
    pub instruction: InstructionWrapper,
    pub opcode: u8,
    pub fetched_data: FetchedData,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Cpu {
    pub registers: Registers,
    pub enabling_ime: bool,
    pub current_opcode: u8,
    pub is_halted: bool,
    pub clock: Clock,
}

impl Cpu {
    pub fn new(clock: Clock) -> Self {
        Self {
            registers: Default::default(),
            enabling_ime: false,
            current_opcode: 0,
            is_halted: false,
            clock,
        }
    }

    /// Costs 2 M-Cycles with push PC
    #[inline]
    pub fn goto_addr(&mut self, cond: Option<ConditionType>, addr: u16, push_pc: bool) {
        if ConditionType::check_cond(&self.registers, cond) {
            self.clock.m_cycles(1); // internal: branch decision?
            if push_pc {
                self.push16(self.registers.pc);
            }

            self.registers.pc = addr;
        }
    }

    /// Reads 8bit immediate data by PC and increments PC + 1. Costs 1 M-Cycle.
    #[inline]
    pub fn read_pc(&mut self) -> u8 {
        let value = self.clock.bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.clock.m_cycles(1);

        value
    }

    /// Reads 16bit immediate data by PC and increments PC + 2. Costs 1 M-Cycle.
    #[inline]
    pub fn read_pc16(&mut self) -> u16 {
        u16::from_le_bytes([self.read_pc(), self.read_pc()])
    }

    /// Reads data from memory. Costs 1 M-Cycle.
    #[inline]
    pub fn read_memory(&mut self, address: u16) -> u16 {
        let value = self.clock.bus.read(address) as u16;
        self.clock.m_cycles(1);

        value
    }

    /// Writes to memory. Costs 1 M-Cycle.
    #[inline]
    pub fn write_to_memory(&mut self, address: u16, value: u8) {
        self.clock.bus.write(address, value);
        self.clock.m_cycles(1);
    }

    pub fn step(
        &mut self,
        mut _debugger: Option<&mut crate::debugger::Debugger>,
    ) -> Result<(), String> {
        #[cfg(debug_assertions)]
        if let Some(ref mut debugger) = _debugger {
            debugger.print(self, None);
        }

        self.handle_interrupts();

        if self.is_halted {
            if !self.clock.bus.io.interrupts.ime && self.clock.bus.io.interrupts.any_is_pending() {
                // HALT bug: continue executing instructions
                self.is_halted = false;
            }

            // Do nothing, just wait for an interrupt to wake up
            self.clock.m_cycles(1);

            return Ok(());
        }

        #[cfg(debug_assertions)]
        let pc = self.registers.pc;

        self.current_opcode = self.read_pc();

        let Some(instruction) = Instruction::get_by_opcode(self.current_opcode) else {
            return Err(format!(
                "Unknown instruction OPCODE: {:X}",
                self.current_opcode,
            ));
        };

        let fetched_data = instruction.fetch(self);

        #[cfg(debug_assertions)]
        let inst_ctx = DebugCtx {
            pc,
            instruction: instruction.clone(),
            opcode: self.current_opcode,
            fetched_data: fetched_data.clone(),
        };
        #[cfg(debug_assertions)]
        if let Some(debugger) = _debugger {
            debugger.print(self, Some(inst_ctx));
            debugger.update_serial(&mut self.clock.bus);
        }

        let prev_enabling_ime = self.enabling_ime;
        instruction.execute(self, fetched_data);

        if self.enabling_ime && prev_enabling_ime {
            // execute after next instruction when flag is changed
            self.enabling_ime = false;
            self.clock.bus.io.interrupts.ime = true;
        }

        Ok(())
    }

    /// Costs 5 M-cycles when an interrupt is executed
    pub fn handle_interrupts(&mut self) {
        if self.clock.bus.io.interrupts.ime {
            if let Some((addr, it)) = self.clock.bus.io.interrupts.get_pending() {
                // execute interrupt handler
                self.clock.m_cycles(2);

                self.is_halted = false;
                self.clock.bus.io.interrupts.acknowledge_interrupt(it);
                self.goto_addr(None, addr, true);

                self.clock.m_cycles(1);
            }

            self.enabling_ime = false;
        }
    }

    pub fn read_reg8(&mut self, rt: RegisterType) -> u8 {
        match rt {
            RegisterType::A => self.registers.a,
            RegisterType::F => self.registers.flags.byte,
            RegisterType::B => self.registers.b,
            RegisterType::C => self.registers.c,
            RegisterType::D => self.registers.d,
            RegisterType::E => self.registers.e,
            RegisterType::H => self.registers.h,
            RegisterType::L => self.registers.l,
            RegisterType::HL => {
                self.read_memory(self.registers.read_register(RegisterType::HL)) as u8
            }
            _ => {
                panic!("**ERR INVALID REG8: {rt:?}");
            }
        }
    }

    pub fn set_reg8(&mut self, rt: RegisterType, val: u8) {
        match rt {
            RegisterType::A => self.registers.a = val,
            RegisterType::F => self.registers.flags.byte = val,
            RegisterType::B => self.registers.b = val,
            RegisterType::C => self.registers.c = val,
            RegisterType::D => self.registers.d = val,
            RegisterType::E => self.registers.e = val,
            RegisterType::H => self.registers.h = val,
            RegisterType::L => self.registers.l = val,
            RegisterType::HL => {
                self.write_to_memory(self.registers.read_register(RegisterType::HL), val)
            }
            _ => {
                panic!("**ERR INVALID REG8: {rt:?}");
            }
        }
    }
}
