use crate::cpu::{Cpu, RegisterType};

impl Cpu {
    #[inline(always)]
    fn execute_cb_bit(&mut self) {
        let op = self.step_ctx.fetched_data.value as u8;
        let bit = get_cb_bit(op);
        let register = get_register(op);

        let val = (register.get)(self);
        self.registers
            .flags
            .set_znh((val & (1 << bit)) == 0, false, true);
    }

    #[inline(always)]
    fn execute_cb_res(&mut self) {
        let op = self.step_ctx.fetched_data.value as u8;
        let bit = get_cb_bit(op);
        let register = get_register(op);

        let mut val = (register.get)(self);
        val &= !(1 << bit);
        (register.set)(self, val);
    }

    #[inline(always)]
    fn execute_cb_set(&mut self) {
        let op = self.step_ctx.fetched_data.value as u8;
        let bit = get_cb_bit(op);
        let register = get_register(op);

        let mut reg_val = (register.get)(self);
        reg_val |= 1 << bit;
        (register.set)(self, reg_val);
    }

    #[inline(always)]
    fn execute_cb_rlc(&mut self) {
        let op = self.step_ctx.fetched_data.value;
        let register = get_register(op as u8);

        let reg_val = (register.get)(self);
        let carry = (reg_val & 0x80) != 0; // Check MSB for carry
        let result = (reg_val << 1) | (carry as u8); // Rotate left and wrap MSB to LSB

        (register.set)(self, result);
        self.registers
            .flags
            .set_znhc(result == 0, false, false, carry);
    }

    #[inline(always)]
    fn execute_cb_rrc(&mut self) {
        let op = self.step_ctx.fetched_data.value;
        let register = get_register(op as u8);

        let mut reg_val = (register.get)(self);
        let old = reg_val;
        reg_val = reg_val >> 1 | (old << 7);

        (register.set)(self, reg_val);
        self.registers
            .flags
            .set_znhc(reg_val == 0, false, false, old & 1 != 0);
    }

    #[inline(always)]
    fn execute_cb_rl(&mut self) {
        let op = self.step_ctx.fetched_data.value;
        let register = get_register(op as u8);

        let mut reg_val = (register.get)(self);
        let old = reg_val;
        let flag_c = self.registers.flags.get_c();
        reg_val = (reg_val << 1) | (flag_c as u8);

        (register.set)(self, reg_val);
        self.registers
            .flags
            .set_znhc(reg_val == 0, false, false, (old & 0x80) != 0);
    }

    #[inline(always)]
    fn execute_cb_rr(&mut self) {
        let op = self.step_ctx.fetched_data.value;
        let register = get_register(op as u8);

        let mut reg_val = (register.get)(self);
        let old = reg_val;
        let flag_c = self.registers.flags.get_c();
        reg_val = (reg_val >> 1) | ((flag_c as u8) << 7);

        (register.set)(self, reg_val);
        self.registers
            .flags
            .set_znhc(reg_val == 0, false, false, (old & 1) != 0);
    }

    #[inline(always)]
    fn execute_cb_sla(&mut self) {
        let op = self.step_ctx.fetched_data.value;
        let register = get_register(op as u8);

        let mut reg_val = (register.get)(self);
        let old = reg_val;
        reg_val <<= 1;

        (register.set)(self, reg_val);
        self.registers
            .flags
            .set_znhc(reg_val == 0, false, false, (old & 0x80) != 0);
    }

    #[inline(always)]
    fn execute_cb_sra(&mut self) {
        let op = self.step_ctx.fetched_data.value;
        let register = get_register(op as u8);

        let reg_val = (register.get)(self);
        let u: i8 = reg_val as i8;
        let u = (u >> 1) as u8;
        let result = (reg_val >> 1) | (reg_val & 0x80); // Shift right and preserve MSB
        let carry = reg_val & 0x01 != 0; // Save LSB as Carry

        (register.set)(self, result);
        self.registers.flags.set_znhc(u == 0, false, false, carry);
    }

