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
    framebuffer: [u8; (RESOLUTION_X * RESOLUTION_Y) as usize],
    // Temp
    init: bool,
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
            bgp: 0xE4,
            obp0: 0xFF,
            obp1: 0xFF,
            wy: 0,
            wx: 0,
            framebuffer: [0; RESOLUTION_X * RESOLUTION_Y],
            init: false,
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
        if !self.init {
            Ppu::fill_test_vram(&mut self.vram);
            self.init = true
        }
        self.update_frame_buffer();
        self.scx = self.scx.wrapping_add(1)
    }

    fn update_frame_buffer(&mut self) {
        let data_base: i32 = if is_bit_set(self.lcdc, 4) {
            0x8000
        } else {
            0x9000
        };
        let signed = !is_bit_set(self.lcdc, 4);
        let map_base: usize = if is_bit_set(self.lcdc, 3) {
            0x9C00
        } else {
            0x9800
        };

        for y in 0..144 {
            for x in 0..160 {
                let bx = (x + self.scx as usize) & 0xFF;
                let by = (y + self.scy as usize) & 0xFF;

                let tile_index = self.vram[map_base + (by / 8) * 32 + (bx / 8) - 0x8000];
                let tile_addr = if signed {
                    data_base + (tile_index as i8 as i32) * 16
                } else {
                    data_base + (tile_index as i32) * 16
                } as usize;

                let row = by % 8;
                let lo = self.vram[tile_addr + row * 2 - 0x8000];
                let hi = self.vram[tile_addr + row * 2 + 1 - 0x8000];

                let bit = 7 - (bx % 8);
                let color_index = (((hi >> bit) & 1) << 1) | ((lo >> bit) & 1);
                self.framebuffer[y * 160 + x] = (self.bgp >> (color_index * 2)) & 0b11;
            }
        }
    }

    pub fn framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }

    fn set_tile(vram: &mut [u8], tile: usize, rows: [(u8, u8); 8]) {
        for (r, (lo, hi)) in rows.iter().enumerate() {
            vram[tile * 16 + r * 2] = *lo;
            vram[tile * 16 + r * 2 + 1] = *hi;
        }
    }

    fn fill_test_vram(vram: &mut [u8]) {
        Ppu::set_tile(vram, 0, [(0x00, 0x00); 8]);
        Ppu::set_tile(vram, 1, [(0xFF, 0x00); 8]);
        Ppu::set_tile(vram, 2, [(0x00, 0xFF); 8]);
        Ppu::set_tile(vram, 3, [(0xFF, 0xFF); 8]);

        let mut checker = [(0u8, 0u8); 8];
        let mut stripes = [(0u8, 0u8); 8];
        for r in 0..8 {
            let p = if r % 2 == 0 { 0xAA } else { 0x55 };
            checker[r] = (p, p);
            let d = 0x80u8 >> r;
            stripes[r] = (d, d);
        }
        Ppu::set_tile(vram, 4, checker);
        Ppu::set_tile(vram, 5, stripes);

        let map = &mut vram[0x1800..0x1800 + 32 * 32];
        for y in 0..32usize {
            for x in 0..32usize {
                let on_screen_border = x == 0 || x == 19 || y == 0 || y == 17;
                map[y * 32 + x] = if on_screen_border {
                    3
                } else if y < 9 {
                    match (x - 1) / 5 {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        _ => 3,
                    }
                } else if x < 10 {
                    4
                } else {
                    5
                };
            }
        }
    }
}

fn is_bit_set(number: u8, position: u8) -> bool {
    ((number >> position) & 1) == 1
}
