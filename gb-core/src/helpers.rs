use crate::registers::{Src, Reg8, Dst};

pub fn src_from_index(idx: u8) -> Src {
    match idx {
        0 => Src::Reg(Reg8::B),
        1 => Src::Reg(Reg8::C),
        2 => Src::Reg(Reg8::D),
        3 => Src::Reg(Reg8::E),
        4 => Src::Reg(Reg8::H),
        5 => Src::Reg(Reg8::L),
        6 => Src::Memory,
        7 => Src::Reg(Reg8::A),
        _ => unreachable!(),
    }
}

pub fn dst_from_index(idx: u8) -> Dst {
    match idx {
        0 => Dst::Reg(Reg8::B),
        1 => Dst::Reg(Reg8::C),
        2 => Dst::Reg(Reg8::D),
        3 => Dst::Reg(Reg8::E),
        4 => Dst::Reg(Reg8::H),
        5 => Dst::Reg(Reg8::L),
        6 => Dst::Memory,
        7 => Dst::Reg(Reg8::A),
        _ => unreachable!(),
    }
}
