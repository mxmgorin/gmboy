use crate::cpu::jit::x64::opcode::{compile_ld_r_d8, compile_ld_r_r};
use crate::cpu::jit::{
    get_ld_r_d8_dst, get_ld_r_r_dst, get_ld_r_r_src, is_control_flow, is_ld_r_d8, is_ld_r_r,
    REGISTER_OFFSETS,
};
use crate::cpu::{Cpu, RegisterType};
use dynasm::dynasm;
use dynasmrt::{x64::Assembler, AssemblyOffset, DynasmApi, ExecutableBuffer};
use std::collections::HashMap;

const MAX_BLOCK_LEN: usize = 10;

type CompiledBlockFn = unsafe extern "sysv64" fn(*mut Cpu) -> u32;

pub struct CompiledBlock {
    pub fn_ptr: CompiledBlockFn,
    _buf: ExecutableBuffer,
    pub cycles: usize,
}

pub struct JitCompiler {
    cache: HashMap<u16, CompiledBlock>,
    fns: [Option<unsafe extern "sysv64" fn(*mut Cpu)>; 256],
    _bufs: Vec<ExecutableBuffer>,
}

impl Default for JitCompiler {
    fn default() -> Self {
        let mut jit = JitCompiler {
            cache: Default::default(),
            fns: [None; 256],
            _bufs: Vec::new(),
        };

        jit.emit_fns();

        jit
    }
}

impl JitCompiler {
    fn emit_ld_r_r(&mut self, opcode: u8, dst: RegisterType, src: RegisterType) {
        let mut ops = Assembler::new().unwrap();
        compile_ld_r_r(&mut ops, dst, src);
        let offset = AssemblyOffset(0); // no need to offset for buffer with one fn
        let buf = ops.finalize().unwrap();
        let fn_ptr: unsafe extern "sysv64" fn(*mut Cpu) =
            unsafe { std::mem::transmute(buf.ptr(offset)) }; //

        self.fns[opcode as usize] = Some(fn_ptr);
        self._bufs.push(buf); // store to keep executable memory alive
    }

    #[inline(always)]
    pub fn execute_opcode(&self, cpu: &mut Cpu) -> bool {
        let opcode = cpu.step_ctx.opcode as usize;

        unsafe {
            if let Some(f) = self.fns.get_unchecked(opcode) {
                f(cpu as *mut Cpu);
                true
            } else {
                false
            }
        }
    }

    /// Attempts to execute a compiled block at cpu.pc.
    /// Returns true if JIT ran, false => interpreter should run one opcode.
    pub fn try_exec_block(&mut self, cpu: &mut Cpu) -> bool {
        let pc = cpu.registers.pc;

        if let Some(block) = self.cache.get(&pc) {
            unsafe { (block.fn_ptr)(cpu as *mut Cpu) };
            cpu.clock.tick_m_cycles(block.cycles);
            return true;
        }

        if let Some(block) = self.compile_block(cpu, pc) {
            unsafe { (block.fn_ptr)(cpu as *mut Cpu) };
            cpu.clock.tick_m_cycles(block.cycles);
            self.cache.insert(pc, block);
            return true;
        }

        false
    }

