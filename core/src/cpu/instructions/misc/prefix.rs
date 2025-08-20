
use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline]
    pub fn execute_prefix(&mut self) {
        let op = self.step_ctx.fetched_data.value;
        let reg = decode_reg(op & 0b111);

        let Some(reg) = reg else {
            return;
        };

        let bit = (op >> 3) & 0b111;
        let bit_op = (op >> 6) & 0b11;
        let mut reg_val = self.read_reg8(reg);

        match bit_op {
            1 => {
                // BIT
                self.registers.flags.set_z((reg_val & (1 << bit)) == 0);
                self.registers.flags.set_n(false);
                self.registers.flags.set_h(true);
                return;
            }
            2 => {
                // RST
                reg_val &= !(1 << bit);
                self.set_reg8(reg, reg_val);
                return;
            }
            3 => {
                // SET
                reg_val |= 1 << bit;
                self.set_reg8(reg, reg_val);
                return;
            }
            _ => {}
        }

        let flag_c = self.registers.flags.get_c();

        match bit {
            0 => {
                // RLC
                let carry = (reg_val & 0x80) != 0; // Check MSB for carry
                let result = (reg_val << 1) | (carry as u8); // Rotate left and wrap MSB to LSB

                self.set_reg8(reg, result);
                self.registers.flags.set_z(result == 0);
                self.registers.flags.set_n(false);
                self.registers.flags.set_h(false);
                self.registers.flags.set_c(carry);
            }
            1 => {
                // RRC
                let old = reg_val;
                reg_val = reg_val >> 1 | (old << 7);

                self.set_reg8(reg, reg_val);
                self.registers.flags.set_z(reg_val == 0);
                self.registers.flags.set_n(false);
                self.registers.flags.set_h(false);
                self.registers.flags.set_c(old & 1 != 0);
            }
            2 => {
                // RL
                let old = reg_val;
                reg_val = (reg_val << 1) | (flag_c as u8);

                self.set_reg8(reg, reg_val);

                self.registers.flags.set_z(reg_val == 0);
                self.registers.flags.set_n(false);
                self.registers.flags.set_h(false);
                self.registers.flags.set_c((old & 0x80) != 0);
            }
            3 => {
                // RR
                let old = reg_val;
                reg_val = (reg_val >> 1) | ((flag_c as u8) << 7);

                self.set_reg8(reg, reg_val);

                self.registers.flags.set_z(reg_val == 0);
                self.registers.flags.set_n(false);
                self.registers.flags.set_h(false);
                self.registers.flags.set_c((old & 1) != 0);
            }
            4 => {
                // SLA
                let old = reg_val;
                reg_val <<= 1;

                self.set_reg8(reg, reg_val);

                self.registers.flags.set_z(reg_val == 0);
                self.registers.flags.set_n(false);
                self.registers.flags.set_h(false);
                self.registers.flags.set_c((old & 0x80) != 0);
            }
            5 => {
                // SRA
                let u: i8 = reg_val as i8;
                let u = (u >> 1) as u8;
                let result = (reg_val >> 1) | (reg_val & 0x80); // Shift right and preserve MSB
                let carry = reg_val & 0x01 != 0; // Save LSB as Carry

                self.set_reg8(reg, result);

                self.registers.flags.set_z(u == 0);
                self.registers.flags.set_n(false);
                self.registers.flags.set_h(false);
                self.registers.flags.set_c(carry);
            }
            6 => {
                // SWAP
                reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0x0F) << 4);
                self.set_reg8(reg, reg_val);

                self.registers.flags.set_z(reg_val == 0);
                self.registers.flags.set_n(false);
                self.registers.flags.set_h(false);
                self.registers.flags.set_c(false);
            }
            7 => {
                // SRL
                let u = reg_val >> 1;

                self.set_reg8(reg, u);

                self.registers.flags.set_z(u == 0);
                self.registers.flags.set_n(false);
                self.registers.flags.set_h(false);
                self.registers.flags.set_c((reg_val & 1) != 0);
            }
            _ => {
                panic!("ERROR: INVALID CB: {op:02X}");
            }
        }

        self.clock.m_cycles(1);
    }
}

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
