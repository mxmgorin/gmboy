use crate::cpu::{Cpu, Flags, RegisterType, Registers};
use memoffset::offset_of;

pub mod x64;

#[inline(always)]
pub fn is_ld_r_r(opcode: u8) -> bool {
    (0x40..=0x7F).contains(&opcode) && opcode != 0x76
}

pub fn is_ld_r_d8(opcode: u8) -> bool {
    matches!(opcode, 0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x3E)
}

pub fn is_control_flow(opcode: u8) -> bool {
    matches!(
        opcode,
        0xC3 | 0xC2 | 0xCA | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 | // JP/JR (cond+uncond)
        0xCD | 0xC9 | 0xD9 | // CALL/RET/RETI
        0x76 // HALT
    )
}

pub fn get_ld_r_d8_dst(opcode: u8) -> RegisterType {
    match opcode {
        0x06 => RegisterType::B,
        0x0E => RegisterType::C,
        0x16 => RegisterType::D,
        0x1E => RegisterType::E,
        0x26 => RegisterType::H,
        0x2E => RegisterType::L,
        0x3E => RegisterType::A,
        _ => unreachable!(),
    }
}

pub fn get_ld_r_r_dst(opcode: u8) -> Option<RegisterType> {
    let dst = (opcode >> 3) & 7;

    get_ld_r_r_register(dst)
}

pub fn get_ld_r_r_src(opcode: u8) -> Option<RegisterType> {
    let dst = (opcode >> 3) & 7;

    get_ld_r_r_register(dst)
}

fn get_ld_r_r_register(code: u8) -> Option<RegisterType> {
    let o = match code & 0x7 {
        0 => RegisterType::B,
        1 => RegisterType::C,
        2 => RegisterType::D,
        3 => RegisterType::E,
        4 => RegisterType::H,
        5 => RegisterType::L,
        6 => return None, // (HL) -> stop block before memory for now
        7 => RegisterType::A,
        _ => unreachable!(),
    };

    Some(o)
}

pub const REGISTERS_OFFSET: usize = offset_of!(Cpu, registers);

pub const REGISTER_OFFSETS: [usize; 14] = {
    const REGISTERS: &[RegisterType] = RegisterType::get_all();
    let mut array = [0; REGISTERS.len()];
    let mut i = 0;

    loop {
        if i >= REGISTERS.len() {
            break;
        }

        let reg = REGISTERS[i];

        if !reg.is_16bit() {
            array[reg as usize] = REGISTERS_OFFSET + reg.get_offset();
        }

        i += 1;
    }

    array
};

impl RegisterType {
    pub const fn get_offset(&self) -> usize {
        match self {
            RegisterType::A => offset_of!(Registers, a),
            RegisterType::B => offset_of!(Registers, b),
            RegisterType::C => offset_of!(Registers, c),
            RegisterType::D => offset_of!(Registers, d),
            RegisterType::E => offset_of!(Registers, e),
            RegisterType::H => offset_of!(Registers, h),
            RegisterType::L => offset_of!(Registers, l),
            RegisterType::F => offset_of!(Registers, flags) + offset_of!(Flags, byte),
            RegisterType::SP => offset_of!(Registers, sp),
            RegisterType::PC => offset_of!(Registers, pc),
            RegisterType::AF | RegisterType::BC | RegisterType::DE | RegisterType::HL => {
                unimplemented!()
            }
        }
    }
}
