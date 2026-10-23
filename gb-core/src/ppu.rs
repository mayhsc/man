mod fetcher;
mod flags;
mod sprites;

use fetcher::FetcherState;
use sprites::Sprite;
use std::{collections::VecDeque, vec};

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
    lx: u8,
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
    pixel_fifo: VecDeque<u8>,
    fetcher: FetcherState,
    wly: u8, // Invisible to the CPU

    framebuffer: [u8; (RESOLUTION_X * RESOLUTION_Y) as usize],
    pub drawn: [bool; (RESOLUTION_X * RESOLUTION_Y) as usize], // Debugging which pixels have been drawn
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
            lx: 0,
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
            pixel_fifo: VecDeque::new(),
            fetcher: FetcherState::new(),
            wly: 0,
            framebuffer: [0; RESOLUTION_X * RESOLUTION_Y],
            drawn: [false; RESOLUTION_X * RESOLUTION_Y],
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

        if self.ly == 144 {
            self.set_ppu_mode(PpuMode::VBlank);
        }

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
                // self.oambuffer.clear();
                // self.pixel_fifo.clear();
                self.drawing();
                if self.lx == 160 {
                    self.set_ppu_mode(PpuMode::HBlank);
                    self.ly += 1;
                }
            }
            PpuMode::HBlank => {
                if dot == 456 {
                    self.set_ppu_mode(PpuMode::OamScan);
                    self.lx = 0;
                    self.dot = 0;
                }
            }
            PpuMode::VBlank => {
                if dot == 153 {
                    self.ly = 0;
                    self.set_ppu_mode(PpuMode::OamScan);
                }
            }
        }
        self.dot += 1;
    }

    fn oam_scan(&mut self) {
        let sprite_index = (self.dot / 2) as usize;
        let offset = sprite_index * 4;

        if sprite_index >= 40 {
            return;
        }

        let sprite = Sprite::new(
            self.oam[offset],
            self.oam[offset + 1],
            self.oam[offset + 2],
            self.oam[offset + 3],
        );

        let sprite_height = self.lcdc.sprite_height();

        if self.ly + 16 >= sprite.y
            && self.ly + 16 < sprite.y + sprite_height
            && self.oambuffer.len() < 10
        {
            self.oambuffer.push(sprite);
        }
    }

    fn drawing(&mut self) {
        let tile_id_addr = self.get_tile();
        let pixels = self.get_tile_data(tile_id_addr);
        for pixel in pixels {
            self.pixel_fifo.push_back(pixel);
        }
        if let Some(pixel) = self.pixel_fifo.pop_front() {
            self.render_pixel(pixel);
        }
    }

    fn get_tile(&mut self) -> u16 {
        self.fetcher.set_tilemap_addr(
            self.in_window(),
            self.lcdc.bg_tilemap_high(),
            self.lcdc.window_tilemap_high(),
        );

        self.fetcher
            .get_tile_id_addr(self.is_window_tile(), self.scx, self.scy, self.ly, self.wly)
    }

    fn get_tile_data(&mut self, tile_id_addr: u16) -> [u8; 8] {
        let tile_index = self.vram[(tile_id_addr - 0x8000) as usize];
        let tile_addr = if self.lcdc.is_tile_data_mode1() {
            0x8000i32 + tile_index as i32 * 16
        } else {
            0x9000i32 + (tile_index as i8 as i32) * 16
        } as usize;
        let row_in_tile = if self.is_window_tile() {
            self.wly % 8
        } else {
            (self.ly + self.scy) % 8
        };
        let row_addr = tile_addr + row_in_tile as usize * 2;

        let lo = self.vram[row_addr - 0x8000];
        let hi = self.vram[row_addr + 1 - 0x8000];

        let mut pixels = [0u8; 8];

        for x in 0..8 {
            let bit = 7 - x;
            let low_bit = (lo >> bit) & 1;
            let high_bit = (hi >> bit) & 1;

            pixels[x] = (high_bit << 1) | low_bit;
        }

        pixels
    }

    fn in_window(&self) -> bool {
        (self.lx + 7) >= self.wx
    }

    fn is_window_tile(&self) -> bool {
        return self.lcdc.is_window_enabled() && self.in_window() && self.ly >= self.wy;
    }

    fn render_pixel(&mut self, color_index: u8) {
        let (x, y) = (self.lx as usize, self.ly as usize);
        self.framebuffer[y * 160 + x] = (self.bgp >> (color_index * 2)) & 0b11;
        self.drawn[y * 160 + x] = true;
        self.lx += 1;
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
