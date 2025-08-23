use crate::cpu::jit::REGISTER_OFFSETS;
use crate::cpu::RegisterType;
use dynasm::dynasm;
use dynasmrt::{x64::Assembler, DynasmApi};

pub fn compile_ld_r_r(ops: &mut Assembler, dst: RegisterType, src: RegisterType) {
    let dst_offset = REGISTER_OFFSETS[dst as usize];
    let src_offset = REGISTER_OFFSETS[src as usize];

    dynasm!(ops
        ; .arch x64
        ; movzx eax, BYTE [rdi + src_offset as i32]
        ; mov    [rdi + dst_offset as i32], al
        ; ret
    );
}

pub fn compile_ld_r_d8(ops: &mut Assembler, dst: RegisterType, imm8: u8) {
    let dst_offset = REGISTER_OFFSETS[dst as usize];

    dynasm!(ops
        ; .arch x64
        ; mov BYTE [rdi + dst_offset as i32], BYTE imm8 as i8
    );
}
