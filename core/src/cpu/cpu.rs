use crate::bus::Bus;
use crate::cpu::instructions::{AddressMode, ExecutableInstruction, Instruction};
use crate::cpu::instructions::{FetchedData, RegisterType};
use crate::cpu::Registers;
use crate::LittleEndianBytes;
use serde::{Deserialize, Serialize};

pub const CPU_CLOCK_SPEED: u32 = 4194304;

pub struct DebugCtx {
    pub pc: u16,
    pub instruction: Instruction,
    pub opcode: u8,
    pub fetched_data: FetchedData,
}

pub trait CpuCallback {
    fn m_cycles(&mut self, m_cycles: usize);
    fn update_serial(&mut self, cpu: &mut Cpu);
    fn debug(&mut self, cpu: &mut Cpu, ctx: Option<DebugCtx>);
    fn get_bus_mut(&mut self) -> &mut Bus;
}

#[derive(Debug, Clone)]
pub struct CounterCpuCallback {
    pub m_cycles_count: usize,
    pub bus: Bus,
}

impl CpuCallback for CounterCpuCallback {
    fn m_cycles(&mut self, m_cycles: usize) {
        self.m_cycles_count += m_cycles;
    }

    fn update_serial(&mut self, _cpu: &mut Cpu) {}

    fn debug(&mut self, _cpu: &mut Cpu, _ctx: Option<DebugCtx>) {}

    fn get_bus_mut(&mut self) -> &mut Bus {
        &mut self.bus
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cpu {
    pub registers: Registers,
    pub enabling_ime: bool,
    pub current_opcode: u8,
    pub is_halted: bool,
}

impl Cpu {
    /// Reads 8bit immediate data by PC and increments PC + 1. Costs 1 M-Cycle.
    pub fn fetch_data(&mut self, callback: &mut impl CpuCallback) -> u8 {
        let value = callback.get_bus_mut().read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        callback.m_cycles(1);

        value
    }

    /// Reads 16bit immediate data by PC and increments PC + 2. Costs 1 M-Cycle.
    pub fn fetch_data16(&mut self, callback: &mut impl CpuCallback) -> u16 {
        let bytes = LittleEndianBytes {
            low_byte: self.fetch_data(callback),
            high_byte: self.fetch_data(callback),
        };

        bytes.into()
    }

    /// Reads data from memory. Costs 1 M-Cycle.
    pub fn read_memory(&mut self, address: u16, callback: &mut impl CpuCallback) -> u16 {
        let value = callback.get_bus_mut().read(address) as u16;
        callback.m_cycles(1);

        value
    }

    /// Writes to memory. Costs 1 M-Cycle.
    pub fn write_to_memory(&mut self, address: u16, value: u8, callback: &mut impl CpuCallback) {
        callback.get_bus_mut().write(address, value);
        callback.m_cycles(1);
    }

    pub fn step(&mut self, callback: &mut impl CpuCallback) -> Result<(), String> {
        #[cfg(debug_assertions)]
        callback.debug(self, None);

        self.handle_interrupts(callback);

        if self.is_halted {
            if !callback.get_bus_mut().io.interrupts.ime && callback.get_bus_mut().io.interrupts.any_is_pending() {
                // HALT bug: continue executing instructions
                self.is_halted = false;
            }

            // Do nothing, just wait for an interrupt to wake up
            callback.m_cycles(1);

            return Ok(());
        }

        #[cfg(debug_assertions)]
        let pc = self.registers.pc;

        self.current_opcode = self.fetch_data(callback);

        let Some(instruction) = Instruction::get_by_opcode(self.current_opcode) else {
            return Err(format!(
                "Unknown instruction OPCODE: {:X}",
                self.current_opcode,
            ));
        };

        let fetched_data = AddressMode::fetch_data(self, instruction.get_address_mode(), callback);

        #[cfg(debug_assertions)]
        let inst_ctx = DebugCtx {
            pc,
            instruction: instruction.to_owned(),
            opcode: self.current_opcode,
            fetched_data: fetched_data.clone(),
        };
        #[cfg(debug_assertions)]
        callback.debug(self, Some(inst_ctx));
        callback.update_serial(self);

        let prev_enabling_ime = self.enabling_ime;
        instruction.execute(self, callback, fetched_data);

        if self.enabling_ime && prev_enabling_ime {
            // execute after next instruction when flag is changed
            self.enabling_ime = false;
            callback.get_bus_mut().io.interrupts.ime = true;
        }

        Ok(())
    }

    /// Costs 5 M-cycles when an interrupt is executed
    pub fn handle_interrupts(&mut self, callback: &mut impl CpuCallback) {
        if callback.get_bus_mut().io.interrupts.ime {
            if let Some((addr, it)) = callback.get_bus_mut().io.interrupts.get_pending() {
                // execute interrupt handler
                callback.m_cycles(2);

                self.is_halted = false;
                callback.get_bus_mut().io.interrupts.acknowledge_interrupt(it);
                Instruction::goto_addr(self, None, addr, true, callback);

                callback.m_cycles(1);
            }

            self.enabling_ime = false;
        }
    }

    pub fn read_reg8(&mut self, rt: RegisterType, callback: &mut impl CpuCallback) -> u8 {
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
                self.read_memory(self.registers.read_register(RegisterType::HL), callback) as u8
            }
            _ => {
                panic!("**ERR INVALID REG8: {:?}", rt);
            }
        }
    }

    pub fn set_reg8(&mut self, rt: RegisterType, val: u8, callback: &mut impl CpuCallback) {
        match rt {
            RegisterType::A => self.registers.a = val & 0xFF,
            RegisterType::F => self.registers.flags.byte = val & 0xFF,
            RegisterType::B => self.registers.b = val & 0xFF,
            RegisterType::C => self.registers.c = val & 0xFF,
            RegisterType::D => self.registers.d = val & 0xFF,
            RegisterType::E => self.registers.e = val & 0xFF,
            RegisterType::H => self.registers.h = val & 0xFF,
            RegisterType::L => self.registers.l = val & 0xFF,
            RegisterType::HL => self.write_to_memory(
                self.registers.read_register(RegisterType::HL),
                val,
                callback,
            ),
            _ => {
                panic!("**ERR INVALID REG8: {:?}", rt);
            }
        }
    }
}
