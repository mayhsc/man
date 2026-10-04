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

pub struct MemoryMap {}

impl Default for MemoryMap {
    fn default() -> Self {
        Self {}
    }
}

impl Bus for MemoryMap {
    fn read(&self, addr: u16) -> u8 {
        0
    }

    fn write(&mut self, addr: u16, value: u8) {}
}

impl MemoryMap {
    pub fn new() -> Self{
        MemoryMap::default()
    }
}