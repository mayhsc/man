use crate::{
    bus::Bus,
    flags::Condition,
    helpers,
    registers::{Operand8, Registers},
};

pub struct Cpu<B: Bus> {
    pub(crate) regs: Registers,
    pub(crate) bus: B,
    opcode: u8,
    ime: bool,
}

impl<B: Bus> Cpu<B> {
    pub fn new(bus: B) -> Self {
        Self {
            regs: Registers::default(),
            bus,
            opcode: 0,
            ime: false,
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

    fn fetch_byte16(&mut self) -> u16 {
        (self.fetch_byte() as u16) | ((self.fetch_byte() as u16) << 8)
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
                let addr = self
                    .regs
                    .get16mem(helpers::reg16mem_from_index(op >> 4 & 0b11));
                self.bus.write(addr, v);
            }
            op if (op & 0b11001111 == 0b00001010) => {
                let addr = self
                    .regs
                    .get16mem(helpers::reg16mem_from_index(op >> 4 & 0b11));
                let v = self.bus.read(addr);
                self.regs.set8(crate::registers::Reg8::A, v);
            }
            0x80..=0x9F => {
                let v = self.read_operand8_index(op & 0b111);
                let carry = ((op >> 3) & 0b1) == 1;
                let sub = ((op >> 4) & 0b1) == 1;

                if sub {
                    self.regs.sub_a(v, carry);
                } else {
                    self.regs.add_a(v, carry);
                }
            }
            0xC6 | 0xCE | 0xD6 | 0xDE => {
                let byte = self.fetch_byte();
                let carry = ((op >> 3) & 0b1) == 1;
                let sub = ((op >> 4) & 0b1) == 1;

                if sub {
                    self.regs.sub_a(byte, carry);
                } else {
                    self.regs.add_a(byte, carry);
                }
            }
            0xA0..=0xA7 => {
                let v = self.read_operand8_index(op & 0b111);
                self.regs.and(v);
            }
            0xA8..=0xAF => {
                let v = self.read_operand8_index(op & 0b111);
                self.regs.xor(v);
            }
            0xB0..=0xB7 => {
                let v = self.read_operand8_index(op & 0b111);
                self.regs.or(v);
            }
            0xB8..=0xBF => {
                let v = self.read_operand8_index(op & 0b111);
                self.regs.cp(v);
            }
            0xE6 | 0xEE | 0xf6 | 0xfe => {
                let v = self.fetch_byte();
                let a = (op >> 3) & 0b00000111;
                if a == 0b100 {
                    self.regs.and(v);
                } else if a == 0b101 {
                    self.regs.xor(v);
                } else if a == 0b110 {
                    self.regs.or(v);
                } else {
                    self.regs.cp(v);
                }
            }
            op if (op & 0b11001111 == 0b00000001) => {
                let r16 = helpers::reg16_from_index((op >> 4) & 0b11);
                let v = self.fetch_byte16();
                self.regs.set16(r16, v);
            }
            0x08 => {
                let addr = self.fetch_byte16();
                let v = self.regs.sp;
                self.bus.write(addr, v as u8);
                self.bus.write(addr + 1, (v >> 8) as u8);
            }
            op if (op & 0b11001111 == 0b00000011) => {
                let r = helpers::reg16_from_index((op >> 4) & 0b11);
                let v = self.regs.get16(&r) + 1;
                self.regs.set16(r, v);
            }
            op if (op & 0b11001111 == 0b00001011) => {
                let r = helpers::reg16_from_index((op >> 4) & 0b11);
                let v = self.regs.get16(&r) - 1;
                self.regs.set16(r, v);
            }
            op if (op & 0b11001111 == 0b00001001) => {
                let r = helpers::reg16_from_index((op >> 4) & 0b11);
                let v = self.regs.get16(&r);
                self.regs.add_hl(v);
            }
            op if (op & 0b11000111 == 0b00000100) => {
                let dst = helpers::operand8_from_index((op >> 3) & 0b111);
                let v = self.read_operand8_index((op >> 3) & 0b111);
                self.inc(dst, v);
            }
            op if (op & 0b11000111 == 0b00000101) => {
                let dst = helpers::operand8_from_index((op >> 3) & 0b111);
                let v = self.read_operand8_index((op >> 3) & 0b111);
                self.dec(dst, v);
            }
            0x07 => self.regs.rlca(),
            0x0F => self.regs.rrca(),
            0x17 => self.regs.rla(),
            0x1F => self.regs.rra(),
            0x27 => self.regs.daa(),
            0x2F => self.regs.cpl(),
            0x37 => self.regs.scf(),
            0x3F => self.regs.ccf(),
            0x18 => {
                let offset = self.fetch_byte() as i8;
                self.regs.pc = self.regs.pc.wrapping_add_signed(offset as i16);
            }
            op if (op & 0b11100111 == 0b00100000) => {
                let offset = self.fetch_byte() as i8;
                let c = helpers::cond_from_index((op >> 3) & 0b11);
                self.regs.jr(offset, c);
            }
            op if (op & 0b11100111 == 0b11000000) => {
                let c = helpers::cond_from_index((op >> 3) & 0b11);
                if self.regs.get_condition(c) {
                    self.ret();
                }
            }
            0xC9 => {
                self.ret();
            }
            0xD9 => {
                self.ret();
                self.ime = true;
            }
            op if (op & 0b11100111 == 0b11000010) => {
                let c = helpers::cond_from_index((op >> 3) & 0b11);
                self.jmp(Some(c));
            }
            0xC3 => {
                self.jmp(None);
            }
            0xE9 => self.regs.pc = self.regs.hl(),

            _ => panic!("Instruction has not been implemented yet"),
        };
    }