    fn compile_block(&self, cpu: &Cpu, start_pc: u16) -> Option<CompiledBlock> {
        let mut ops = Assembler::new().ok()?;
        let mut pc = start_pc;
        let mut cycles = 0;
        let mut count = 0;

        // Prologue (SysV64): &mut Cpu in rdi. We'll set PC at the end and return cycles.
        // Body emits straight-line ops.
        loop {
            if count >= MAX_BLOCK_LEN {
                break;
            }

            let opcode = cpu.clock.bus.read(cpu.registers.pc);

            if is_control_flow(opcode) {
                break;
            }

            if is_ld_r_r(opcode) {
                let dst = get_ld_r_r_dst(opcode);
                let src = get_ld_r_r_src(opcode);

                let (Some(dst), Some(src)) = (dst, src) else {
                    break;
                };

                compile_ld_r_r(&mut ops, dst, src);
                pc = pc.wrapping_add(1);
                cycles += 4; // LD r,r is 4 cycles
                count += 1;
                continue;
            }

            if is_ld_r_d8(opcode) {
                let imm = cpu.clock.bus.read(cpu.registers.pc.wrapping_add(1));
                let dst = get_ld_r_d8_dst(opcode);
                compile_ld_r_d8(&mut ops, dst, imm);
                pc = pc.wrapping_add(2);
                cycles += 8; // LD r,d8 is 8 cycles
                count += 1;
                continue;
            }

            break;
        }

        // If we didn’t compile anything, bail out
        if count == 0 {
            return None;
        }

        // Epilogue: write next PC, set return value (cycles)
        //   store pc -> Cpu.registers.pc (u16)
        //   move cycles into eax and ret (SysV: 32-bit return in eax)
        let next_pc = pc;

        dynasm!(ops
            ; .arch x64
            ; mov ax, WORD next_pc as i16
            ; mov [rdi + REGISTER_OFFSETS[RegisterType::PC as usize] as i32], ax
            ; mov eax, cycles as i32
            ; ret
        );

        let buf = ops.finalize().ok()?;
        let fn_ptr: CompiledBlockFn = unsafe { std::mem::transmute(buf.ptr(AssemblyOffset(0))) };

        Some(CompiledBlock {
            _buf: buf,
            fn_ptr,
            cycles,
        })
    }

