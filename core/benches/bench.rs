use core::cart::Cart;
use core::auxiliary::timer::Timer;
use core::bus::Bus;
use core::cpu::instructions::{
    AddressMode, ExecutableInstruction, Instruction, INSTRUCTIONS_BY_OPCODES,
};
use core::cpu::interrupts::Interrupts;
use core::cpu::{CounterCpuCallback, Cpu};
use core::ppu::Ppu;
use criterion::{criterion_group, criterion_main, Criterion};

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
        ctx.bus.write(0, opcode as u8);
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
    let cart = Cart::new(vec![0; 100000].into_boxed_slice()).unwrap();
    let mut bus = Bus::new(cart, Default::default());
    let mut callback = CounterCpuCallback {
        m_cycles_count: 0,
        bus: bus.clone()
    };
    let mut cpu = Cpu::default();
    let mut timer = Timer::default();

    c.bench_function("timer_tick", |b| {
        b.iter(|| timer_tick(&mut timer, &mut callback.bus.io.interrupts))
    });
    c.bench_function("fetch_data", |b| {
        b.iter(|| fetch_data(&mut cpu, &mut callback))
    });
    c.bench_function("execute", |b| b.iter(|| execute(&mut cpu, &mut callback)));
    c.bench_function("cpu_step", |b| {
        b.iter(|| instructions(&mut cpu, &mut callback))
    });

    let mut ppu = Ppu::default();
    c.bench_function("ppu_tick", |b| b.iter(|| ppu_tick(&mut ppu, &mut bus)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
