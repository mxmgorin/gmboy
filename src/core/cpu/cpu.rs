use crate::bus::Bus;
use crate::core::cpu::instructions::{AddressMode, ExecutableInstruction, Instruction};
use crate::core::cpu::Registers;
use crate::cpu::instructions::RegisterType;
use crate::debugger::Debugger;
use crate::LittleEndianBytes;

pub trait CpuCallback {
    fn m_cycles(&mut self, m_cycles: usize, bus: &mut Bus);
}

#[derive(Default, Debug, Clone, Copy)]
pub struct CounterCpuCallback {
    pub m_cycles_count: usize,
}

impl CpuCallback for CounterCpuCallback {
    fn m_cycles(&mut self, m_cycles: usize, _bus: &mut Bus) {
        self.m_cycles_count += m_cycles;
    }
}

#[derive(Debug, Clone)]
pub struct Cpu {
    pub bus: Bus,
    pub registers: Registers,
    pub enabling_ime: bool,
    pub current_opcode: u8,
    pub is_halted: bool,
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        Self {
            bus,
            registers: Registers::new(),
            enabling_ime: false,
            current_opcode: 0,
            is_halted: false,
        }
    }

    /// Reads 8bit immediate data by PC and increments PC + 1. Costs 1 M-Cycle.
    pub fn fetch_data(&mut self, callback: &mut impl CpuCallback) -> u8 {
        let value = self.bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        callback.m_cycles(1, &mut self.bus);

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
        let value = self.bus.read(address) as u16;
        callback.m_cycles(1, &mut self.bus);

        value
    }

    /// Writes to memory. Costs 1 M-Cycle.
    pub fn write_to_memory(&mut self, address: u16, value: u8, callback: &mut impl CpuCallback) {
        self.bus.write(address, value);
        callback.m_cycles(1, &mut self.bus);
    }

    pub fn step(
        &mut self,
        callback: &mut impl CpuCallback,
        debugger: Option<&mut Debugger>,
    ) -> Result<(), String> {
        #[cfg(debug_assertions)]
        if let Some(debugger) = &debugger {
            debugger.print_gb_doctor_info(self);
        }

        if self.is_halted {
            callback.m_cycles(1, &mut self.bus);

            if self.bus.io.interrupts.int_flags != 0 {
                self.is_halted = false;
            }

            return Ok(());
        }

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
        if let Some(debugger) = debugger {
            debugger.print_cpu_info(self, pc, instruction, self.current_opcode, &fetched_data);
            debugger.update_serial(self);
        }

        let prev_enabling_ime = self.enabling_ime;
        instruction.execute(self, callback, fetched_data);

        if self.bus.io.interrupts.ime {
            if let Some((addr, it)) = self.bus.io.interrupts.check_interrupts() {
                Instruction::goto_addr(self, None, addr, true, callback);
                self.bus.io.interrupts.handle_interrupt(it);
                self.is_halted = false;
            }

            self.enabling_ime = false;
        }

        if self.enabling_ime && prev_enabling_ime {
            // execute after next instruction when flag is changed
            self.enabling_ime = false;
            self.bus.io.interrupts.ime = true;
        }

        Ok(())
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
