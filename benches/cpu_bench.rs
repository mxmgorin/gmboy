use criterion::{criterion_group, criterion_main, Criterion};
use rusty_gb_emu::bus::Bus;
use rusty_gb_emu::cpu::instructions::{
    AddressMode, ExecutableInstruction, Instruction, INSTRUCTIONS_BY_OPCODES,
};
use rusty_gb_emu::cpu::Cpu;
use rusty_gb_emu::emu::EmuCtx;

pub fn instructions(cpu: &mut Cpu, ctx: &mut EmuCtx) {
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

pub fn fetch_data(cpu: &mut Cpu) {
    for instr in INSTRUCTIONS_BY_OPCODES.iter() {
        if let Instruction::Unknown(_) = instr {
            continue;
        }

        cpu.registers.pc = 0;
        AddressMode::fetch_data(cpu, instr.get_address_mode());
    }
}

pub fn execute(cpu: &mut Cpu) {
    for instr in INSTRUCTIONS_BY_OPCODES.iter() {
        if let Instruction::Unknown(_) = instr {
            continue;
        }

        cpu.registers.pc = 0;
        let fetched_data = AddressMode::fetch_data(cpu, instr.get_address_mode());
        instr.execute(cpu, fetched_data);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut ctx = EmuCtx::new();
    let mut cpu = Cpu::new(Bus::with_bytes(vec![10; 100000])); // Pre-allocate memory

    c.bench_function("fetch data", |b| b.iter(|| fetch_data(&mut cpu)));
    c.bench_function("execute", |b| b.iter(|| execute(&mut cpu)));
    c.bench_function("cpu step", |b| b.iter(|| instructions(&mut cpu, &mut ctx)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
