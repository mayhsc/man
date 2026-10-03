use crate::registers::{Operand8, Reg8};

pub fn operand8_from_index(idx: u8) -> Operand8 {
    match idx {
        0 => Operand8::Reg(Reg8::B),
        1 => Operand8::Reg(Reg8::C),
        2 => Operand8::Reg(Reg8::D),
        3 => Operand8::Reg(Reg8::E),
        4 => Operand8::Reg(Reg8::H),
        5 => Operand8::Reg(Reg8::L),
        6 => Operand8::Memory,
        7 => Operand8::Reg(Reg8::A),
        _ => unreachable!(),
    }
}
