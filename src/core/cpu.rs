use crate::core::bus::Bus;
use crate::core::debugger::Debugger;
use crate::core::instructions::common::{
    AddressMode, ExecutableInstruction, Instruction, RegisterType,
};
use crate::core::stack::Stack;
use crate::core::util::{get_bit_flag, reverse_u16, set_bit};
use crate::core::InterruptType;

#[derive(Debug, Clone)]
pub struct Cpu {
    pub bus: Bus,
    pub registers: Registers,
    pub enabling_ime: bool,
    pub ticks: i32,
    pub current_opcode: u8,
}

#[derive(Debug, Clone, Default)]
pub struct FetchedData {
    pub value: u16,
    pub mem_dest: u16,
    pub dest_is_mem: bool,
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

    pub fn step(&mut self, debugger: &mut Option<Debugger>) -> Result<(), String> {
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

        let fetched_data = self.fetch_data(instruction);

        #[cfg(debug_assertions)]
        if let Some(debugger) = debugger.as_mut() {
            debugger.print_cpu_info(self, pc, instruction, self.current_opcode, &fetched_data);
            debugger.update(self);
            debugger.print();
        }

        self.execute(instruction, fetched_data)?;

        if self.bus.io.interrupts.int_master_enabled {
            if let Some(it) = self.bus.io.interrupts.get_interrupt() {
                self.handle_interrupt(it);
            }

            self.enabling_ime = false;
        }

        if self.enabling_ime {
            self.bus.io.interrupts.int_master_enabled = true;
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

    fn handle_interrupt(&mut self, it: InterruptType) {
        self.bus.io.interrupts.handle_interrupt(it);

        let address = it.get_address();
        Stack::push16(&mut self.registers, &mut self.bus, address);
        self.registers.pc = address;
    }

    fn execute(
        &mut self,
        instruction: &Instruction,
        fetched_data: FetchedData,
    ) -> Result<(), String> {
        instruction.execute(self, fetched_data);

        Ok(())
    }

    fn fetch_opcode(&mut self) -> u8 {
        let opcode = self.bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        opcode
    }

    fn fetch_data(&mut self, instruction: &Instruction) -> FetchedData {
        let mut fetched_data = FetchedData::default();

        match instruction.get_address_mode() {
            AddressMode::IMP => (),
            AddressMode::R(r1) => {
                fetched_data.value = self.registers.read_register(r1);
            }
            AddressMode::R_R(_r1, r2) => {
                fetched_data.value = self.registers.read_register(r2);
            }
            AddressMode::R_D8(_r1) => {
                fetched_data.value = self.bus.read(self.registers.pc) as u16;
                self.update_cycles(1);
                self.registers.pc += 1;
            }
            AddressMode::D16 | AddressMode::R_D16(_) => {
                let lo = self.bus.read(self.registers.pc);
                self.update_cycles(1);
                let hi = self.bus.read(self.registers.pc + 1);
                self.update_cycles(1);
                fetched_data.value = (hi as u16) << 8 | (lo as u16);
                self.registers.pc += 2;
            }
            AddressMode::R_MR(_r1, r2) => {
                let mut addr = self.registers.read_register(r2);

                if r2 == RegisterType::C {
                    addr |= 0xFF0;
                }
                fetched_data.value = self.bus.read(addr) as u16;
                self.update_cycles(1);
            }
            AddressMode::MR_R(r1, r2) => {
                fetched_data.value = self.registers.read_register(r2);
                fetched_data.mem_dest = self.registers.read_register(r1);
                fetched_data.dest_is_mem = true;

                if r1 == RegisterType::C {
                    fetched_data.mem_dest |= 0xFF00;
                }
            }
            AddressMode::R_HLI(_r1, r2) => {
                fetched_data.value = self.bus.read(self.registers.read_register(r2)) as u16;
                self.update_cycles(1);
                self.registers.set_register(
                    RegisterType::HL,
                    self.registers.read_register(RegisterType::H) + 1,
                );
            }
            AddressMode::R_HLD(_r1, r2) => {
                fetched_data.value = self.bus.read(self.registers.read_register(r2)) as u16;
                self.update_cycles(1);
                self.registers.set_register(
                    RegisterType::HL,
                    self.registers
                        .read_register(RegisterType::H)
                        .wrapping_sub(1),
                );
            }
            AddressMode::HLI_R(r1, r2) => {
                fetched_data.value = self.registers.read_register(r2);
                fetched_data.mem_dest = self.registers.read_register(r1);
                fetched_data.dest_is_mem = true;
                self.registers.set_register(
                    RegisterType::HL,
                    self.registers.read_register(RegisterType::HL) + 1,
                );
            }
            AddressMode::HLD_R(r1, r2) => {
                fetched_data.value = self.registers.read_register(r2);
                fetched_data.mem_dest = self.registers.read_register(r1);
                fetched_data.dest_is_mem = true;
                self.registers.set_register(
                    RegisterType::HL,
                    self.registers.read_register(RegisterType::HL) - 1,
                );
            }
            AddressMode::R_A8(_r1) => {
                fetched_data.value = self.bus.read(self.registers.pc) as u16;
                self.update_cycles(1);
                self.registers.pc += 1;
            }
            AddressMode::A8_R(_r1) => {
                fetched_data.mem_dest = self.bus.read(self.registers.pc) as u16 | 0xFF00;
                fetched_data.dest_is_mem = true;
                self.update_cycles(1);
                self.registers.pc += 1;
            }
            AddressMode::HL_SPR(_r1, _r2) => {
                fetched_data.value = self.bus.read(self.registers.pc) as u16;
                self.update_cycles(1);
                self.registers.pc += 1;
            }
            AddressMode::D8 => {
                fetched_data.value = self.bus.read(self.registers.pc) as u16;
                self.update_cycles(1);
                self.registers.pc += 1;
            }
            AddressMode::D16_R(r1) | AddressMode::A16_R(r1) => {
                let lo = self.bus.read(self.registers.pc) as u16;
                self.update_cycles(1);

                let hi = self.bus.read(self.registers.pc + 1) as u16;
                self.update_cycles(1);

                fetched_data.mem_dest = lo | (hi << 8);
                fetched_data.dest_is_mem = true;

                self.registers.pc += 2;
                fetched_data.value = self.registers.read_register(r1);
            }
            AddressMode::MR_D8(r1) => {
                fetched_data.value = self.bus.read(self.registers.pc) as u16;
                self.update_cycles(1);
                self.registers.pc += 1;
                fetched_data.mem_dest = self.registers.read_register(r1);
                fetched_data.dest_is_mem = true;
            }
            AddressMode::MR(r1) => {
                fetched_data.mem_dest = self.registers.read_register(r1);
                fetched_data.dest_is_mem = true;
                fetched_data.value = self.bus.read(self.registers.read_register(r1)) as u16;
            }
            AddressMode::R_A16(_r1) => {
                let lo = self.bus.read(self.registers.pc) as u16;
                self.update_cycles(1);

                let hi = self.bus.read(self.registers.pc + 1) as u16;
                self.update_cycles(1);

                let addr = lo | (hi << 8);

                self.registers.pc += 2;
                fetched_data.value = self.bus.read(addr) as u16;
                self.update_cycles(1);
            }
        }

        fetched_data
    }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

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
            RegisterType::AF => reverse_u16(((self.a as u16) << 8) | (self.f as u16)),
            RegisterType::BC => reverse_u16(((self.b as u16) << 8) | (self.c as u16)),
            RegisterType::DE => reverse_u16(((self.d as u16) << 8) | (self.e as u16)),
            RegisterType::HL => reverse_u16(((self.h as u16) << 8) | (self.l as u16)),
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
            set_bit(&mut self.f, ZERO_FLAG_BYTE_POSITION, z != 0);
        }

        if n != -1 {
            set_bit(&mut self.f, SUBTRACT_FLAG_BYTE_POSITION, n != 0);
        }

        if h != -1 {
            set_bit(&mut self.f, HALF_CARRY_FLAG_BYTE_POSITION, h != 0);
        }

        if c != -1 {
            set_bit(&mut self.f, CARRY_FLAG_BYTE_POSITION, c != 0);
        }
    }

    pub fn get_flag_z(&self) -> bool {
        get_bit_flag(self.f, ZERO_FLAG_BYTE_POSITION)
    }

    pub fn get_flag_n(&self) -> bool {
        get_bit_flag(self.f, SUBTRACT_FLAG_BYTE_POSITION)
    }

    pub fn get_flag_h(&self) -> bool {
        get_bit_flag(self.f, HALF_CARRY_FLAG_BYTE_POSITION)
    }

    pub fn get_flag_c(&self) -> bool {
        get_bit_flag(self.f, CARRY_FLAG_BYTE_POSITION)
    }

    pub fn flags_to_string(&self) -> String {
        [
            (self.get_flag_z(), 'Z'),
            (self.get_flag_n(), 'N'),
            (self.get_flag_h(), 'H'),
            (self.get_flag_c(), 'C'),
        ]
        .iter()
        .map(|&(flag, c)| if flag { c } else { '-' })
        .collect()
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
