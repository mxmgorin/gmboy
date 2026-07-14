//! CPU/memory inspection helpers for debugging test ROMs: register snapshot,
//! memory hex dump, and an instruction tracer.

use core::cpu::Cpu;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Print CPU registers, interrupt state, and the four opcode bytes at PC — a
/// snapshot handy for locating where a ROM is stuck when it hangs/times out.
pub fn dump_regs(cpu: &mut Cpu) {
    let f = cpu.registers.flags.get_byte();
    let af = ((cpu.registers.a as u16) << 8) | f as u16;
    let bc = u16::from_be_bytes([cpu.registers.b, cpu.registers.c]);
    let de = u16::from_be_bytes([cpu.registers.d, cpu.registers.e]);
    let hl = u16::from_be_bytes([cpu.registers.h, cpu.registers.l]);
    let sp = cpu.registers.sp;
    let pc = cpu.registers.pc;
    let ime = cpu.clock.bus.io.interrupts.ime as u8;
    let ie = cpu.clock.bus.io.interrupts.ie;
    let iflag = cpu.clock.bus.io.interrupts.int_flags;
    let ops = [
        cpu.clock.bus.read(pc),
        cpu.clock.bus.read(pc.wrapping_add(1)),
        cpu.clock.bus.read(pc.wrapping_add(2)),
        cpu.clock.bus.read(pc.wrapping_add(3)),
    ];

    println!(
        "regs: AF={af:04X} BC={bc:04X} DE={de:04X} HL={hl:04X} SP={sp:04X} PC={pc:04X}  \
         IME={ime} IE={ie:02X} IF={iflag:02X}"
    );
    println!(
        "  @PC: {:02X} {:02X} {:02X} {:02X}",
        ops[0], ops[1], ops[2], ops[3]
    );
}

/// Print a hex dump of `len` bytes starting at `addr`, as seen through the bus
/// (so memory-mapping/PPU-mode blocking applies, just like the CPU sees it).
pub fn dump_memory(cpu: &Cpu, addr: u16, len: u16) {
    for row in (0..len).step_by(16) {
        let base = addr.wrapping_add(row);
        print!("{base:04X}:");

        for col in 0..16 {
            if row + col >= len {
                break;
            }
            print!(" {:02X}", cpu.clock.bus.read(base.wrapping_add(col)));
        }

        println!();
    }
}

// A self-loop (an instruction that jumps to itself, e.g. `JR -2`) has to repeat
// this many times before we call it a hang — one repeat can be a transient
// (e.g. a HALT that rewinds PC to service a pending interrupt next step).
const SELF_LOOP_HANG: u32 = 4;

/// Run `cpu` while keeping a ring buffer of the last `len` executed instructions,
/// then print it. The buffer freezes on a detected self-loop so it captures the
/// path *into* a hang instead of `len` copies of the loop; otherwise it prints
/// the last `len` instructions at `timeout`. A debugging aid, not a pass/fail run.
pub fn trace(cpu: &mut Cpu, timeout: Duration, len: usize) {
    let mut ring: VecDeque<(u16, [u8; 3])> = VecDeque::with_capacity(len);
    let start = Instant::now();
    let mut steps: u64 = 0;
    let mut self_loops: u32 = 0;
    let mut hung = false;

    loop {
        let pc = cpu.registers.pc;
        let ops = [
            cpu.clock.bus.read(pc),
            cpu.clock.bus.read(pc.wrapping_add(1)),
            cpu.clock.bus.read(pc.wrapping_add(2)),
        ];
        if ring.len() == len {
            ring.pop_front();
        }
        ring.push_back((pc, ops));

        cpu.step();
        steps += 1;

        // HALT / STOP freeze PC while idly waiting; that's not a hang, and
        // recording every stall cycle would flush the ring. Drop the entry and
        // reset the counter for those.
        let idle = cpu.clock.cpu_halted || cpu.stop_m_cycles > 0;
        if cpu.registers.pc == pc {
            if idle {
                ring.pop_back();
                self_loops = 0;
            } else {
                self_loops += 1;
                if self_loops >= SELF_LOOP_HANG {
                    hung = true;
                    break;
                }
            }
        } else {
            self_loops = 0;
        }

        if steps.is_multiple_of(4096) && start.elapsed() > timeout {
            break;
        }
    }

    let reason = if hung { "self-loop (hang)" } else { "timeout" };
    println!(
        "--- trace: {} instr, {steps} steps, stopped on {reason} ---",
        ring.len()
    );
    for (pc, ops) in &ring {
        println!("{pc:04X}: {:02X} {:02X} {:02X}", ops[0], ops[1], ops[2]);
    }
}
