use crate::registers::Reg8;

pub fn reg8_from_index(idx: u8) -> Reg8 {
    match idx {
        0 => Reg8::B,
        1 => Reg8::C,
        2 => Reg8::D,
        3 => Reg8::E,
        4 => Reg8::H,
        5 => Reg8::L,
        6 => Reg8::HL,
        7 => Reg8::A,
        _ => unreachable!(),
    }
}
