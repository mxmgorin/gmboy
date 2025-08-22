use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    fn _execute_cb_bit_const<const B: u8, const R: u8>(&mut self) {
        let val = self.get_register8::<R>();
        self.registers.flags.set_z((val & (1 << B)) == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(true);
    }

    #[inline(always)]
    fn execute_cb_bit(&mut self, register: &RegisterFn, bit: u16) {
        let val = (register.get)(self);
        self.registers.flags.set_z((val & (1 << bit)) == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(true);
    }

    #[inline(always)]
    fn execute_cb_res(&mut self, register: &RegisterFn, bit: u16) {
        let mut val = (register.get)(self);
        val &= !(1 << bit);
        (register.set)(self, val);
    }

    #[inline(always)]
    fn execute_cb_set(&mut self, register: &RegisterFn, bit: u16) {
        let mut reg_val = (register.get)(self);
        reg_val |= 1 << bit;
        (register.set)(self, reg_val);
    }

    #[inline(always)]
    fn execute_cb_rlc(&mut self, register: &RegisterFn) {
        let reg_val = (register.get)(self);
        let carry = (reg_val & 0x80) != 0; // Check MSB for carry
        let result = (reg_val << 1) | (carry as u8); // Rotate left and wrap MSB to LSB

        (register.set)(self, result);
        self.registers.flags.set_z(result == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(carry);
    }

    #[inline(always)]
    fn execute_cb_rrc(&mut self, register: &RegisterFn) {
        let mut reg_val = (register.get)(self);
        let old = reg_val;
        reg_val = reg_val >> 1 | (old << 7);

        (register.set)(self, reg_val);
        self.registers.flags.set_z(reg_val == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(old & 1 != 0);
    }

    #[inline(always)]
    fn execute_cb_rl(&mut self, register: &RegisterFn) {
        let mut reg_val = (register.get)(self);
        let old = reg_val;
        let flag_c = self.registers.flags.get_c();
        reg_val = (reg_val << 1) | (flag_c as u8);

        (register.set)(self, reg_val);

        self.registers.flags.set_z(reg_val == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c((old & 0x80) != 0);
    }

    #[inline(always)]
    fn execute_cb_rr(&mut self, register: &RegisterFn) {
        let mut reg_val = (register.get)(self);
        let old = reg_val;
        let flag_c = self.registers.flags.get_c();
        reg_val = (reg_val >> 1) | ((flag_c as u8) << 7);

        (register.set)(self, reg_val);

        self.registers.flags.set_z(reg_val == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c((old & 1) != 0);
    }

    #[inline(always)]
    fn execute_cb_sla(&mut self, register: &RegisterFn) {
        let mut reg_val = (register.get)(self);
        let old = reg_val;
        reg_val <<= 1;

        (register.set)(self, reg_val);

        self.registers.flags.set_z(reg_val == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c((old & 0x80) != 0);
    }

    #[inline(always)]
    fn execute_cb_sra(&mut self, register: &RegisterFn) {
        let reg_val = (register.get)(self);
        let u: i8 = reg_val as i8;
        let u = (u >> 1) as u8;
        let result = (reg_val >> 1) | (reg_val & 0x80); // Shift right and preserve MSB
        let carry = reg_val & 0x01 != 0; // Save LSB as Carry

        (register.set)(self, result);

        self.registers.flags.set_z(u == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(carry);
    }

    #[inline(always)]
    fn execute_cb_swap(&mut self, register: &RegisterFn) {
        let mut reg_val = (register.get)(self);
        reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0x0F) << 4);
        (register.set)(self, reg_val);

        self.registers.flags.set_z(reg_val == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c(false);
    }

    #[inline(always)]
    fn execute_cb_srl(&mut self, register: &RegisterFn) {
        let reg_val = (register.get)(self);
        let u = reg_val >> 1;

        (register.set)(self, u);

        self.registers.flags.set_z(u == 0);
        self.registers.flags.set_n(false);
        self.registers.flags.set_h(false);
        self.registers.flags.set_c((reg_val & 1) != 0);
    }

    #[inline(always)]
    pub fn fetch_execute_prefix(&mut self) {
        self.fetch_d8();
        let op = self.step_ctx.fetched_data.value;
        let register_index = (op & 0b111) as usize;

        if register_index > REGISTER_FNS.len() {
            return;
        }

        let register = REGISTER_FNS[register_index];
        let bit = (op >> 3) & 0b111;
        let bit_op = (op >> 6) & 0b11;

        match bit_op {
            1 => {
                self.execute_cb_bit(&register, bit);
                return;
            }
            2 => {
                self.execute_cb_res(&register, bit);
                return;
            }
            3 => {
                self.execute_cb_set(&register, bit);
                return;
            }
            _ => {}
        }

        match bit {
            0 => self.execute_cb_rlc(&register),
            1 => self.execute_cb_rrc(&register),
            2 => self.execute_cb_rl(&register),
            3 => self.execute_cb_rr(&register),
            4 => self.execute_cb_sla(&register),
            5 => self.execute_cb_sra(&register),
            6 => self.execute_cb_swap(&register),
            7 => self.execute_cb_srl(&register),
            _ => {
                panic!("ERROR: INVALID CB: {op:02X}");
            }
        }

        self.clock.tick_m_cycles(1);
    }

    #[inline(always)]
    fn get_register8<const R: u8>(&mut self) -> u8 {
        let rt = RegisterType::from_u8(R);

        match rt {
            RegisterType::A => self.registers.a,
            RegisterType::F => self.registers.flags.byte,
            RegisterType::B => self.registers.b,
            RegisterType::C => self.registers.c,
            RegisterType::D => self.registers.d,
            RegisterType::E => self.registers.e,
            RegisterType::H => self.registers.h,
            RegisterType::L => self.registers.l,
            RegisterType::HL => {
                self.read_memory(self.registers.read_register::<{ RegisterType::HL as u8 }>())
            }
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn set_register8<const R: u8>(&mut self, val: u8) {
        let rt = RegisterType::from_u8(R);
        match rt {
            RegisterType::A => self.registers.a = val,
            RegisterType::F => self.registers.flags.byte = val,
            RegisterType::B => self.registers.b = val,
            RegisterType::C => self.registers.c = val,
            RegisterType::D => self.registers.d = val,
            RegisterType::E => self.registers.e = val,
            RegisterType::H => self.registers.h = val,
            RegisterType::L => self.registers.l = val,
            RegisterType::HL => self.write_to_memory(
                self.registers.read_register::<{ RegisterType::HL as u8 }>(),
                val,
            ),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
struct RegisterFn {
    get: fn(&mut Cpu) -> u8,
    set: fn(&mut Cpu, u8),
}

impl RegisterFn {
    pub const fn new<const R: u8>() -> Self {
        Self {
            set: Cpu::set_register8::<R>,
            get: Cpu::get_register8::<R>,
        }
    }
}

const REGISTER_FNS: [RegisterFn; 8] = [
    RegisterFn::new::<{ RegisterType::B as u8 }>(),
    RegisterFn::new::<{ RegisterType::C as u8 }>(),
    RegisterFn::new::<{ RegisterType::D as u8 }>(),
    RegisterFn::new::<{ RegisterType::E as u8 }>(),
    RegisterFn::new::<{ RegisterType::H as u8 }>(),
    RegisterFn::new::<{ RegisterType::L as u8 }>(),
    RegisterFn::new::<{ RegisterType::HL as u8 }>(),
    RegisterFn::new::<{ RegisterType::A as u8 }>(),
];
