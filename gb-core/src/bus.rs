use crate::{IF_ADDR, ppu::Ppu, timer::Timer};

pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);

    fn peek(&self, addr: u16) -> u8 {
        self.read(addr)
    }

    fn tick_timers(&mut self, t_cycles: u32) {
        let _ = t_cycles;
    }
}

pub struct MemoryMap {
    rombank00: [u8; 0x4000],
    rombank01_nn: [u8; 0x4000],

    extram: [u8; 0x2000],

    wram0: [u8; 0x1000],
    wram1_7: [[u8; 0x1000]; 7],

    io_regs: [u8; 0x0080],
    hram: [u8; 0x007F],
    ie: u8,

    timer: Timer,
    ppu: Ppu,
}

impl Default for MemoryMap {
    fn default() -> Self {
        Self {
            rombank00: [0; 0x4000],
            rombank01_nn: [0; 0x4000],

            extram: [0; 0x2000],

            wram0: [0; 0x1000],
            wram1_7: [[0; 0x1000]; 7],

            io_regs: [0; 0x0080],
            hram: [0; 0x007F],
            ie: 0,

            timer: Timer::new(),
            ppu: Ppu::new(),
        }
    }
}

impl Bus for MemoryMap {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rombank00[addr as usize],
            0x4000..=0x7FFF => self.rombank01_nn[(addr - 0x4000) as usize],
            0xA000..=0xBFFF => self.extram[(addr - 0xA000) as usize],
            0xC000..=0xCFFF => self.wram0[(addr - 0xC000) as usize],
            0xD000..=0xDFFF => {
                let bank = 0;
                self.wram1_7[bank][(addr - 0xD000) as usize]
            }
            0xE000..=0xFDFF => self.read(addr - 0x2000),
            0xFEA0..=0xFEFF => 0xFF,
            0xFF04..=0xFF07 => self.timer.get_timer(addr),
            0x8000..=0x9FFF | 0xFE00..=0xFE9F | 0xFF40..=0xFF4B => self.ppu.read(addr),
            // // Temporary
            0xFF00 => 0xCF,
            0xFF00..=0xFF7F => self.io_regs[(addr - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x3FFF => {}
            0x4000..=0x7FFF => {}
            0xA000..=0xBFFF => self.extram[(addr - 0xA000) as usize] = value,
            0xC000..=0xCFFF => self.wram0[(addr - 0xC000) as usize] = value,
            0xD000..=0xDFFF => {
                let bank = 0;
                self.wram1_7[bank][(addr - 0xD000) as usize] = value;
            }
            0xE000..=0xFDFF => self.write(addr - 0x2000, value),
            0xFEA0..=0xFEFF => {}
            0xFF04..=0xFF07 => self.timer.set_timer(addr, value),
            0xFF46 => {}
            0x8000..=0x9FFF | 0xFE00..=0xFE9F | 0xFF40..=0xFF4B => self.ppu.write(addr, value),
            0xFF02 => {
                if (value & 0x80) != 0 {
                    self.io_regs[0x02] = value & !0x80;

                    self.io_regs[0x01] = 0xFF;

                    let current_if = self.read(IF_ADDR);
                    self.write(IF_ADDR, current_if | (1 << 3));
                } else {
                    self.io_regs[0x02] = value;
                }
            }
            0xFF00..=0xFF7F => self.io_regs[(addr - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            0xFFFF => self.ie = value,
        }
    }

    fn tick_timers(&mut self, t_cycles: u32) {
        for _ in 0..t_cycles {
            if self.timer.tick() {
                let _if = self.read(IF_ADDR) | 1 << 2;
                self.write(IF_ADDR, _if);
            }
            let interrupts = self.ppu.tick();
            if interrupts != 0 {
                let current_if = self.read(IF_ADDR);
                self.write(IF_ADDR, current_if | interrupts);
            }
        }
    }
}

impl MemoryMap {
    pub fn new() -> Self {
        MemoryMap::default()
    }

    pub fn load(&mut self, data: &[u8]) {
        let bank00_len = data.len().min(0x4000);
        self.rombank00[..bank00_len].copy_from_slice(&data[..bank00_len]);

        if data.len() > 0x4000 {
            let bank01_len = (data.len() - 0x4000).min(0x4000);
            self.rombank01_nn[..bank01_len].copy_from_slice(&data[0x4000..0x4000 + bank01_len]);
        }

        if data.len() > 0x8000 {
            todo!();
        }
    }

    pub fn framebuffer(&self) -> &[u8] {
        &self.ppu.framebuffer()
    }

    pub fn drawn(&self) -> &[bool] {
        &self.ppu.drawn
    }
}
