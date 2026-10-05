pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

#[cfg(test)]
pub struct FlatBus {
    pub memory: [u8; 0x10000],
}

#[cfg(test)]
impl Default for FlatBus {
    fn default() -> Self {
        Self { memory: [0; 65536] }
    }
}

#[cfg(test)]
impl Bus for FlatBus {
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }
}

pub struct MemoryMap {
    rombank00: [u8; 0x4000],
    rombank01_nn: [u8; 0x4000],

    vram: [u8; 0x2000],
    extram: [u8; 0x2000],

    wram0: [u8; 0x1000],
    wram1_7: [[u8; 0x1000]; 7],

    oam: [u8; 0x00A0],
    io_regs: [u8; 0x0080],
    hram: [u8; 0x007F],
    ie: u8,
}

impl Default for MemoryMap {
    fn default() -> Self {
        Self {
            rombank00: [0; 0x4000],
            rombank01_nn: [0; 0x4000],

            vram: [0; 0x2000],
            extram: [0; 0x2000],

            wram0: [0; 0x1000],
            wram1_7: [[0; 0x1000]; 7],

            oam: [0; 0x00A0],
            io_regs: [0; 0x0080],
            hram: [0; 0x007F],
            ie: 0,
        }
    }
}

impl Bus for MemoryMap {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rombank00[addr as usize],
            0x4000..=0x7FFF => self.rombank01_nn[(addr - 0x4000) as usize],
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.extram[(addr - 0xA000) as usize],
            0xC000..=0xCFFF => self.wram0[(addr - 0xC000) as usize],
            0xD000..=0xDFFF => {
                let bank = 0; 
                self.wram1_7[bank][(addr - 0xD000) as usize]
            }
            0xE000..=0xFDFF => {
                self.read(addr - 0x2000)
            }
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFEA0..=0xFEFF => 0xFF, 
            0xFF00..=0xFF7F => self.io_regs[(addr - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.ie,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x3FFF => {}
            0x4000..=0x7FFF => {}
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.extram[(addr - 0xA000) as usize] = value,
            0xC000..=0xCFFF => self.wram0[(addr - 0xC000) as usize] = value,
            0xD000..=0xDFFF => {
                let bank = 0;
                self.wram1_7[bank][(addr - 0xD000) as usize] = value;
            }
            0xE000..=0xFDFF => self.write(addr - 0x2000, value),
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            0xFEA0..=0xFEFF => {} 
            0xFF00..=0xFF7F => self.io_regs[(addr - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            0xFFFF => self.ie = value,
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
}
