use serde::Deserialize;

use crate::cpu::flags::{Condition, Flags};

pub(crate) enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub(crate) enum Reg16 {
    BC,
    DE,
    HL,
    SP,
}

pub(crate) enum Reg16Mem {
    BC,
    DE,
    HLI,
    HLD,
}

pub(crate) enum Reg16Stk {
    BC,
    DE,
    HL,
    AF,
}

pub(crate) enum Operand8 {
    Reg(Reg8),
    MemHL,
    D8(u8),
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub(crate) f: Flags,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub(crate) fn get8(&self, r: Reg8) -> u8 {
        match r {
            Reg8::A => self.a,
            Reg8::B => self.b,
            Reg8::C => self.c,
            Reg8::D => self.d,
            Reg8::E => self.e,
            Reg8::H => self.h,
            Reg8::L => self.l,
        }
    }

    pub(crate) fn set8(&mut self, r: Reg8, v: u8) {
        match r {
            Reg8::A => self.a = v,
            Reg8::B => self.b = v,
            Reg8::C => self.c = v,
            Reg8::D => self.d = v,
            Reg8::E => self.e = v,
            Reg8::H => self.h = v,
            Reg8::L => self.l = v,
        };
    }

    pub(crate) fn get16(&self, r: &Reg16) -> u16 {
        match r {
            Reg16::BC => self.bc(),
            Reg16::DE => self.de(),
            Reg16::HL => self.hl(),
            Reg16::SP => self.sp,
        }
    }

    pub(crate) fn set16(&mut self, r: Reg16, v: u16) {
        match r {
            Reg16::BC => {
                self.b = (v >> 8) as u8;
                self.c = (v) as u8;
            }
            Reg16::DE => {
                self.d = (v >> 8) as u8;
                self.e = (v) as u8;
            }
            Reg16::HL => {
                self.h = (v >> 8) as u8;
                self.l = (v) as u8;
            }
            Reg16::SP => self.sp = v,
        };
    }

    /// Also increments/decerements HL register
    pub(crate) fn get16mem(&mut self, r: Reg16Mem) -> u16 {
        match r {
            Reg16Mem::BC => self.bc(),
            Reg16Mem::DE => self.de(),
            Reg16Mem::HLI => {
                let hl = self.hl();
                self.set16(Reg16::HL, hl.wrapping_add(1));
                hl
            }
            Reg16Mem::HLD => {
                let hl = self.hl();
                self.set16(Reg16::HL, hl.wrapping_sub(1));
                hl
            }
        }
    }

    pub(crate) fn get16stk(&self, r: &Reg16Stk) -> u16 {
        match r {
            Reg16Stk::BC => self.bc(),
            Reg16Stk::DE => self.de(),
            Reg16Stk::HL => self.hl(),
            Reg16Stk::AF => self.af(),
        }
    }

    pub(crate) fn set16stk(&mut self, r: Reg16Stk, v: u16) {
        match r {
            Reg16Stk::BC => {
                self.b = (v >> 8) as u8;
                self.c = (v) as u8;
            }
            Reg16Stk::DE => {
                self.d = (v >> 8) as u8;
                self.e = (v) as u8;
            }
            Reg16Stk::HL => {
                self.h = (v >> 8) as u8;
                self.l = (v) as u8;
            }
            Reg16Stk::AF => {
                self.a = (v >> 8) as u8;
                self.f.from_u8((v as u8) & 0xF0);
            }
        };
    }

    pub(crate) fn get_condition(&self, c: Condition) -> bool {
        match c {
            Condition::NZ => !self.f.z(),
            Condition::Z => self.f.z(),
            Condition::NC => !self.f.c(),
            Condition::C => self.f.c(),
        }
    }

    pub(crate) fn increment(&mut self) {
        self.pc = self.pc.wrapping_add(1)
    }

