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
}
