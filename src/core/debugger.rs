use crate::core::cpu::instructions::common::Instruction;
use crate::core::cpu::{Cpu, FetchedData};

#[cfg(debug_assertions)]
#[derive(Debug, Clone)]
pub struct Debugger {
    msg: [u8; 1024],
    size: usize,
}

#[cfg(debug_assertions)]
impl Debugger {
    pub fn new() -> Self {
        Debugger {
            msg: [0; 1024],
            size: 0,
        }
    }

    pub fn update(&mut self, cpu: &mut Cpu) {
        if cpu.bus.io.serial.has_data() {
            self.msg[self.size] = cpu.bus.io.serial.take_data();
            self.size += 1;
        }
    }

    pub fn print(&self) {
        if self.msg[0] != 0 {
            let msg_str = String::from_utf8_lossy(&self.msg[..self.size]);
            println!("DBG: {:?}", msg_str);
        }
    }

    #[cfg(debug_assertions)]
    pub fn print_cpu_info(
        &self,
        cpu: &Cpu,
        pc: u16,
        instruction: &Instruction,
        opcode: u8,
        fetched_data: &FetchedData,
    ) {
        println!(
            "{:08X} - {:04X}: {:<20} ({:02X} {:02X} {:02X}) A: {:02X} F: {} BC: {:02X}{:02X} DE: {:02X}{:02X} HL: {:02X}{:02X}",
            cpu.ticks,
            pc,
            instruction.to_asm_string(cpu, fetched_data),
            opcode,
            cpu.bus.read(pc.wrapping_add(1)),
            cpu.bus.read(pc.wrapping_add(2)),
            cpu.registers.a,
            cpu.registers.flags_to_string(),
            cpu.registers.b,
            cpu.registers.c,
            cpu.registers.d,
            cpu.registers.e,
            cpu.registers.h,
            cpu.registers.l
        );
    }
}
