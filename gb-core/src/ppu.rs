mod sprites;
mod fetcher;
mod flags;

use sprites::Sprite;
use fetcher::Fetcher;
use std::vec;

use crate::ppu::flags::LCDC;

const RESOLUTION_X: usize = 160;
const RESOLUTION_Y: usize = 144;

enum PpuMode {
    OamScan,
    Drawing,
    HBlank,
    VBlank,
}

pub struct Ppu {
    vram: [u8; 0x2000],
    oam: [u8; 0x00A0],

    lcdc: flags::LCDC,
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

    dot: u16,
    mode: PpuMode,

    oambuffer: Vec<Sprite>,
    fetcher: Fetcher,

    framebuffer: [u8; (RESOLUTION_X * RESOLUTION_Y) as usize],
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            vram: [0; 0x2000],
            oam: [0; 0x00A0],
            lcdc: LCDC::new(),
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
            dot: 0,
            mode: PpuMode::OamScan,
            oambuffer: vec![],
            fetcher: Fetcher::new(),
            framebuffer: [0; RESOLUTION_X * RESOLUTION_Y],
        }
    }
}

impl Ppu {
    pub fn new() -> Self {
        Ppu::default()
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => match self.mode {
                PpuMode::Drawing => 0xFF,
                _ => self.vram[(addr - 0x8000) as usize],
            },
            0xFE00..=0xFE9F => match self.mode {
                PpuMode::OamScan | PpuMode::Drawing => 0xFF,
                _ => self.oam[(addr - 0xFE00) as usize],
            },
            0xFF40 => self.lcdc.get(),
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
            0x8000..=0x9FFF => match self.mode {
                PpuMode::Drawing => {}
                _ => self.vram[(addr - 0x8000) as usize] = value,
            },
            0xFE00..=0xFE9F => match self.mode {
                PpuMode::OamScan | PpuMode::Drawing => {}
                _ => self.oam[(addr - 0xFE00) as usize] = value,
            },
            0xFF40 => self.lcdc.set(value),
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
        let dot = self.dot;

        match self.mode {
            PpuMode::OamScan => {
                if dot % 2 == 0 {
                    self.oam_scan();
                }
                if dot == 80 {
                    self.set_ppu_mode(PpuMode::Drawing);
                }
            }
            PpuMode::Drawing => {
                self.drawing();
            }
            PpuMode::HBlank => {}
            PpuMode::VBlank => {}
        }
        self.update_frame_buffer();
        self.dot += 1;
    }

    fn oam_scan(&mut self) {
        let idx = (self.dot / 2) as usize;
        let sprite = Sprite::new(
            self.oam[idx + 1],
            self.oam[idx],
            self.oam[idx + 2],
            self.oam[idx + 3],
        );
        let sprite_height = self.lcdc.sprite_height();

        if (self.ly + 16) >= sprite.y
            && (self.ly + 16) < (sprite.y + sprite_height)
            && self.oambuffer.len() < 10
        {
            self.oambuffer.push(sprite);
        }
    }

    fn drawing(&mut self) {}
    fn update_frame_buffer(&mut self) {
        let data_base: i32 = if self.lcdc.is_tile_data_mode1() {
            0x8000
        } else {
            0x9000
        };
        let signed = !self.lcdc.is_tile_data_mode1();
        let map_base: usize = if self.lcdc.bg_tilemap_high() {
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

    fn set_ppu_mode(&mut self, mode: PpuMode) {
        self.mode = mode;

        self.lcds &= !0b11;

        self.lcds |= match self.mode {
            PpuMode::HBlank => 0b00,
            PpuMode::VBlank => 0b01,
            PpuMode::OamScan => 0b10,
            PpuMode::Drawing => 0b11,
        };
    }
}

