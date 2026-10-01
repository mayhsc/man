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

pub(crate) enum Dst {
    Reg(Reg8),
    Memory,
}

pub(crate) enum Src {
    Reg(Reg8),
    Memory,
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
    pub(crate) fn get(&self, r: Reg8) -> u8 {
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

    pub(crate) fn set(&mut self, r: Reg8, v: u8) {
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

    pub(crate) fn increment(&mut self) {
        self.pc = self.pc.wrapping_add(1)
    }

    pub(crate) fn hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }
}
