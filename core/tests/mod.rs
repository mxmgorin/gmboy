use core::auxiliary::clock::{Clock, Tickable};
use core::bus::Bus;
use core::cpu::{Cpu, CpuCallback, DebugCtx};
use core::debugger::Debugger;
use core::ppu::Ppu;

mod blargg;
mod mooneye;
mod sm83;

pub struct EmptyTick;

impl Tickable for EmptyTick {
    fn tick(&mut self, _bus: &mut Bus) {}
}

pub fn print_with_dashes(content: &str) {
    const TOTAL_LEN: usize = 100;
    let content_length = content.len();
    let dashes = "-".repeat(TOTAL_LEN.saturating_sub(content_length));
    println!("{} {}", content, dashes);
}

pub struct TestCpuCtx {
    clock: Clock,
    debugger: Debugger,
}

impl CpuCallback for TestCpuCtx {
    fn m_cycles(&mut self, m_cycles: usize, bus: &mut Bus) {
        self.clock.m_cycles(m_cycles, bus, &mut EmptyTick);
    }

    fn update_serial(&mut self, cpu: &mut Cpu) {
        self.debugger.update_serial(cpu);
    }

    fn debug(&mut self, _cpu: &mut Cpu, _ctx: Option<DebugCtx>) {}
}

pub struct TestCpuCtxWithPPu {
    clock: Clock,
    debugger: Debugger,
    ppu: Ppu,
}

impl CpuCallback for TestCpuCtxWithPPu {
    fn m_cycles(&mut self, m_cycles: usize, bus: &mut Bus) {
        self.clock.m_cycles(m_cycles, bus, &mut self.ppu);
    }

    fn update_serial(&mut self, cpu: &mut Cpu) {
        self.debugger.update_serial(cpu);
    }

    fn debug(&mut self, _cpu: &mut Cpu, _ctx: Option<DebugCtx>) {}
}
