use criterion::{criterion_group, criterion_main, Criterion};
use gmboy::auxiliary::timer::Timer;
use gmboy::bus::Bus;
use gmboy::cpu::instructions::{
    AddressMode, ExecutableInstruction, Instruction, INSTRUCTIONS_BY_OPCODES,
};
use gmboy::cpu::interrupts::Interrupts;
use gmboy::cpu::{CounterCpuCallback, Cpu};
use gmboy::ppu::Ppu;

pub fn instructions(cpu: &mut Cpu, ctx: &mut CounterCpuCallback) {
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
        cpu.step(ctx).unwrap();
    }
}

pub fn fetch_data(cpu: &mut Cpu, callback: &mut CounterCpuCallback) {
    for instr in INSTRUCTIONS_BY_OPCODES.iter() {
        if let Instruction::Unknown(_) = instr {
            continue;
        }

        cpu.registers.pc = 0;
        AddressMode::fetch_data(cpu, instr.get_address_mode(), callback);
    }
}

pub fn execute(cpu: &mut Cpu, callback: &mut CounterCpuCallback) {
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

pub fn ppu_tick(ppu: &mut Ppu, bus: &mut Bus) {
    for _ in 0..1000 {
        ppu.tick(bus);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut callback = CounterCpuCallback::default();
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
    
    let mut bus = Bus::with_bytes(vec![10; 100000]);
    let mut ppu = Ppu::default();
    c.bench_function("ppu tick", |b| {
        b.iter(|| ppu_tick(&mut ppu, &mut bus))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
