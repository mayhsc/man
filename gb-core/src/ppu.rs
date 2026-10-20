const RESOLUTION_X: usize = 160;
const RESOLUTION_Y: usize = 144;

pub struct Ppu {
    vram: [u8; 0x2000],
    oam: [u8; 0x00A0],
    lcdc: u8,
    lcds: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    dma: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    framebuffer: [u8; (RESOLUTION_X * RESOLUTION_Y * 4) as usize],
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [0; 0x00A0],
            lcdc: 0x91,
            lcds: 0x85,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
            framebuffer: [0; RESOLUTION_X * RESOLUTION_Y * 4],
        }
    }
}

impl Ppu {
    pub fn new() -> Self {
        Ppu::default()
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            0xFF40 => self.lcdc,
            0xFF41 => self.lcds | 0x80,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => 0xFF, // DMA is write-only from the CPU's perspective
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            0xFF40 => self.lcdc = value,
            0xFF41 => {
                self.lcds = (self.lcds & 0x07) | (value & 0x78);
            }
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            0xFF44 => {}
            0xFF45 => self.lyc = value,
            0xFF46 => {
                self.dma = value;
                todo!("Implement DMA transfer from CPU to PPU OAM");
            }
            0xFF47 => self.bgp = value,
            0xFF48 => self.obp0 = value,
            0xFF49 => self.obp1 = value,
            0xFF4A => self.wy = value,
            0xFF4B => self.wx = value,
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        Ppu::fill_test_vram(&mut self.vram);
        self.populate_frame_buffer();
    }

    fn populate_frame_buffer(&mut self) {
        
    }

    fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x < RESOLUTION_X && y < RESOLUTION_Y {
            let index = (y * RESOLUTION_X + x) * 4;
            self.framebuffer[index] = r; // Red
            self.framebuffer[index + 1] = g; // Green
            self.framebuffer[index + 2] = b; // Blue
            self.framebuffer[index + 3] = 0xFF; // Alpha (fully opaque)
        }
    }

    pub fn framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }

    fn fill_test_vram(vram: &mut [u8]) {
        for tile in 0..384 {
            let tile_base = tile * 16;

            for row in 0..8 {
                let pattern = match (tile / 8) % 4 {
                    0 => 0xFF,     // Solid
                    1 => 0xAA,     // Alternating pixels
                    2 => 0xF0,     // Half-width stripes
                    _ => 1 << row, // Diagonal-ish pattern
                };

                vram[tile_base + row * 2] = pattern;
                vram[tile_base + row * 2 + 1] = !pattern;
            }
        }
    }
}
