use crate::core::bus::Bus;
use crate::core::instructions::common::{
    AddressMode, ExecutableInstruction, Instruction, RegisterType,
};
use crate::core::util::{get_bit_flag, reverse_u16};

#[derive(Debug, Clone)]
pub struct Cpu {
    bus: Bus,
    registers: Registers,
    halted: bool,
    mem_dest: u16,
    fetched_data: u16,
    dest_is_mem: bool,
}

impl Cpu {
    pub fn new(bus: Bus) -> Cpu {
        Self {
            bus,
            registers: Registers::new(),
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

        let Some(instruction) = Instruction::get_by_opcode(opcode) else {
            return Err(format!("Unknown instruction opcode: 0x{opcode:X}",));
        };

        self.fetch_data(instruction);
        self.execute(instruction)?;

        Ok(())
    }

    fn execute(&mut self, instruction: &Instruction) -> Result<(), String> {
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

    fn fetch_data(&mut self, instruction: &Instruction) {
        match instruction.get_address_mode() {
            AddressMode::IMP => (),
            AddressMode::R(r1) => {
                self.fetched_data = self.read_register(r1);
            }
            AddressMode::R_R(_r1, r2) => {
                self.fetched_data = self.read_register(r2);
            }
            AddressMode::R_D8(_r1) => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                //emu_cycles(1);
                self.registers.pc += 1;
            }
            AddressMode::D16 | AddressMode::R_D16(_)  => {
                let lo = self.bus.read(self.registers.pc);
                //emu_cycles(1);
                let hi = self.bus.read(self.registers.pc + 1);
                //emu_cycles(1);
                self.fetched_data = (hi as u16) << 8 | (lo as u16);
                self.registers.pc += 2;
            }
            AddressMode::R_MR(r1) => {
                let mut addr = self.read_register(r1);
                
                if r1 == RegisterType::C {
                    addr |= 0xFF0;
                }
                self.fetched_data = self.bus.read(addr) as u16;
                //emu_cycles(1);
            }
            AddressMode::MR_R(r1, r2) => {
                self.fetched_data = self.read_register(r2);
                self.mem_dest = self.read_register(r1);
                self.dest_is_mem = true;
                
                if r1 == RegisterType::C {
                    self.mem_dest |= 0xFF00;
                }
            }
            AddressMode::R_HLI(_r1, r2) => {
                self.fetched_data = self.bus.read(self.read_register(r2)) as u16;
                //emu_cycles(1);
                self.set_register(RegisterType::HL, self.read_register(RegisterType::H) - 1);
            }
            AddressMode::R_HLD(_, _) => {}
            AddressMode::HLI_R(_, _) => {}
            AddressMode::HLD_R(_, _) => {}
            AddressMode::R_A8(_) => {}
            AddressMode::A8_R(_) => {}
            AddressMode::HL_SPR(_, _) => {}
            AddressMode::D8 => {}
            AddressMode::D16_R(_) => {}
            AddressMode::MR_D8(_) => {}
            AddressMode::MR(_) => {}
            AddressMode::A16_R(_) => {}
            AddressMode::R_A16(_) => {}
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
    
    fn set_register(&mut self, register_type: RegisterType, val: u16) {
        match register_type {
            RegisterType::A => self.registers.a = (val & 0xFF) as u8,
            RegisterType::F => self.registers.f = (val & 0xFF) as u8,
            RegisterType::B => self.registers.b = (val & 0xFF) as u8,
            RegisterType::C => self.registers.c = (val & 0xFF) as u8,
            RegisterType::D => self.registers.d = (val & 0xFF) as u8,
            RegisterType::E => self.registers.e = (val & 0xFF) as u8,
            RegisterType::H => self.registers.h = (val & 0xFF) as u8,
            RegisterType::L => self.registers.l = (val & 0xFF) as u8,
            RegisterType::AF => {
                let reversed = reverse_u16(val);
                self.registers.a = (reversed >> 8) as u8;
                self.registers.f = (reversed & 0xFF) as u8;
            }
            RegisterType::BC => {
                let reversed = reverse_u16(val);
                self.registers.b = (reversed >> 8) as u8;
                self.registers.c = (reversed & 0xFF) as u8;
            }
            RegisterType::DE => {
                let reversed = reverse_u16(val);
                self.registers.d = (reversed >> 8) as u8;
                self.registers.e = (reversed & 0xFF) as u8;
            }
            RegisterType::HL => {
                let reversed = reverse_u16(val);
                self.registers.h = (reversed >> 8) as u8;
                self.registers.l = (reversed & 0xFF) as u8;
            }
            RegisterType::PC => self.registers.pc = val,
            RegisterType::SP => self.registers.sp = val,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Registers {
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

impl Registers {
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

    pub fn get_flag_z(&self) -> bool {
        get_bit_flag(self.f, 7)
    }

    pub fn get_flag_c(&self) -> bool {
        get_bit_flag(self.f, 4)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::Registers;

    #[test]
    fn test_get_flag_z() {
        let mut regs = Registers::new();
        regs.f = 0b10000000;
        assert!(regs.get_flag_z());

        regs.f = 0b00000000;
        assert!(!regs.get_flag_z());
    }

    #[test]
    fn test_get_flag_c() {
        let mut regs = Registers::new();
        regs.f = 0b00010000;
        assert!(regs.get_flag_c());

        regs.f = 0b00000000;
        assert!(!regs.get_flag_c());
    }
}
