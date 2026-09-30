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
            // LD A
            0x7f => self.load_register(Reg8::A, Reg8::A),
            0x78 => self.load_register(Reg8::A, Reg8::B),
            0x79 => self.load_register(Reg8::A, Reg8::C),
            0x7a => self.load_register(Reg8::A, Reg8::D),
            0x7b => self.load_register(Reg8::A, Reg8::E),
            0x7c => self.load_register(Reg8::A, Reg8::H),
            0x7d => self.load_register(Reg8::A, Reg8::L),
            // LD B
            0x47 => self.load_register(Reg8::B, Reg8::A),
            0x40 => self.load_register(Reg8::B, Reg8::B),
            0x41 => self.load_register(Reg8::B, Reg8::C),
            0x42 => self.load_register(Reg8::B, Reg8::D),
            0x43 => self.load_register(Reg8::B, Reg8::E),
            0x44 => self.load_register(Reg8::B, Reg8::H),
            0x45 => self.load_register(Reg8::B, Reg8::L),
            // LD C
            0x4f => self.load_register(Reg8::C, Reg8::A),
            0x48 => self.load_register(Reg8::C, Reg8::B),
            0x49 => self.load_register(Reg8::C, Reg8::C),
            0x4a => self.load_register(Reg8::C, Reg8::D),
            0x4b => self.load_register(Reg8::C, Reg8::E),
            0x4c => self.load_register(Reg8::C, Reg8::H),
            0x4d => self.load_register(Reg8::C, Reg8::L),
            // LD D
            0x57 => self.load_register(Reg8::D, Reg8::A),
            0x50 => self.load_register(Reg8::D, Reg8::B),
            0x51 => self.load_register(Reg8::D, Reg8::C),
            0x52 => self.load_register(Reg8::D, Reg8::D),
            0x53 => self.load_register(Reg8::D, Reg8::E),
            0x54 => self.load_register(Reg8::D, Reg8::H),
            0x55 => self.load_register(Reg8::D, Reg8::L),
            // LD E
            0x5f => self.load_register(Reg8::E, Reg8::A),
            0x58 => self.load_register(Reg8::E, Reg8::B),
            0x59 => self.load_register(Reg8::E, Reg8::C),
            0x5a => self.load_register(Reg8::E, Reg8::D),
            0x5b => self.load_register(Reg8::E, Reg8::E),
            0x5c => self.load_register(Reg8::E, Reg8::H),
            0x5d => self.load_register(Reg8::E, Reg8::L),
            // LD H
            0x67 => self.load_register(Reg8::H, Reg8::A),
            0x60 => self.load_register(Reg8::H, Reg8::B),
            0x61 => self.load_register(Reg8::H, Reg8::C),
            0x62 => self.load_register(Reg8::H, Reg8::D),
            0x63 => self.load_register(Reg8::H, Reg8::E),
            0x64 => self.load_register(Reg8::H, Reg8::H),
            0x65 => self.load_register(Reg8::H, Reg8::L),
            // LD L
            0x6f => self.load_register(Reg8::L, Reg8::A),
            0x68 => self.load_register(Reg8::L, Reg8::B),
            0x69 => self.load_register(Reg8::L, Reg8::C),
            0x6a => self.load_register(Reg8::L, Reg8::D),
            0x6b => self.load_register(Reg8::L, Reg8::E),
            0x6c => self.load_register(Reg8::L, Reg8::H),
            0x6d => self.load_register(Reg8::L, Reg8::L),

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
