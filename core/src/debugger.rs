use crate::bus::Bus;
use crate::cpu::instructions::{AddressMode, Instruction, JumpCondition};
use crate::cpu::Cpu;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DebugLogType {
    None,
    Asm,
    GbDoc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Debugger {
    pub log_type: DebugLogType,
    serial_bytes: Vec<u8>,
    serial_enabled: bool,
}

impl Debugger {
    pub fn new_disabled() -> Self {
        Self::new(DebugLogType::None, false)
    }

    pub fn new(log_type: DebugLogType, serial_enabled: bool) -> Self {
        Debugger {
            serial_bytes: vec![],
            log_type,
            serial_enabled,
        }
    }

    pub fn print(&mut self, cpu: &mut Cpu) {
        self.print_gb_doctor(cpu);
        self.print_asm(cpu);
    }

    pub fn print_serial(&self) {
        if !self.get_serial_msg().is_empty() {
            log::info!("Serial: {}", self.get_serial_msg());
        }
    }

    pub fn get_serial_msg<'a>(&'a self) -> Cow<'a, str> {
        String::from_utf8_lossy(&self.serial_bytes[..])
    }

    pub fn poll_serial(&mut self, bus: &mut Bus) {
        if self.serial_enabled && bus.io.serial.has_data() {
            self.serial_bytes.push(bus.io.serial.take_data());
        }
    }

    pub fn print_gb_doctor(&self, cpu: &mut Cpu) {
        if self.log_type != DebugLogType::GbDoc {
            return;
        }

        let pc_mem = format!(
            "PCMEM:{:02X},{:02X},{:02X},{:02X}",
            cpu.clock.bus.read(cpu.registers.pc),
            cpu.clock.bus.read(cpu.registers.pc.wrapping_add(1)),
            cpu.clock.bus.read(cpu.registers.pc.wrapping_add(2)),
            cpu.clock.bus.read(cpu.registers.pc.wrapping_add(3))
        );
        println!(
            "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} {}",
            cpu.registers.a,
            cpu.registers.flags.get_byte(),
            cpu.registers.b,
            cpu.registers.c,
            cpu.registers.d,
            cpu.registers.e,
            cpu.registers.h,
            cpu.registers.l,
            cpu.registers.sp,
            cpu.registers.pc,
            pc_mem
        );
    }

    pub fn print_asm(&self, cpu: &mut Cpu) {
        if self.log_type != DebugLogType::Asm {
            return;
        }

        let opcode = peek_imm8(cpu, 0);
        let instruction = Instruction::get_by_opcode(opcode);
        let pc = cpu.registers.pc;

        log::info!(
            "{:08} - {:04X}: {:<20} ({:02X}) A: {:02X} F: {} BC: {:02X}{:02X} DE: {:02X}{:02X} HL: {:02X}{:02X}",
            cpu.clock.get_t_cycles(),
            pc,
            get_asm_string(cpu, instruction),
            opcode,
            cpu.registers.a,
            cpu.registers.flags.display(),
            cpu.registers.b,
            cpu.registers.c,
            cpu.registers.d,
            cpu.registers.e,
            cpu.registers.h,
            cpu.registers.l
        );
    }
}

pub fn get_asm_string(cpu: &Cpu, instruction: &Instruction) -> String {
    let mode = instruction.get_address_mode();
    let mnemonic = instruction.get_mnemonic();
    let cond = instruction.get_condition().unwrap_or(JumpCondition::None);
    let cond = match cond {
        JumpCondition::None => "".to_string(),
        _ => format!(" {cond:?}"),
    };
    let byte1 = peek_imm8(cpu, 1);
    let byte2 = peek_imm8(cpu, 2);
    let imm16 = u16::from_be_bytes([byte1, byte2]);
    let str = match mode {
        AddressMode::IMP => format!("{mnemonic:?}{cond}"),
        AddressMode::R_D16(r1) => {
            format!("{mnemonic:?}{cond} {r1:?},${:04X}", imm16)
        }
        AddressMode::R_A16(r1) => {
            format!("{mnemonic:?}{cond} {r1:?},(${:04X})", imm16)
        }
        AddressMode::R(r1) => {
            format!("{mnemonic:?}{cond} {r1:?}")
        }
        AddressMode::R_R(r1, r2) => {
            format!("{mnemonic:?}{cond} {r1:?},{r2:?}")
        }
        AddressMode::MR_R(r1, r2) => {
            format!("{mnemonic:?}{cond} ({r1:?}),{r2:?}")
        }
        AddressMode::MR(r1) => {
            format!("{mnemonic:?}{cond} ({r1:?})")
        }
        AddressMode::R_MR(r1, r2) => {
            format!("{mnemonic:?}{cond} {r1:?},({r2:?})")
        }
        AddressMode::R_HMR(r1, r2) => {
            format!("{mnemonic:?}{cond} {r1:?},(FF00+{r2:?})")
        }
        AddressMode::R_D8(r1) => {
            format!("{mnemonic:?}{cond} {r1:?},${:02X}", byte1)
        }
        AddressMode::R_HA8(r1) | AddressMode::R_A8(r1) => {
            format!("{mnemonic:?}{cond} {r1:?},(FF00+${:02X})", byte1)
        }
        AddressMode::R_HLI(r1) => {
            format!("{mnemonic:?}{cond} {r1:?},(HL+)")
        }
        AddressMode::R_HLD(r1) => {
            format!("{mnemonic:?}{cond} {r1:?},(HL-)")
        }
        AddressMode::HLI_R(r1) => {
            format!("{mnemonic:?}{cond} (HL+),{r1:?}")
        }
        AddressMode::HLD_R(r1) => {
            format!("{mnemonic:?}{cond} (HL-),{r1:?}")
        }
        AddressMode::A8_R(r2) => {
            format!("{mnemonic:?}{cond} (${:02X}),{r2:?}", byte1)
        }
        AddressMode::LH_SPi8 => {
            format!("{mnemonic:?}{cond} (HL,SP+{:?})", byte1)
        }
        AddressMode::D8 => {
            format!("{mnemonic:?}{cond} ${:02X}", byte1)
        }
        AddressMode::D16 => {
            format!("{mnemonic:?}{cond} ${:04X}", imm16)
        }
        AddressMode::MR_D8(r1) => {
            format!("{mnemonic:?}{cond} ({r1:?}),${:02X}", byte1)
        }
        AddressMode::A16_R(r2) => {
            format!("{mnemonic:?}{cond} (${:04X}),{r2:?}", imm16)
        }
    };

    str.to_uppercase()
}

fn peek_imm8(cpu: &Cpu, offset: u16) -> u8 {
    cpu.clock.bus.read(cpu.registers.pc.wrapping_add(offset))
}
