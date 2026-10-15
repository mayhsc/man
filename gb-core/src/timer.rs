#[derive(Default)]
pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn new() -> Self {
        Timer::default()
    }

    pub fn tick(&mut self, t_cycles: u32) {
        self.div += t_cycles as u16
    }

    pub fn set_timer(&mut self, addr: u16, v: u8) {
        match addr {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = v,
            0xFF06 => self.tma = v,
            0xFF07 => self.tac = v,
            _ => panic!(),
        };
    }

    pub fn get_timer(&self, addr: u16) -> u8 {
        match addr {
            0xFF04 => (self.div >> 8) as u8,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac,
            _ => panic!(),
        }
    }
}
