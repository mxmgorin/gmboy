use criterion::{criterion_group, criterion_main, Criterion};
use rusty_gb_emu::bus::Bus;
use rusty_gb_emu::cpu::instructions::{Instruction, INSTRUCTIONS_BY_OPCODES};
use rusty_gb_emu::cpu::Cpu;
use rusty_gb_emu::emu::EmuCtx;

pub fn instructions() {
    let mut cpu = Cpu::new(Bus::flat_mem(vec![0; 100000]));

    for (opcode, instr) in INSTRUCTIONS_BY_OPCODES.iter().enumerate() {
        match instr {
            Instruction::Halt(_)
            | Instruction::Di(_)
            | Instruction::Stop(_)
            | Instruction::Unknown(_) => continue,
            _ => (),
        }

        cpu.set_pc(0);
        cpu.bus.write(0, opcode as u8);
        cpu.step(&mut EmuCtx::new()).unwrap();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("instructions", |b| b.iter(instructions));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
