pub(super) struct LCDC {
    flag: u8,
}

impl LCDC {
    pub fn new() -> Self {
        Self { flag: 0x91 }
    }

    pub fn get(&self) -> u8 {
        self.flag
    }

    pub fn set(&mut self, v: u8) {
        self.flag = v;
    }

    pub fn is_ppu_enabled(&self) -> bool {
        (self.flag & 0x80) != 0
    }

    pub fn window_tilemap_high(&self) -> bool {
        (self.flag & 0x40) != 0
    }

    pub fn is_window_enabled(&self) -> bool {
        (self.flag & 0x20) != 0
    }

    pub fn is_tile_data_mode1(&self) -> bool {
        (self.flag & 0x10) != 0
    }

    pub fn bg_tilemap_high(&self) -> bool {
        (self.flag & 0x08) != 0
    }

    pub fn sprite_height(&self) -> u8 {
        if (self.flag & 0x04) != 0 { 16 } else { 8 }
    }

    pub fn is_sprites_enabled(&self) -> bool {
        (self.flag & 0x02) != 0
    }

    pub fn is_bg_enabled(&self) -> bool {
        (self.flag & 0x01) != 0
    }
}
