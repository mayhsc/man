use serde::Deserialize;

#[derive(Debug, Default, Clone, Copy, PartialEq, Deserialize)]
pub(crate) struct Flags(u8);

pub(crate) enum Condition {
    NZ,
    Z,
    NC,
    C,
}

impl Flags {
    pub fn z(&self) -> bool {
        (self.0 >> 7) & 1 == 1
    }
    pub fn n(&self) -> bool {
        (self.0 >> 6) & 1 == 1
    }
    pub fn h(&self) -> bool {
        (self.0 >> 5) & 1 == 1
    }
    pub fn c(&self) -> bool {
        (self.0 >> 4) & 1 == 1
    }

    pub fn set_z(&mut self, v: bool) {
        self.set_bit(7, v);
    }
    pub fn set_n(&mut self, v: bool) {
        self.set_bit(6, v);
    }
    pub fn set_h(&mut self, v: bool) {
        self.set_bit(5, v);
    }
    pub fn set_c(&mut self, v: bool) {
        self.set_bit(4, v);
    }

    fn set_bit(&mut self, bit: u8, v: bool) {
        self.0 = (self.0 & !(1 << bit)) | ((v as u8) << bit);
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
    pub fn from_u8(v: u8) -> Self {
        Self(v & 0xF0)
    }
}
