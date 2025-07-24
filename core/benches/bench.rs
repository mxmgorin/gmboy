use core::apu::Apu;
use core::auxiliary::timer::Timer;
use core::bus::Bus;
use core::cart::Cart;
use core::cpu::instructions::{
    AddressMode, ExecutableInstruction, Instruction, INSTRUCTIONS_BY_OPCODES,
};
use core::cpu::interrupts::Interrupts;
use core::cpu::{CounterCpuCallback, Cpu};
use core::emu::read_bytes;
use core::ppu::Ppu;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::path::PathBuf;

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

pub fn get_cart() -> Cart {
    let path = PathBuf::from("benches").join("roms").join("dmg-acid2.gb");

    Cart::new(read_bytes(&path).unwrap()).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("timer_tick_5_000_000", |b| {
        b.iter_batched(
            || {
                let timer = Timer::default();
                let interrupts = Interrupts::default();
                (timer, interrupts)
            },
            |(mut timer, mut interrupts)| {
                for _ in 0..5_000_000 {
                    timer.tick(&mut interrupts);
                }
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("ppu_tick_5_000_000", |b| {
        b.iter_batched(
            || {
                let ppu = Ppu::default();
                let bus = Bus::new(get_cart(), Default::default());
                (ppu, bus)
            },
            |(mut ppu, mut bus)| {
                for _ in 0..5_000_000 {
                    ppu.tick(&mut bus);
                }
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("apu_tick_5_000_000", |b| {
        b.iter_batched(
            Apu::default,
            |mut apu| {
                for _ in 0..5_000_000 {
                    apu.tick();
                }
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
