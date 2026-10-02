use crate::{
    bus::Bus,
    helpers,
    registers::{Dst, Registers, Src},
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
                let src = helpers::src_from_index(op & 0b111);
                let dst = helpers::dst_from_index((op >> 3) & 0b111);
                self.ld(dst, src);
            }
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                let dst = helpers::dst_from_index((op >> 3) & 0b111);
                let byte = self.fetch_byte();
                self.ld(dst, Src::D8(byte));
            }
            _ => panic!("Instruction has not been implemented yet"),
        };
    }

    fn ld(&mut self, dst: Dst, src: Src) {
        let v = match src {
            Src::Reg(r) => self.regs.get(r),
            Src::Memory => self.bus.read(self.regs.hl()),
            Src::D8(v) => v,
        };

        match dst {
            Dst::Reg(r) => self.regs.set(r, v),
            Dst::Memory => self.bus.write(self.regs.hl(), v),
        };
    }

    fn halt(&self) {}

    #[cfg(test)]
    pub fn set_opcode(&mut self, op: u8) {
        self.opcode = op
    }
}
