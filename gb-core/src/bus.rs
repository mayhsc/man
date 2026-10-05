pub trait Bus {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

pub struct FlatBus {
    pub memory: [u8; 0x10000],
}

impl Default for FlatBus {
    fn default() -> Self {
        Self { memory: [0; 65536] }
    }
}

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
        0
    }

    fn write(&mut self, addr: u16, value: u8) {}
}

impl MemoryMap {
    pub fn new() -> Self {
        MemoryMap::default()
    }
}
