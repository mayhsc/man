pub struct Timer {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
    overflow_cycles_left: Option<i8>,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            overflow_cycles_left: None,
        }
    }
}

impl Timer {
    pub fn new() -> Self {
        Timer::default()
    }

    pub fn tick(&mut self, t_cycles: u32) -> bool {
        let mut request_interrupt = false;
        for _ in 0..t_cycles {
            let old = signal(self.div, self.tac);
            self.div = self.div.wrapping_add(1);

            match self.overflow_cycles_left {
                Some(v) => {
                    if v == 1 {
                        self.tima = self.tma;
                        request_interrupt = true;
                        self.overflow_cycles_left = None;
                    } else {
                        self.overflow_cycles_left = Some(v - 1);
                    }
                }
                None => {}
            }

            if old && !signal(self.div, self.tac) {
                let (r, overflow) = self.tima.overflowing_add(1);
                self.tima = r;
                if overflow {
                    self.overflow_cycles_left = Some(4)
                }
            }
        }
        request_interrupt
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

fn signal(div: u16, tac: u8) -> bool {
    let bit = match tac & 3 {
        0 => 9,
        1 => 3,
        2 => 5,
        _ => 7,
    };
    (tac & 0b100 != 0) && (div >> bit) & 1 == 1
}