    fn emit_fns(&mut self) {
        // 0x4X
        self.emit_ld_r_r(0x40, RegisterType::B, RegisterType::B);
        self.emit_ld_r_r(0x41, RegisterType::B, RegisterType::C);
        self.emit_ld_r_r(0x42, RegisterType::B, RegisterType::D);
        self.emit_ld_r_r(0x43, RegisterType::B, RegisterType::E);
        self.emit_ld_r_r(0x44, RegisterType::B, RegisterType::H);
        self.emit_ld_r_r(0x45, RegisterType::B, RegisterType::L);
        //self.emit_ld_r_r(0x46] =
        //    Cpu::fetch_execute_ld_r_mr::<{ RegisterType::B, RegisterType::HL);
        self.emit_ld_r_r(0x47, RegisterType::B, RegisterType::A);
        self.emit_ld_r_r(0x48, RegisterType::C, RegisterType::B);
        self.emit_ld_r_r(0x49, RegisterType::C, RegisterType::C);
        self.emit_ld_r_r(0x4A, RegisterType::C, RegisterType::D);
        self.emit_ld_r_r(0x4B, RegisterType::C, RegisterType::E);
        self.emit_ld_r_r(0x4C, RegisterType::C, RegisterType::H);
        self.emit_ld_r_r(0x4D, RegisterType::C, RegisterType::L);
        //self.emit_ld_r_r(0x4E] =
        //    Cpu::fetch_execute_ld_r_mr::<{ RegisterType::C, RegisterType::HL);
        self.emit_ld_r_r(0x4F, RegisterType::C, RegisterType::A);

        // 0x5X
        self.emit_ld_r_r(0x50, RegisterType::D, RegisterType::B);
        self.emit_ld_r_r(0x51, RegisterType::D, RegisterType::C);
        self.emit_ld_r_r(0x52, RegisterType::D, RegisterType::D);
        self.emit_ld_r_r(0x53, RegisterType::D, RegisterType::E);
        self.emit_ld_r_r(0x54, RegisterType::D, RegisterType::H);
        self.emit_ld_r_r(0x55, RegisterType::D, RegisterType::L);
        //self.emit_ld_r_r(0x56] =
        //    Cpu::fetch_execute_ld_r_mr::<{ RegisterType::D, RegisterType::HL);
        self.emit_ld_r_r(0x57, RegisterType::D, RegisterType::A);
        self.emit_ld_r_r(0x58, RegisterType::E, RegisterType::B);
        self.emit_ld_r_r(0x59, RegisterType::E, RegisterType::C);
        self.emit_ld_r_r(0x5A, RegisterType::E, RegisterType::D);
        self.emit_ld_r_r(0x5B, RegisterType::E, RegisterType::E);
        self.emit_ld_r_r(0x5C, RegisterType::E, RegisterType::H);
        self.emit_ld_r_r(0x5D, RegisterType::E, RegisterType::L);
        //self.emit_ld_r_r(0x5E] =
        //    Cpu::fetch_execute_ld_r_mr::<{ RegisterType::E, RegisterType::HL);
        self.emit_ld_r_r(0x5F, RegisterType::E, RegisterType::A);

        // 0x6X
        self.emit_ld_r_r(0x60, RegisterType::H, RegisterType::B);
        self.emit_ld_r_r(0x61, RegisterType::H, RegisterType::C);
        self.emit_ld_r_r(0x62, RegisterType::H, RegisterType::D);
        self.emit_ld_r_r(0x63, RegisterType::H, RegisterType::E);
        self.emit_ld_r_r(0x64, RegisterType::H, RegisterType::H);
        self.emit_ld_r_r(0x65, RegisterType::H, RegisterType::L);
        //self.emit_ld_r_r(0x66] =
        //    Cpu::fetch_execute_ld_r_mr::<{ RegisterType::H, RegisterType::HL);
        self.emit_ld_r_r(0x67, RegisterType::H, RegisterType::A);
        self.emit_ld_r_r(0x68, RegisterType::L, RegisterType::B);
        self.emit_ld_r_r(0x69, RegisterType::L, RegisterType::C);
        self.emit_ld_r_r(0x6A, RegisterType::L, RegisterType::D);
        self.emit_ld_r_r(0x6B, RegisterType::L, RegisterType::E);
        self.emit_ld_r_r(0x6C, RegisterType::L, RegisterType::H);
        self.emit_ld_r_r(0x6D, RegisterType::L, RegisterType::L);
        //self.emit_ld_r_r(0x6E] =
        //    Cpu::fetch_execute_ld_r_mr::<{ RegisterType::L, RegisterType::HL);
        self.emit_ld_r_r(0x6F, RegisterType::L, RegisterType::A);

        // 0x7X
        self.emit_ld_r_r(0x78, RegisterType::A, RegisterType::B);
        self.emit_ld_r_r(0x79, RegisterType::A, RegisterType::C);
        self.emit_ld_r_r(0x7A, RegisterType::A, RegisterType::D);
        self.emit_ld_r_r(0x7B, RegisterType::A, RegisterType::E);
        self.emit_ld_r_r(0x7C, RegisterType::A, RegisterType::H);
        self.emit_ld_r_r(0x7D, RegisterType::A, RegisterType::L);
        //self.emit_ld_r_r(0x7E] =
        //    Cpu::fetch_execute_ld_r_mr::<{ RegisterType::A, RegisterType::HL);
        self.emit_ld_r_r(0x7F, RegisterType::A, RegisterType::A);
    }
}

#[cfg(test)]
mod tests {
    use crate::auxiliary::clock::Clock;
    use crate::auxiliary::io::Io;
    use crate::bus::Bus;
    use crate::cpu::jit::x64::compiler::JitCompiler;
    use crate::cpu::{Cpu, RegisterType};
    use crate::ppu::Ppu;

    #[test]
    pub fn test_ld_r_r() {
        let jit = JitCompiler::default();
        let mut cpu = Cpu::new(Clock::new(
            Ppu::default(),
            Bus::with_bytes(vec![0; 100000], Io::default()),
        ));
        let expected_val = 10;

        cpu.registers
            .set_register::<{ RegisterType::C as u8 }>(expected_val);
        cpu.step_ctx.opcode = 0x41;
        jit.execute_opcode(&mut cpu);
        let actual_val = cpu.registers.read_register::<{ RegisterType::B as u8 }>();

        assert_eq!(actual_val, expected_val);
    }
}
