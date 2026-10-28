pub(crate) struct Joypad {
    byte: u8,
    pressed: u8,
}

#[derive(Debug)]
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
        Self {
            byte: 0xCF,
            pressed: 0,
        }
    }

    pub fn read(&self) -> u8 {
        let mut result = self.byte | 0xCF;

        if self.byte & 0x10 == 0 {
            result &= !(self.pressed & 0x0F);
        }

        if self.byte & 0x20 == 0 {
            result &= !((self.pressed >> 4) & 0x0F);
        }

        result
    }

    pub fn write(&mut self, value: u8) {
        self.byte = (self.byte & 0xCF) | (value & 0x30)
    }

    pub fn press(&mut self, btn: Button) -> bool {
        let (bit_pos, line) = Joypad::get_pos(btn);

        let old = self.read() & 0x0F;

        self.pressed |= 1 << (bit_pos + if line == 5 { 4 } else { 0 });

        let new = self.read() & 0x0F;

        old & !new != 0
    }

    pub fn release(&mut self, btn: Button) {
        let (bit, line) = Joypad::get_pos(btn);
        self.pressed &= !(1 << (bit + if line == 5 { 4 } else { 0 }));
    }

    fn get_pos(btn: Button) -> (u8, u8) {
        let bit_pos = match btn {
            Button::A | Button::Right => 0,
            Button::B | Button::Left => 1,
            Button::Select | Button::Up => 2,
            Button::Start | Button::Down => 3,
        };
        let line = match btn {
            Button::A | Button::B | Button::Select | Button::Start => 5,
            _ => 4,
        };
        (bit_pos, line)
    }
}
