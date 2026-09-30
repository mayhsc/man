use crate::{
    bus::Bus,
    helpers,
    registers::{Reg8, Registers},
};

pub struct Cpu<B: Bus> {
    pub(crate) regs: Registers,
    pub(crate) bus: B,
    opcode: u8,
}

impl<B: Bus> Cpu<B> {
    pub fn new(bus: B) -> Self {
        Self {
            regs: Registers::default(),
            bus,
            opcode: 0,
        }
    }

    pub fn step(&mut self) {
        self.execute(self.opcode);
        self.opcode = self.fetch_byte();
    }

    fn fetch_byte(&mut self) -> u8 {
        let b = self.bus.read(self.regs.pc);
        self.regs.increment();
        b
    }

    fn execute(&mut self, op: u8) {
        match op {
            // NOP
            0x00 => {}
            // LD A
            0x76 => self.halt(),
            0x40..=0x7f => {
                let src = helpers::reg8_from_index(op & 0b111);
                let dst = helpers::reg8_from_index((op >> 3) & 0b111);
                self.load_register(dst, src);
            }

            _ => panic!("Instruction has not been implemented yet"),
        };
    }

    fn load_register(&mut self, dst: Reg8, src: Reg8) {
        self.regs.set(dst, self.regs.get(src));
    }

    fn halt(&self) {}

    #[cfg(test)]
    pub fn set_opcode(&mut self, op: u8) {
        self.opcode = op
    }
}
