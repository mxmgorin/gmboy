use crate::cpu::instructions::{AddressMode, ExecutableInstruction};
use crate::cpu::instructions::{FetchedData, RegisterType};
use crate::cpu::{Cpu};

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
    fn execute(&self, cpu: &mut Cpu, fetched_data: FetchedData) {
        let op = fetched_data.value;
        let reg = decode_reg(op & 0b111);

        let Some(reg) = reg else {
            return;
        };

        let bit = (op >> 3) & 0b111;
        let bit_op = (op >> 6) & 0b11;
        let mut reg_val = cpu.read_reg8(reg);

        match bit_op {
            1 => {
                // BIT
                cpu.registers.flags.set_z((reg_val & (1 << bit)) == 0);
                cpu.registers.flags.set_n(false);
                cpu.registers.flags.set_h(true);
                return;
            }
            2 => {
                // RST
                reg_val &= !(1 << bit);
                cpu.set_reg8(reg, reg_val);
                return;
            }
            3 => {
                // SET
                reg_val |= 1 << bit;
                cpu.set_reg8(reg, reg_val);
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

                cpu.set_reg8(reg, result);
                cpu.registers.flags.set_z(result == 0);
                cpu.registers.flags.set_n(false);
                cpu.registers.flags.set_h(false);
                cpu.registers.flags.set_c(carry);
            }
            1 => {
                // RRC
                let old = reg_val;
                reg_val = reg_val >> 1 | (old << 7);

                cpu.set_reg8(reg, reg_val);
                cpu.registers.flags.set_z(reg_val == 0);
                cpu.registers.flags.set_n(false);
                cpu.registers.flags.set_h(false);
                cpu.registers.flags.set_c(old & 1 != 0);
            }
            2 => {
                // RL
                let old = reg_val;
                reg_val = (reg_val << 1) | (flag_c as u8);

                cpu.set_reg8(reg, reg_val);

                cpu.registers.flags.set_z(reg_val == 0);
                cpu.registers.flags.set_n(false);
                cpu.registers.flags.set_h(false);
                cpu.registers.flags.set_c((old & 0x80) != 0);
            }
            3 => {
                // RR
                let old = reg_val;
                reg_val = (reg_val >> 1) | ((flag_c as u8) << 7);

                cpu.set_reg8(reg, reg_val);

                cpu.registers.flags.set_z(reg_val == 0);
                cpu.registers.flags.set_n(false);
                cpu.registers.flags.set_h(false);
                cpu.registers.flags.set_c((old & 1) != 0);
            }
            4 => {
                // SLA
                let old = reg_val;
                reg_val <<= 1;

                cpu.set_reg8(reg, reg_val);

                cpu.registers.flags.set_z(reg_val == 0);
                cpu.registers.flags.set_n(false);
                cpu.registers.flags.set_h(false);
                cpu.registers.flags.set_c((old & 0x80) != 0);
            }
            5 => {
                // SRA
                let u: i8 = reg_val as i8;
                let u = (u >> 1) as u8;
                let result = (reg_val >> 1) | (reg_val & 0x80); // Shift right and preserve MSB
                let carry = reg_val & 0x01 != 0; // Save LSB as Carry

                cpu.set_reg8(reg, result);

                cpu.registers.flags.set_z(u == 0);
                cpu.registers.flags.set_n(false);
                cpu.registers.flags.set_h(false);
                cpu.registers.flags.set_c(carry);
            }
            6 => {
                // SWAP
                reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0x0F) << 4);
                cpu.set_reg8(reg, reg_val);

                cpu.registers.flags.set_z(reg_val == 0);
                cpu.registers.flags.set_n(false);
                cpu.registers.flags.set_h(false);
                cpu.registers.flags.set_c(false);
            }
            7 => {
                // SRL
                let u = reg_val >> 1;

                cpu.set_reg8(reg, u);

                cpu.registers.flags.set_z(u == 0);
                cpu.registers.flags.set_n(false);
                cpu.registers.flags.set_h(false);
                cpu.registers.flags.set_c((reg_val & 1) != 0);
            }
            _ => {
                panic!("ERROR: INVALID CB: {op:02X}");
            }
        }

        cpu.clock.m_cycles(1);
    }

    fn get_address_mode(&self) -> AddressMode {
        AddressMode::D8
    }
}
