use crate::{
    bus::Bus,
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
            // Load
            0x7f => self.load_register(Reg8::A, Reg8::A),
            0x78 => self.load_register(Reg8::A, Reg8::B),
            0x79 => self.load_register(Reg8::A, Reg8::C),
            0x7a => self.load_register(Reg8::A, Reg8::D),
            0x7b => self.load_register(Reg8::A, Reg8::E),
            0x7c => self.load_register(Reg8::A, Reg8::H),
            0x7d => self.load_register(Reg8::A, Reg8::L),

            _ => panic!("Instruction has not been implemented yet"),
        };
    }

    fn load_register(&mut self, dst: Reg8, src: Reg8) {
        self.regs.set(dst, self.regs.get(src));
    }

    #[cfg(test)]
    pub fn set_opcode(&mut self, op: u8) {
        self.opcode = op
    }
}
