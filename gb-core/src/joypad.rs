pub(crate) struct Joypad {
    byte: u8,
}

pub enum Button {
    A,
    B,
    Start,
    Select,
    Up,
    Down,
    Left,
    Right,
}

impl Joypad {
    pub fn new() -> Self {
        Self { byte: 0xCF }
    }

    pub fn read(&self) -> u8 {
        self.byte
    }

    pub fn write(&mut self, value: u8) {
        self.byte = (value & 0x30) | (self.byte & 0xCF);
    }

    pub fn press(&mut self, btn: Button) {}

    pub fn release(&mut self, btn: Button) {}
}
