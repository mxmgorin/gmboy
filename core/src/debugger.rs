use crate::bus::Bus;
use crate::cpu::instructions::{FetchedData, Instruction};
use crate::cpu::{Cpu, DebugCtx};
use std::borrow::Cow;
use serde::{Deserialize, Serialize};

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
            self.print_cpu_info(
                cpu,
                ctx.pc,
                &ctx.instruction,
                ctx.opcode,
                &ctx.fetched_data,
            );
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

        log::info!(
            "{:08} - {:04X}: {:<20} ({:02X} {:02X} {:02X}) A: {:02X} F: {} BC: {:02X}{:02X} DE: {:02X}{:02X} HL: {:02X}{:02X}",
            cpu.clock.t_cycles,
            pc,
            instruction.to_asm_string(cpu, fetched_data),
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