    #[inline(always)]
    fn execute_cb_swap(&mut self) {
        let op = self.step_ctx.fetched_data.value;
        let register = get_register(op as u8);

        let mut reg_val = (register.get)(self);
        reg_val = ((reg_val & 0xF0) >> 4) | ((reg_val & 0x0F) << 4);

        (register.set)(self, reg_val);
        self.registers
            .flags
            .set_znhc(reg_val == 0, false, false, false);
    }

    #[inline(always)]
    fn execute_cb_srl(&mut self) {
        let op = self.step_ctx.fetched_data.value;
        let register = get_register(op as u8);

        let reg_val = (register.get)(self);
        let u = reg_val >> 1;

        (register.set)(self, u);
        self.registers
            .flags
            .set_znhc(u == 0, false, false, (reg_val & 1) != 0);
    }

    #[inline(always)]
    pub fn fetch_execute_cb(&mut self) {
        self.fetch_d8();
        let op = self.step_ctx.fetched_data.value;
        CB_FNS[op as usize](self);
    }

    #[inline(always)]
    fn get_register8_cb<const R: u8>(&mut self) -> u8 {
        let rt = RegisterType::from_u8(R);

        match rt {
            RegisterType::A => self.registers.a,
            RegisterType::F => self.registers.flags.get_byte(),
            RegisterType::B => self.registers.b,
            RegisterType::C => self.registers.c,
            RegisterType::D => self.registers.d,
            RegisterType::E => self.registers.e,
            RegisterType::H => self.registers.h,
            RegisterType::L => self.registers.l,
            RegisterType::HL => {
                let addr = self.registers.get_register::<{ RegisterType::HL as u8 }>();
                self.read_memory(addr)
            }
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn set_register8_cb<const R: u8>(&mut self, val: u8) {
        let rt = RegisterType::from_u8(R);
        match rt {
            RegisterType::A => self.registers.a = val,
            RegisterType::F => self.registers.flags.set_byte(val),
            RegisterType::B => self.registers.b = val,
            RegisterType::C => self.registers.c = val,
            RegisterType::D => self.registers.d = val,
            RegisterType::E => self.registers.e = val,
            RegisterType::H => self.registers.h = val,
            RegisterType::L => self.registers.l = val,
            RegisterType::HL => {
                let addr = self.registers.get_register::<{ RegisterType::HL as u8 }>();
                self.write_to_memory(addr, val)
            }
            _ => unreachable!(),
        }
    }

    fn invalid_cb(&mut self) {
        panic!("INVALID CB");
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
            set: Cpu::set_register8_cb::<R>,
            get: Cpu::get_register8_cb::<R>,
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

type CbFn = fn(&mut Cpu);

#[inline(always)]
const fn get_cb_bit(op: u8) -> u8 {
    (op >> 3) & 0b111
}

#[inline(always)]
const fn get_cb_bit_op(op: u8) -> u8 {
    (op >> 6) & 0b11
}

#[inline(always)]
const fn get_register(op: u8) -> RegisterFn {
    let register_index = (op & 0b111) as usize;

    if register_index > REGISTER_FNS.len() {
        panic!("INVALID CB register index");
    }

    REGISTER_FNS[register_index]
}

#[inline(always)]
pub const fn new_cb_fn(op: u8) -> CbFn {
    let bit_op = get_cb_bit_op(op);
    let bit = get_cb_bit(op);

    match bit_op {
        1 => Cpu::execute_cb_bit,
        2 => Cpu::execute_cb_res,
        3 => Cpu::execute_cb_set,
        _ => match bit {
            0 => Cpu::execute_cb_rlc,
            1 => Cpu::execute_cb_rrc,
            2 => Cpu::execute_cb_rl,
            3 => Cpu::execute_cb_rr,
            4 => Cpu::execute_cb_sla,
            5 => Cpu::execute_cb_sra,
            6 => Cpu::execute_cb_swap,
            7 => Cpu::execute_cb_srl,
            _ => panic!(),
        },
    }
}

const CB_FNS: [CbFn; 0x100] = {
    let mut table: [CbFn; 0x100] = [Cpu::invalid_cb; 0x100];
    let mut i = 0;

    while i < 0x100 {
        table[i] = new_cb_fn(i as u8);
        i += 1;
    }

    table
};
