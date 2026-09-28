use crate::{bus::Bus, registers::Registers};

pub struct Cpu<B: Bus> {
    pub(crate) regs: Registers,
    pub(crate) bus: B,
}

impl<B: Bus> Cpu<B> {
    pub fn new(bus: B) -> Self {
        Self {
            regs: Registers::default(),
            bus,
        }
    }

    pub fn step(&mut self) {
        let op = self.regs.pc;
        let bytes_consumed = self.execute(op);
        self.regs.increment(bytes_consumed);
    }

    fn execute(&self, op: u16) -> u16 {
        match op {
            _ => 0,
        }
    }
}
