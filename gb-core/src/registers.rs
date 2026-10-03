use serde::Deserialize;

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
    pub f: u8,
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

    pub(crate) fn get16(&self, r: Reg16) -> u16 {
        match r {
            Reg16::BC => self.bc(),
            Reg16::DE => self.de(),
            Reg16::HL => self.hl(),
            Reg16::SP => todo!(),
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
            Reg16::SP => todo!(),
        };
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

    pub(crate) fn add_a(&mut self, value: u8, carry: bool) {
        let carry_in = if carry && self.flag_c() { 1 } else { 0 };
        let a = self.a;

        let (r1, c1) = a.overflowing_add(value);
        let (result, c2) = r1.overflowing_add(carry_in);
        let carry = c1 || c2;

        let half_carry = (a & 0xF) + (value & 0xF) + carry_in > 0xF;

        self.a = result;
        self.set_zf(result == 0);
        self.set_nf(false);
        self.set_hf(half_carry);
        self.set_cf(carry);
    }

    fn flag_c(&self) -> bool {
        (self.f >> 4 & 1) == 1
    }

    fn set_zf(&mut self, f: bool) {
        self.f = self.f & !(1 << 7) | ((f as u8) << 7);
    }

    fn set_nf(&mut self, f: bool) {
        self.f = self.f & !(1 << 6) | ((f as u8) << 6);
    }

    fn set_hf(&mut self, f: bool) {
        self.f = self.f & !(1 << 5) | ((f as u8) << 5);
    }
    fn set_cf(&mut self, f: bool) {
        self.f = self.f & !(1 << 4) | ((f as u8) << 4);
    }
}