    fn ld(&mut self, dst: Operand8, src: Operand8) {
        let v = match src {
            Operand8::Reg(r) => self.regs.get8(r),
            Operand8::MemHL => self.bus.read(self.regs.hl()),
            Operand8::D8(v) => v,
        };

        match dst {
            Operand8::Reg(r) => self.regs.set8(r, v),
            Operand8::MemHL => self.bus.write(self.regs.hl(), v),
            Operand8::D8(_) => panic!(),
        };
    }

    fn inc(&mut self, dst: Operand8, v: u8) {
        let (result, _) = v.overflowing_add(1);
        let half_carry = (v & 0xF) + (1 & 0xF) > 0xF;

        match dst {
            Operand8::Reg(r) => self.regs.set8(r, result),
            Operand8::MemHL => self.bus.write(self.regs.hl(), result),
            Operand8::D8(_) => panic!(),
        }

        self.regs.f.set_z(result == 0);
        self.regs.f.set_n(false);
        self.regs.f.set_h(half_carry);
    }

    fn dec(&mut self, dst: Operand8, v: u8) {
        let (result, _) = v.overflowing_sub(1);
        let half_carry = (v & 0xF) < (1 & 0xF);

        match dst {
            Operand8::Reg(r) => self.regs.set8(r, result),
            Operand8::MemHL => self.bus.write(self.regs.hl(), result),
            Operand8::D8(_) => panic!(),
        }

        self.regs.f.set_z(result == 0);
        self.regs.f.set_n(true);
        self.regs.f.set_h(half_carry);
    }

    fn ret(&mut self) {
        let lower_byte = self.bus.read(self.regs.pop());
        let higher_byte = self.bus.read(self.regs.pop());
        self.regs.pc = ((higher_byte as u16) << 8) | (lower_byte as u16);
    }

    fn jmp(&mut self, condition: Option<Condition>) {
        let addr = self.fetch_byte16();

        if condition.map_or(true, |cond| self.regs.get_condition(cond)) {
            self.regs.pc = addr;
        }
    }

    fn read_operand8_index(&mut self, idx: u8) -> u8 {
        match helpers::operand8_from_index(idx) {
            Operand8::Reg(r) => self.regs.get8(r),
            Operand8::MemHL => self.bus.read(self.regs.hl()),
            Operand8::D8(v) => v,
        }
    }

    fn halt(&self) {}

    #[cfg(test)]
    pub fn set_opcode(&mut self, op: u8) {
        self.opcode = op
    }
}
