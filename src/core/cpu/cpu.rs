use crate::core::bus::Bus;
use crate::core::cpu::instructions::common::{AddressMode, ExecutableInstruction, Instruction};
use crate::core::cpu::stack::Stack;
use crate::core::cpu::Registers;
use crate::core::debugger::Debugger;
use crate::core::{InterruptType};
use crate::cpu::instructions::common::{RegisterType};
use crate::util::{LittleEndianBytes};

#[derive(Debug, Clone)]
pub struct Cpu {
    pub bus: Bus,
    pub registers: Registers,
    pub enabling_ime: bool,
    pub ticks: i32,
    pub current_opcode: u8,
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        Self {
            bus,
            registers: Registers::new(),
            enabling_ime: false,
            ticks: 0,
            current_opcode: 0,
        }
    }

    pub fn fetch_data(&mut self) -> u8 {
        let value = self.bus.read(self.registers.pc);
        self.update_cycles(1);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        value
    }

    pub fn fetch_data16(&mut self) -> u16 {
        let bytes = LittleEndianBytes {
            low_byte: self.fetch_data(),
            high_byte: self.fetch_data(),
        };

        bytes.into()
    }

    pub fn step(&mut self, debugger: &mut Option<Debugger>) -> Result<(), String> {
        #[cfg(debug_assertions)]
        if let Some(debugger) = debugger {
            debugger.print_gb_doctor_info(self);
        }

        if self.bus.io.interrupts.cpu_halted {
            self.update_cycles(1);

            if self.bus.io.interrupts.int_flags != 0 {
                self.bus.io.interrupts.cpu_halted = false;
            }

            return Ok(());
        } 

        let pc = self.registers.pc;
        self.current_opcode = self.fetch_opcode();

        let Some(instruction) = Instruction::get_by_opcode(self.current_opcode) else {
            return Err(format!(
                "Unknown instruction OPCODE: {:X}",
                self.current_opcode,
            ));
        };

        let fetched_data = AddressMode::fetch_data(self, instruction.get_address_mode());

        #[cfg(debug_assertions)]
        if let Some(debugger) = debugger.as_mut() {
            debugger.print_cpu_info(self, pc, instruction, self.current_opcode, &fetched_data);
            debugger.update(self);
            debugger.print();
        }

        let prev_enabling_ime = self.enabling_ime;        
        instruction.execute(self, fetched_data);

        if self.bus.io.interrupts.ime {
            if let Some((addr, it)) = self.bus.io.interrupts.check_interrupts() {
                self.handle_interrupt(addr, it);
            }

            self.enabling_ime = false;
        }

        if self.enabling_ime && prev_enabling_ime { // execute after next instruction when flag is changed
            self.enabling_ime = false;
            self.bus.io.interrupts.ime = true;
        }

        Ok(())
    }

    pub fn update_cycles(&mut self, cpu_cycles: i32) {
        for _ in 0..cpu_cycles {
            for _ in 0..4 {
                self.ticks += 1;

                if self.bus.io.timer.tick() {
                    self.bus
                        .io
                        .interrupts
                        .request_interrupt(InterruptType::Timer);
                }

                //ppu_tick(); todo
            }

            //dma_tick(); todo
        }
    }

    pub fn read_reg8(&self, rt: RegisterType) -> u8 {
        match rt {
            RegisterType::A => self.registers.a,
            RegisterType::F => self.registers.flags.byte,
            RegisterType::B => self.registers.b,
            RegisterType::C => self.registers.c,
            RegisterType::D => self.registers.d,
            RegisterType::E => self.registers.e,
            RegisterType::H => self.registers.h,
            RegisterType::L => self.registers.l,
            RegisterType::HL => self.bus.read(self.registers.read_register(RegisterType::HL)),
            _ => {
                panic!("**ERR INVALID REG8: {:?}", rt);
            }
        }
    }

    pub fn set_reg8(&mut self, rt: RegisterType, val: u8) {
        match rt {
            RegisterType::A => self.registers.a = val & 0xFF,
            RegisterType::F => self.registers.flags.byte = val & 0xFF,
            RegisterType::B => self.registers.b = val & 0xFF,
            RegisterType::C => self.registers.c = val & 0xFF,
            RegisterType::D => self.registers.d = val & 0xFF,
            RegisterType::E => self.registers.e = val & 0xFF,
            RegisterType::H => self.registers.h = val & 0xFF,
            RegisterType::L => self.registers.l = val & 0xFF,
            RegisterType::HL => self.bus.write(self.registers.read_register(RegisterType::HL), val),
            _ => {
                panic!("**ERR INVALID REG8: {:?}", rt);
            }
        }
    }

    fn handle_interrupt(&mut self, address: u16, it: InterruptType) {
        Stack::push16(&mut self.registers, &mut self.bus, address);
        self.registers.pc = address;
        
        self.bus.io.interrupts.handle_interrupt(it);
    }

    fn fetch_opcode(&mut self) -> u8 {
        let opcode = self.bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        opcode
    }
}
