pub(crate) struct Joypad {
    byte: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self { byte: 0xCF }
    }

    pub fn read(&self) -> u8 {
        self.byte
    }

    pub fn write(&mut self, value: u8) {
        self.byte = value;
    }
}
