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

        jit.emit_fns();

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
