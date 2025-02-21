use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{FetchedData, RegisterType};
use crate::cpu::{Cpu, CpuCallback};

#[derive(Debug, Clone, Copy)]
pub struct PrefixInstruction;

const REG_TYPES_BY_OPS: [RegisterType; 8] = [
    RegisterType::B,
    RegisterType::C,
    RegisterType::D,
    RegisterType::E,
    RegisterType::H,
    RegisterType::L,
    RegisterType::HL,
    RegisterType::A,
];

pub fn decode_reg(reg: u16) -> Option<RegisterType> {
    let reg = reg as u8;

    if reg > 0b111 {
        return None;
    }

    Some(REG_TYPES_BY_OPS[reg as usize])
}

impl ExecutableInstruction for PrefixInstruction {
    fn execute(&self, cpu: &mut Cpu, callback: &mut impl CpuCallback, fetched_data: FetchedData) {
        let op = fetched_data.value;
        let reg = decode_reg(op & 0b111);

        let Some(reg) = reg else {
            return;
        };

        let bit = (op >> 3) & 0b111;
        let bit_op = (op >> 6) & 0b11;
        let mut reg_val = cpu.read_reg8(reg, callback);

        match bit_op {
            1 => {
                // BIT
                cpu.registers.flags.set(
                    ((reg_val & (1 << bit)) == 0).into(),
                    false.into(),
                    true.into(),
                    None,
                );
                return;
            }
            2 => {
                // RST
                reg_val &= !(1 << bit);
                cpu.set_reg8(reg, reg_val, callback);
                return;
            }
            3 => {
                // SET
                reg_val |= 1 << bit;
                cpu.set_reg8(reg, reg_val, callback);
                return;
            }
            _ => {}
        }

        let flag_c = cpu.registers.flags.get_c();

        match bit {
            0 => {
                // RLC
                let carry = (reg_val & 0x80) != 0; // Check MSB for carry
                let result = (reg_val << 1) | (carry as u8); // Rotate left and wrap MSB to LSB

                cpu.set_reg8(reg, result, callback);
                cpu.registers.flags.set(
                    (result == 0).into(), // Zero flag (not set for RLCA)
                    false.into(),         // Subtract flag
                    false.into(),         // Half-Carry flag
                    carry.into(),         // Carry flag
                );
            }
            1 => {
                // RRC
                let old = reg_val;
                reg_val = reg_val >> 1 | (old << 7);

                cpu.set_reg8(reg, reg_val, callback);
                cpu.registers.flags.set(
                    (reg_val == 0).into(),
                    false.into(),
                    false.into(),
                    Some(old & 1 != 0),
                );
            }
            2 => {
                // RL
                let old = reg_val;
                reg_val = (reg_val << 1) | (flag_c as u8);

                cpu.set_reg8(reg, reg_val, callback);
                cpu.registers.flags.set(
                    (reg_val == 0).into(),
                    false.into(),
                    false.into(),
                    Some((old & 0x80) != 0),
                );
            }
            3 => {
                // RR
                let old = reg_val;
                reg_val = (reg_val >> 1) | ((flag_c as u8) << 7);

                cpu.set_reg8(reg, reg_val, callback);
                cpu.registers.flags.set(
                    (reg_val == 0).into(),
                    false.into(),
                    false.into(),
                    Some((old & 1) != 0),
                );
            }
            4 => {
                // SLA
                let old = reg_val;
                reg_val <<= 1;

                cpu.set_reg8(reg, reg_val, callback);
                cpu.registers.flags.set(
                    (reg_val == 0).into(),
                    false.into(),
                    false.into(),
                    Some((old & 0x80) != 0),
                );
            }
            5 => {
                // SRA
                let u: i8 = reg_val as i8;
                let u = (u >> 1) as u8;
                let result = (reg_val >> 1) | (reg_val & 0x80); // Shift right and preserve MSB
                let carry = reg_val & 0x01 != 0; // Save LSB as Carry

                cpu.set_reg8(reg, result, callback);
                cpu.registers
                    .flags
                    .set((u == 0).into(), false.into(), false.into(), carry.into());
            }
            6 => {
                // SWAP
                reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0x0F) << 4);
                cpu.set_reg8(reg, reg_val, callback);

                cpu.registers.flags.set(
                    (reg_val == 0).into(),
                    false.into(),
                    false.into(),
                    false.into(),
                );
            }
            7 => {
                // SRL
                let u = reg_val >> 1;

                cpu.set_reg8(reg, u, callback);
                cpu.registers.flags.set(
                    (u == 0).into(),
                    false.into(),
                    false.into(),
                    Some((reg_val & 1) != 0),
                );
            }
            _ => {
                eprintln!("ERROR: INVALID CB: {:02X}", op);
                unimplemented!();
            }
        }

        callback.m_cycles(1, &mut cpu.bus);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D8
    }
}