    pub(crate) fn bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    pub(crate) fn de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }

    pub(crate) fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    pub(crate) fn af(&self) -> u16 {
        (self.a as u16) << 8 | (self.f.as_u8() as u16)
    }

    pub(crate) fn add_a(&mut self, v: u8, carry: bool) {
        let carry_in = if carry && self.f.c() { 1 } else { 0 };
        let a = self.a;

        let (r1, c1) = a.overflowing_add(v);
        let (result, c2) = r1.overflowing_add(carry_in);
        let carry = c1 || c2;

        let half_carry = (a & 0xF) + (v & 0xF) + carry_in > 0xF;

        self.a = result;
        self.f.set_z(result == 0);
        self.f.set_n(false);
        self.f.set_h(half_carry);
        self.f.set_c(carry);
    }

    pub(crate) fn sub_a(&mut self, v: u8, use_carry: bool) {
        let carry_in = if use_carry && self.f.c() { 1 } else { 0 };
        let a = self.a;

        let (r1, b1) = a.overflowing_sub(v);
        let (result, b2) = r1.overflowing_sub(carry_in);
        let borrow = b1 || b2;

        let half_borrow = (a & 0xF) < (v & 0xF) + carry_in;

        self.a = result;
        self.f.set_z(result == 0);
        self.f.set_n(true);
        self.f.set_h(half_borrow);
        self.f.set_c(borrow);
    }

    pub(crate) fn add_hl(&mut self, v: u16) {
        let hl = self.hl();

        let (result, carry) = hl.overflowing_add(v);

        let half_carry = (hl & 0x0FFF) + (v & 0x0FFF) > 0x0FFF;

        self.set16(Reg16::HL, result);

        self.f.set_n(false);
        self.f.set_h(half_carry);
        self.f.set_c(carry);
    }

    pub(crate) fn and(&mut self, v: u8) {
        let result = self.a & v;
        self.a = result;

        self.f.set_z(result == 0);
        self.f.set_n(false);
        self.f.set_h(true);
        self.f.set_c(false);
    }

    pub(crate) fn or(&mut self, v: u8) {
        let result = self.a | v;
        self.a = result;

        self.f.set_z(result == 0);
        self.f.set_n(false);
        self.f.set_h(false);
        self.f.set_c(false);
    }

    pub(crate) fn xor(&mut self, v: u8) {
        let result = self.a ^ v;
        self.a = result;

        self.f.set_z(result == 0);
        self.f.set_n(false);
        self.f.set_h(false);
        self.f.set_c(false);
    }

    pub(crate) fn cp(&mut self, v: u8) {
        let a = self.a;

        let (r1, b1) = a.overflowing_sub(v);

        let half_borrow = (a & 0xF) < (v & 0xF);
        self.f.set_z(r1 == 0);
        self.f.set_n(true);
        self.f.set_h(half_borrow);
        self.f.set_c(b1);
    }

    pub(crate) fn rlca(&mut self) {
        let carry = self.a & 0x80 != 0;

        self.a = self.a.rotate_left(1);

        self.f.set_z(false);
        self.f.set_n(false);
        self.f.set_h(false);
        self.f.set_c(carry);
    }

    pub(crate) fn rrca(&mut self) {
        let carry = self.a & 0x01 != 0;

        self.a = self.a.rotate_right(1);

        self.f.set_z(false);
        self.f.set_n(false);
        self.f.set_h(false);
        self.f.set_c(carry);
    }

    pub(crate) fn rla(&mut self) {
        let old_carry = self.f.c();
        let new_carry = self.a & 0x80 != 0;

        self.a = (self.a << 1) | old_carry as u8;

        self.f.set_z(false);
        self.f.set_n(false);
        self.f.set_h(false);
        self.f.set_c(new_carry);
    }

    pub(crate) fn rra(&mut self) {
        let old_carry = self.f.c();
        let new_carry = self.a & 0x01 != 0;

        self.a = (self.a >> 1) | ((old_carry as u8) << 7);

        self.f.set_z(false);
        self.f.set_n(false);
        self.f.set_h(false);
        self.f.set_c(new_carry);
    }

    pub(crate) fn cpl(&mut self) {
        self.a = !self.a;

        self.f.set_n(true);
        self.f.set_h(true);
    }

    pub(crate) fn scf(&mut self) {
        self.f.set_n(false);
        self.f.set_h(false);
        self.f.set_c(true);
    }

    pub(crate) fn ccf(&mut self) {
        let carry = self.f.c();

        self.f.set_n(false);
        self.f.set_h(false);
        self.f.set_c(!carry);
    }

    pub(crate) fn daa(&mut self) {
        let mut correction = 0;
        let mut carry = self.f.c();

        if self.f.n() {
            if self.f.h() {
                correction |= 0x06;
            }

            if carry {
                correction |= 0x60;
            }

            self.a = self.a.wrapping_sub(correction);
        } else {
            if self.f.h() || (self.a & 0x0F) > 9 {
                correction |= 0x06;
            }

            if carry || self.a > 0x99 {
                correction |= 0x60;
                carry = true;
            }

            self.a = self.a.wrapping_add(correction);
        }

        self.f.set_z(self.a == 0);
        self.f.set_h(false);
        self.f.set_c(carry);
    }

    pub(crate) fn jr(&mut self, offset: i8, c: Condition) {
        if self.get_condition(c) {
            self.pc = self.pc.wrapping_add_signed(offset as i16);
        }
    }
}
