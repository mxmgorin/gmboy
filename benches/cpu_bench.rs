use criterion::{criterion_group, criterion_main, Criterion};
use rusty_gb_emu::auxiliary::timer::Timer;
use rusty_gb_emu::bus::Bus;
use rusty_gb_emu::cpu::instructions::{
    AddressMode, ExecutableInstruction, Instruction, INSTRUCTIONS_BY_OPCODES,
};
use rusty_gb_emu::cpu::interrupts::Interrupts;
use rusty_gb_emu::cpu::{Cpu, CpuCallback};

pub fn instructions(cpu: &mut Cpu, ctx: &mut Callback) {
    for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
        match instr {
            Instruction::Halt(_)
            | Instruction::Di(_)
            | Instruction::Stop(_)
            | Instruction::Unknown(_) => continue,
            _ => (),
        }

        cpu.registers.pc = 0;
        cpu.bus.write(0, opcode as u8);
        cpu.step(ctx, None).unwrap();
    }
}

pub fn fetch_data(cpu: &mut Cpu, callback: &mut Callback) {
    for instr in INSTRUCTIONS_BY_OPCODES.iter() {
        if let Instruction::Unknown(_) = instr {
            continue;
        }

        cpu.registers.pc = 0;
        AddressMode::fetch_data(cpu, instr.get_address_mode(), callback);
    }
}

pub fn execute(cpu: &mut Cpu, callback: &mut Callback) {
    for instr in INSTRUCTIONS_BY_OPCODES.iter() {
        if let Instruction::Unknown(_) = instr {
            continue;
        }

        cpu.registers.pc = 0;
        let fetched_data = AddressMode::fetch_data(cpu, instr.get_address_mode(), callback);
        instr.execute(cpu, callback, fetched_data);
    }
}

pub fn timer_tick(timer: &mut Timer, interrupts: &mut Interrupts) {
    for _ in 0..1000 {
        timer.tick(interrupts);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut callback = Callback;
    let mut cpu = Cpu::new(Bus::with_bytes(vec![10; 100000])); // Pre-allocate memory
    let mut timer = Timer::default();

    c.bench_function("timer tick", |b| {
        b.iter(|| timer_tick(&mut timer, &mut cpu.bus.io.interrupts))
    });
    c.bench_function("fetch data", |b| {
        b.iter(|| fetch_data(&mut cpu, &mut callback))
    });
    c.bench_function("execute", |b| b.iter(|| execute(&mut cpu, &mut callback)));
    c.bench_function("cpu step", |b| {
        b.iter(|| instructions(&mut cpu, &mut callback))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub struct Callback;

impl CpuCallback for Callback {
    fn m_cycles(&mut self, _m_cycles: usize, _bus: &mut Bus) {}
}
