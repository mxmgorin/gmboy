use crate::cpu::{Cpu, RegisterType, Registers};
use dynasm::dynasm;
use dynasmrt::{x64::Assembler, AssemblyOffset, DynasmApi, ExecutableBuffer};
use memoffset::offset_of;

pub struct JitX64 {
    fns: [Option<unsafe extern "sysv64" fn(*mut Cpu)>; 256],
    _bufs: Vec<ExecutableBuffer>,
}

impl Default for JitX64 {
    fn default() -> Self {
        let mut jit = JitX64 {
            fns: [None; 256],
            _bufs: Vec::new(),
        };
        
        jit.fill_opcodes();

        jit
    }
}

impl JitX64 {
    fn emit_ld_r_r(&mut self, opcode: u8, dst: RegisterType, src: RegisterType) {
        let mut ops = Assembler::new().unwrap();
        let base_offset = offset_of!(Cpu, registers);
        let dst_offset = base_offset + dst.offset();
        let src_offset = base_offset + src.offset();

        dynasm!(ops
            ; .arch x64
            ; movzx eax, BYTE [rdi + src_offset as i32]
            ; mov    [rdi + dst_offset as i32], al
            ; ret
        );

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
    
    fn fill_opcodes(&mut self) {
        self.emit_ld_r_r(0x40, RegisterType::B, RegisterType::B);
        self.emit_ld_r_r(0x41, RegisterType::B, RegisterType::C);
    }
}

impl RegisterType {
    pub fn offset(&self) -> usize {
        match self {
            RegisterType::A => offset_of!(Registers, a),
            RegisterType::B => offset_of!(Registers, b),
            RegisterType::C => offset_of!(Registers, c),
            RegisterType::D => offset_of!(Registers, d),
            RegisterType::E => offset_of!(Registers, e),
            RegisterType::H => offset_of!(Registers, h),
            RegisterType::L => offset_of!(Registers, l),
            _ => panic!("unsupported register"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::auxiliary::clock::Clock;
    use crate::auxiliary::io::Io;
    use crate::bus::Bus;
    use crate::cpu::jit::jit_x64::JitX64;
    use crate::cpu::{Cpu, RegisterType};
    use crate::ppu::Ppu;

    #[test]
    pub fn test_ld_r_r() {
        let jit = JitX64::default();
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
