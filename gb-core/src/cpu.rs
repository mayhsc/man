use crate::{
    bus::Bus,
    helpers,
    registers::{Operand8, Registers},
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
            // HLT
            0x76 => self.halt(),
            // LD
            0x40..=0x7f => {
                let src = helpers::operand8_from_index(op & 0b111);
                let dst = helpers::operand8_from_index((op >> 3) & 0b111);
                self.ld(dst, src);
            }
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                let dst = helpers::operand8_from_index((op >> 3) & 0b111);
                let byte = self.fetch_byte();
                self.ld(dst, Operand8::D8(byte));
            }
            op if (op & 0b11001111 == 0b00000010) => {
                let v = self.regs.a;
                let addr = self.regs.get16(helpers::reg16_from_index(op >> 4 & 0b11));
                self.bus.write(addr, v);
            }
            op if (op & 0b11001111 == 0b00001010) => {
                let addr = self.regs.get16(helpers::reg16_from_index(op >> 4 & 0b11));
                let v = self.bus.read(addr);
                self.regs.set8(crate::registers::Reg8::A, v);
            }

            _ => panic!("Instruction has not been implemented yet"),
        };
    }

    fn ld(&mut self, dst: Operand8, src: Operand8) {
        let v = match src {
            Operand8::Reg(r) => self.regs.get8(r),
            Operand8::Memory => self.bus.read(self.regs.hl()),
            Operand8::D8(v) => v,
        };

        match dst {
            Operand8::Reg(r) => self.regs.set8(r, v),
            Operand8::Memory => self.bus.write(self.regs.hl(), v),
            Operand8::D8(_) => panic!(),
        };
    }

    fn halt(&self) {}

    #[cfg(test)]
    pub fn set_opcode(&mut self, op: u8) {
        self.opcode = op
    }
}
