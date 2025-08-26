use std::hint::black_box;
use core::apu::Apu;
use core::auxiliary::clock::Clock;
use core::auxiliary::timer::Timer;
use core::bus::Bus;
use core::cart::Cart;
use core::cpu::interrupts::Interrupts;
use core::cpu::Cpu;
use core::ppu::oam::OamRam;
use core::ppu::oam::OAM_ADDR_START;
use core::ppu::oam::OAM_ENTRIES_COUNT;
use core::ppu::vram::VideoRam;
use core::ppu::vram::VRAM_ADDR_START;
use core::ppu::vram::VRAM_SIZE;
use core::ppu::Ppu;
use core::read_bytes;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::path::PathBuf;

pub fn get_cart() -> Cart {
    let path = PathBuf::from("benches").join("roms").join("cpu_instrs.gb");

    Cart::new(read_bytes(&path).unwrap()).unwrap()
}

pub fn new_cpu(cart: Option<Cart>) -> Cpu {
    let bus = if let Some(cart) = cart {
        Bus::new(cart, Default::default())
    } else {
        Bus::with_bytes(vec![0; 100000], Default::default())
    };
    let clock = Clock::new(Ppu::default(), bus);

    Cpu::new(clock)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("daa", |b| {
        let mut cpu = new_cpu(None);

        b.iter(|| {
            // test all possible A + flags combinations
            for a in 0..=255u8 {
                for n in 0..=1 {
                    for h in 0..=1 {
                        for c in 0..=1 {
                            cpu.registers.a = black_box(a);
                            cpu.registers.flags.set_n_raw(black_box(n) != 0);
                            cpu.registers.flags.set_h_raw(black_box(h) != 0);
                            cpu.registers.flags.set_c_raw(black_box(c) != 0);

                            cpu.execute_daa();
                        }
                    }
                }
            }
        });
    });

    c.bench_function("fetch_execute_prefix_500_000", |b| {
        b.iter_batched(
            || {
                let ppu = Ppu::default();
                let bus = Bus::new(get_cart(), Default::default());
                let clock = Clock::new(ppu, bus);

                Cpu::new(clock)
            },
            |mut cpu| {
                for _ in 0..50_000 {
                    cpu.fetch_execute_cb();
                    cpu.registers.pc += 1;
                }
            },
            BatchSize::LargeInput,
        );
    });

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
            BatchSize::LargeInput,
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
                    oam.write(
                        (i % OAM_ENTRIES_COUNT * 4) as u16 + OAM_ADDR_START,
                        (i & 0xFF) as u8,
                    );
                }
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("vram_write_5_000_000", |b| {
        b.iter_batched(
            VideoRam::default,
            |mut vram| {
                for i in 0..5_000_000 {
                    vram.write((i % VRAM_SIZE) as u16 + VRAM_ADDR_START, (i & 0xFF) as u8);
                }
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
