#[derive(Clone, Copy)]
pub(super) struct Sprite {
    pub x: u8,
    pub y: u8,
    pub tile_idx: u8,
    pub flags: u8,
}

impl Sprite {
    pub fn new(x: u8, y: u8, tile_idx: u8, flags: u8) -> Self {
        Self {
            x,
            y,
            tile_idx,
            flags,
        }
    }

    pub fn x_flip(&self) -> bool {
        self.flags & 0x20 != 0
    }

    pub fn y_flip(&self) -> bool {
        self.flags & 0x40 != 0
    }

    pub fn priority(&self) -> u8 {
        self.flags & 0x80
    }

    pub fn palette(&self) -> u8 {
        self.flags & 0x08
    }
}

#[derive(Clone, Copy)]
pub struct OamPixel {
    pub color_idx: u8,
    pub palette: u8,
    pub priority: u8,
}
