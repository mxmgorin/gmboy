use crate::core::bus::Bus;
use crate::core::instructions::common::{
    AddressMode, ExecutableInstruction, Instruction, RegisterType,
};
use crate::core::util::{get_bit_flag, reverse_u16, set_bit};

#[derive(Debug, Clone)]
pub struct Cpu {
    pub bus: Bus,
    pub registers: Registers,
    pub halted: bool,
    pub mem_dest: u16,
    pub fetched_data: u16,
    pub dest_is_mem: bool,
    pub int_master_enabled: bool,
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
            int_master_enabled: false,
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.halted {
            return Ok(());
        }

        let pc = self.registers.pc;
        let opcode = self.fetch_opcode();

        let Some(instruction) = Instruction::get_by_opcode(opcode) else {
            return Err(format!("Unknown instruction OPCODE: {opcode:X}",));
        };

        #[cfg(debug_assertions)]
        self.print_debug_info(pc, instruction, opcode);

        self.fetch_data(instruction);
        self.execute(instruction)?;

        Ok(())
    }

    fn execute(&mut self, instruction: &Instruction) -> Result<(), String> {
        instruction.execute(self);

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
                self.fetched_data = self.registers.read_register(r1);
            }
            AddressMode::R_R(_r1, r2) => {
                self.fetched_data = self.registers.read_register(r2);
            }
            AddressMode::R_D8(_r1) => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                //emu_cycles(1);
                self.registers.pc += 1;
            }
            AddressMode::D16 | AddressMode::R_D16(_) => {
                let lo = self.bus.read(self.registers.pc);
                //emu_cycles(1);
                let hi = self.bus.read(self.registers.pc + 1);
                //emu_cycles(1);
                self.fetched_data = (hi as u16) << 8 | (lo as u16);
                self.registers.pc += 2;
            }
            AddressMode::R_MR(r1) => {
                let mut addr = self.registers.read_register(r1);

                if r1 == RegisterType::C {
                    addr |= 0xFF0;
                }
                self.fetched_data = self.bus.read(addr) as u16;
                //emu_cycles(1);
            }
            AddressMode::MR_R(r1, r2) => {
                self.fetched_data = self.registers.read_register(r2);
                self.mem_dest = self.registers.read_register(r1);
                self.dest_is_mem = true;

                if r1 == RegisterType::C {
                    self.mem_dest |= 0xFF00;
                }
            }
            AddressMode::R_HLI(_r1, r2) => {
                self.fetched_data = self.bus.read(self.registers.read_register(r2)) as u16;
                //emu_cycles(1);
                self.registers.set_register(RegisterType::HL, self.registers.read_register(RegisterType::H) + 1);
            }
            AddressMode::R_HLD(_r1, r2) => {
                self.fetched_data = self.bus.read(self.registers.read_register(r2)) as u16;
                //emu_cycles(1);
                self.registers.set_register(
                    RegisterType::HL,
                    self.registers.read_register(RegisterType::H).wrapping_sub(1),
                );
            }
            AddressMode::HLI_R(r1, r2) => {
                self.fetched_data = self.registers.read_register(r2);
                self.mem_dest = self.registers.read_register(r1);
                self.dest_is_mem = true;
                self.registers.set_register(RegisterType::HL, self.registers.read_register(RegisterType::HL) + 1);
            }
            AddressMode::HLD_R(r1, r2) => {
                self.fetched_data = self.registers.read_register(r2);
                self.mem_dest = self.registers.read_register(r1);
                self.dest_is_mem = true;
                self.registers.set_register(RegisterType::HL, self.registers.read_register(RegisterType::HL) - 1);
            }
            AddressMode::R_A8(_r1) => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                //emu_cycles(1);
                self.registers.pc += 1;
            }
            AddressMode::A8_R(_r1) => {
                self.mem_dest = self.bus.read(self.registers.pc) as u16 | 0xFF00;
                self.dest_is_mem = true;
                //emu_cycles(1);
                self.registers.pc += 1;
            }
            AddressMode::HL_SPR(_r1, _r2) => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                //emu_cycles(1);
                self.registers.pc += 1;
            }
            AddressMode::D8 => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                //emu_cycles(1);
                self.registers.pc += 1;
            }
            AddressMode::D16_R(r1) | AddressMode::A16_R(r1) => {
                let lo = self.bus.read(self.registers.pc) as u16;
                //emu_cycles(1);

                let hi = self.bus.read(self.registers.pc + 1) as u16;
                //emu_cycles(1);

                self.mem_dest = lo | (hi << 8);
                self.dest_is_mem = true;

                self.registers.pc += 2;
                self.fetched_data = self.registers.read_register(r1);
            }
            AddressMode::MR_D8(r1) => {
                self.fetched_data = self.bus.read(self.registers.pc) as u16;
                //emu_cycles(1);
                self.registers.pc += 1;
                self.mem_dest = self.registers.read_register(r1);
                self.dest_is_mem = true;
            }
            AddressMode::MR(r1) => {
                self.mem_dest = self.registers.read_register(r1);
                self.dest_is_mem = true;
                self.fetched_data = self.bus.read(self.registers.read_register(r1)) as u16;
            }
            AddressMode::R_A16(_r1) => {
                let lo = self.bus.read(self.registers.pc) as u16;
                //emu_cycles(1);

                let hi = self.bus.read(self.registers.pc + 1) as u16;
                //emu_cycles(1);

                let addr = lo | (hi << 8);

                self.registers.pc += 2;
                self.fetched_data = self.bus.read(addr) as u16;
                //emu_cycles(1);
            }
        }
    }

    #[cfg(debug_assertions)]
    fn print_debug_info(&self, pc: u16, instruction: &Instruction, opcode: u8) {
        let mut flags = String::new();
        flags.push(if self.registers.get_flag_z() {
            'Z'
        } else {
            '-'
        });
        flags.push(if self.registers.get_flag_n() {
            'N'
        } else {
            '-'
        });
        flags.push(if self.registers.get_flag_h() {
            'H'
        } else {
            '-'
        });
        flags.push(if self.registers.get_flag_c() {
            'C'
        } else {
            '-'
        });

        println!(
            "{:08X} - {:04X}: {} ({:02X} {:02X} {:02X}) A: {:02X} F: {} BC: {:02X}{:02X} DE: {:02X}{:02X} HL: {:02X}{:02X}",
            0,
            pc,
            instruction,
            opcode,
            self.bus.read(pc + 1),
            self.bus.read(pc + 2),
            self.registers.a,
            flags,
            self.registers.b,
            self.registers.c,
            self.registers.d,
            self.registers.e,
            self.registers.h,
            self.registers.l
        );
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
            sp: 0xFFFE,
            pc: 0x100,
        }
    }

    pub fn read_register(&self, register_type: RegisterType) -> u16 {
        match register_type {
            RegisterType::A => self.a as u16,
            RegisterType::F => self.f as u16,
            RegisterType::B => self.b as u16,
            RegisterType::C => self.c as u16,
            RegisterType::D => self.d as u16,
            RegisterType::E => self.e as u16,
            RegisterType::H => self.h as u16,
            RegisterType::L => self.l as u16,
            RegisterType::AF => {
                reverse_u16(((self.a as u16) << 8) | (self.f as u16))
            }
            RegisterType::BC => {
                reverse_u16(((self.b as u16) << 8) | (self.c as u16))
            }
            RegisterType::DE => {
                reverse_u16(((self.d as u16) << 8) | (self.e as u16))
            }
            RegisterType::HL => {
                reverse_u16(((self.h as u16) << 8) | (self.l as u16))
            }
            RegisterType::PC => self.pc,
            RegisterType::SP => self.sp,
        }
    }

    pub fn set_register(&mut self, register_type: RegisterType, val: u16) {
        match register_type {
            RegisterType::A => self.a = (val & 0xFF) as u8,
            RegisterType::F => self.f = (val & 0xFF) as u8,
            RegisterType::B => self.b = (val & 0xFF) as u8,
            RegisterType::C => self.c = (val & 0xFF) as u8,
            RegisterType::D => self.d = (val & 0xFF) as u8,
            RegisterType::E => self.e = (val & 0xFF) as u8,
            RegisterType::H => self.h = (val & 0xFF) as u8,
            RegisterType::L => self.l = (val & 0xFF) as u8,
            RegisterType::AF => {
                let reversed = reverse_u16(val);
                self.a = (reversed >> 8) as u8;
                self.f = (reversed & 0xFF) as u8;
            }
            RegisterType::BC => {
                let reversed = reverse_u16(val);
                self.b = (reversed >> 8) as u8;
                self.c = (reversed & 0xFF) as u8;
            }
            RegisterType::DE => {
                let reversed = reverse_u16(val);
                self.d = (reversed >> 8) as u8;
                self.e = (reversed & 0xFF) as u8;
            }
            RegisterType::HL => {
                let reversed = reverse_u16(val);
                self.h = (reversed >> 8) as u8;
                self.l = (reversed & 0xFF) as u8;
            }
            RegisterType::PC => self.pc = val,
            RegisterType::SP => self.sp = val,
        }
    }

    pub fn set_flags(&mut self, z: i8, n: i8, h: i8, c: i8) {
        if z != -1 {
            set_bit(&mut self.f, 7, z != 0);
        }

        if n != -1 {
            set_bit(&mut self.f, 6, n != 0);
        }

        if h != -1 {
            set_bit(&mut self.f, 5, h != 0);
        }

        if c != -1 {
            set_bit(&mut self.f, 4, c != 0);
        }
    }

    pub fn get_flag_z(&self) -> bool {
        get_bit_flag(self.f, 7)
    }

    pub fn get_flag_n(&self) -> bool {
        get_bit_flag(self.f, 6)
    }

    pub fn get_flag_h(&self) -> bool {
        get_bit_flag(self.f, 5)
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
