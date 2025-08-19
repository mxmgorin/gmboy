use crate::bus::Bus;
use crate::cpu::instructions::{AddressMode, FetchedData, Instruction, Mnemonic};
use crate::cpu::{Cpu, DebugCtx};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Debugger {
    msg: Vec<u8>,
    size: usize,
    cpu_log_type: CpuLogType,
    serial_enabled: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum CpuLogType {
    None,
    Assembly,
    GbDoctor,
}

impl Debugger {
    pub fn disabled() -> Self {
        Self::new(CpuLogType::None, false)
    }

    pub fn new(log_type: CpuLogType, serial_enabled: bool) -> Self {
        Debugger {
            msg: vec![0; 1024],
            size: 0,
            cpu_log_type: log_type,
            serial_enabled,
        }
    }

    pub fn print(&mut self, cpu: &mut Cpu, ctx: Option<DebugCtx>) {
        self.print_gb_doctor_info(cpu);

        if let Some(ctx) = ctx {
            self.print_cpu_info(cpu, ctx.pc, &ctx.instruction, ctx.opcode, &ctx.fetched_data);
        }
    }

    pub fn print_serial(&self) {
        if !self.get_serial_msg().is_empty() {
            log::info!("Serial: {}", self.get_serial_msg());
        }
    }

    pub fn get_serial_msg(&self) -> Cow<str> {
        String::from_utf8_lossy(&self.msg[..self.size])
    }

    pub fn update_serial(&mut self, bus: &mut Bus) {
        if self.serial_enabled && bus.io.serial.has_data() {
            self.msg[self.size] = bus.io.serial.take_data();
            self.size += 1;
        }
    }

    fn print_gb_doctor_info(&self, cpu: &Cpu) {
        if self.cpu_log_type != CpuLogType::GbDoctor {
            return;
        }

        let pc_mem = format!(
            "PCMEM:{:02X},{:02X},{:02X},{:02X}",
            cpu.clock.bus.read(cpu.registers.pc),
            cpu.clock.bus.read(cpu.registers.pc.wrapping_add(1)),
            cpu.clock.bus.read(cpu.registers.pc.wrapping_add(2)),
            cpu.clock.bus.read(cpu.registers.pc.wrapping_add(3))
        );
        log::info!(
            "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} {}",
            cpu.registers.a,
            cpu.registers.flags.byte,
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

    fn print_cpu_info(
        &self,
        cpu: &Cpu,
        pc: u16,
        instruction: &Instruction,
        opcode: u8,
        fetched_data: &FetchedData,
    ) {
        if self.cpu_log_type != CpuLogType::Assembly {
            return;
        }
        let mode = instruction.get_address_mode();
        let mnemonic = instruction.get_mnemonic();

        log::info!(
            "{:08} - {:04X}: {:<20} ({:02X} {:02X} {:02X}) A: {:02X} F: {} BC: {:02X}{:02X} DE: {:02X}{:02X} HL: {:02X}{:02X}",
            cpu.clock.get_t_cycles(),
            pc,
            get_asm_string(mode, mnemonic, cpu, fetched_data),
            opcode,
            cpu.clock.bus.read(pc.wrapping_add(1)),
            cpu.clock.bus.read(pc.wrapping_add(2)),
            cpu.registers.a,
            cpu.registers.flags,
            cpu.registers.b,
            cpu.registers.c,
            cpu.registers.d,
            cpu.registers.e,
            cpu.registers.h,
            cpu.registers.l
        );
    }
}

pub fn get_asm_string(
    mode: AddressMode,
    mnemonic: Mnemonic,
    cpu: &Cpu,
    fetched_data: &FetchedData,
) -> String {
    match mode {
        AddressMode::IMP => format!("{:?}", mnemonic),
        AddressMode::R_D16(r1) | AddressMode::R_A16(r1) => {
            format!("{:?} {:?},${:04X}", mnemonic, r1, fetched_data.value)
        }
        AddressMode::R(r1) => {
            format!("{:?} {:?}", mnemonic, r1)
        }
        AddressMode::R_R(r1, r2) => {
            format!("{:?} {:?},{:?}", mnemonic, r1, r2)
        }
        AddressMode::MR_R(r1, r2) => {
            format!("{:?} ({:?}),{:?}", mnemonic, r1, r2)
        }
        AddressMode::MR(r1) => {
            format!("{:?} ({:?})", mnemonic, r1)
        }
        AddressMode::R_MR(r1, r2) => {
            format!("{:?} {:?},({:?})", mnemonic, r1, r2)
        }
        AddressMode::R_HMR(r1, r2) => {
            format!("{:?} {:?},(FF00+{:?})", mnemonic, r1, r2)
        }
        AddressMode::R_D8(r1) | AddressMode::R_A8(r1) | AddressMode::R_HA8(r1) => {
            format!("{:?} {:?},${:02X}", mnemonic, r1, fetched_data.value & 0xFF)
        }
        AddressMode::R_HLI(r1) => {
            format!("{:?} {:?},(HL+)", mnemonic, r1)
        }
        AddressMode::R_HLD(r1) => {
            format!("{:?} {:?},(HL-)", mnemonic, r1)
        }
        AddressMode::HLI_R(r1) => {
            format!("{:?} (HL+),{:?}", mnemonic, r1)
        }
        AddressMode::HLD_R(r1) => {
            format!("{:?} (HL-),{:?}", mnemonic, r1)
        }
        AddressMode::A8_R(r2) => {
            format!(
                "{:?} ${:02X},{:?}",
                mnemonic,
                cpu.clock.bus.read(cpu.registers.pc - 1),
                r2
            )
        }
        AddressMode::LH_SPi8 => {
            format!("{:?} (HL,SP+{:?})", mnemonic, fetched_data.value & 0xFF)
        }
        AddressMode::D8 => {
            format!("{:?} ${:02X}", mnemonic, fetched_data.value & 0xFF)
        }
        AddressMode::D16 => {
            format!("{:?} ${:04X}", mnemonic, fetched_data.value)
        }
        AddressMode::MR_D8(r1) => {
            format!(
                "{:?} ({:?}),${:02X}",
                mnemonic,
                r1,
                fetched_data.value & 0xFF
            )
        }
        AddressMode::A16_R(r2) => {
            format!("{:?} (${:04X}),{:?}", mnemonic, fetched_data.value, r2)
        }
    }
}
