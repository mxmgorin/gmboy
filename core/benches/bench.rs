use core::ppu::oam::OAM_ADDR_START;
use core::apu::Apu;
use core::auxiliary::timer::Timer;
use core::bus::Bus;
use core::cart::Cart;
use core::cpu::interrupts::Interrupts;
use core::ppu::oam::OamRam;
use core::ppu::oam::OAM_ENTRIES_COUNT;
use core::ppu::Ppu;
use core::read_bytes;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::path::PathBuf;

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

    c.bench_function("apu_push_buffer_5_000_000", |b| {
        b.iter_batched(
            Apu::default,
            |mut apu| {
                for i in 0..5_000_000 {
                    apu.push_buffer(i as f32, i as f32);
                }
            },
            BatchSize::SmallInput,
        );
    });

    let oam = OamRam::default();

    c.bench_function("oam_read_5_000_000", |b| {
        b.iter_batched(
            || oam.clone(),
            |oam| {
                for i in 0..5_000_000 {
                    let _ = oam.read((i % (OAM_ENTRIES_COUNT * 4)) as u16 + OAM_ADDR_START);
                }
            },
            BatchSize::SmallInput,
        );
    });

    let oam = OamRam::default();

    c.bench_function("oam_write_5_000_000", |b| {
        b.iter_batched(
            || oam.clone(),
            |mut oam| {
                for i in 0..5_000_000 {
                    oam.write((i % OAM_ENTRIES_COUNT * 4) as u16 + OAM_ADDR_START, (i & 0xFF) as u8);
                }
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
