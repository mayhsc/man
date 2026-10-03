use crate::registers::{
    Operand8, Reg8, Reg16::{self, DE}, Reg16Mem,
};

pub fn operand8_from_index(idx: u8) -> Operand8 {
    match idx {
        0 => Operand8::Reg(Reg8::B),
        1 => Operand8::Reg(Reg8::C),
        2 => Operand8::Reg(Reg8::D),
        3 => Operand8::Reg(Reg8::E),
        4 => Operand8::Reg(Reg8::H),
        5 => Operand8::Reg(Reg8::L),
        6 => Operand8::MemHL,
        7 => Operand8::Reg(Reg8::A),
        _ => unreachable!(),
    }
}

pub fn reg16_from_index(idx: u8) -> Reg16 {
    match idx {
        0 => Reg16::BC,
        1 => Reg16::DE,
        2 => Reg16::HL,
        3 => Reg16::SP,
        _ => unreachable!(),
    }
}


pub fn reg16mem_from_index(idx: u8) -> Reg16Mem {
    match idx {
        0 => Reg16Mem::BC,
        1 => Reg16Mem::DE,
        2 => Reg16Mem::HLI,
        3 => Reg16Mem::HLD,
        _ => unreachable!(),
    }
}
