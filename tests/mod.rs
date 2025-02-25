use gmboy::auxiliary::clock::Clock;
use gmboy::bus::Bus;
use gmboy::debugger::Debugger;
use gmboy::{Cpu, CpuCallback, DebugCtx};

mod blargg;
mod mooneye;
mod sm83;

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
        self.clock.m_cycles(m_cycles, bus);
    }

    fn update_serial(&mut self, cpu: &mut Cpu) {
        self.debugger.update_serial(cpu);
    }

    fn debug(&mut self, _cpu: &mut Cpu, _ctx: Option<DebugCtx>) {}
}
