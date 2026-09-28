use crate::{bus::Bus, registers::Registers};

pub struct Cpu<B: Bus> {
    registers: Registers,
    bus: B,
}

impl<B: Bus> Cpu<B> {
    pub fn new(bus: B) -> Self {
        Self {
            registers: Registers::default(),
            bus,
        }
    }
}
