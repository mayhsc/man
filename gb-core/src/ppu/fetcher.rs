#[derive(Default)]
pub(super) struct FetcherState {
    tilemap_base_addr: u16,
    fetcher_x: u8,
}

impl FetcherState {
    pub fn new() -> Self {
        Self {
            fetcher_x: 0,
            tilemap_base_addr: 0x9800,
        }
    }

    pub fn set_tilemap_addr(
        &mut self,
        in_window: bool,
        bg_tilemap_high: bool,
        window_tilemap_high: bool,
    ) {
        self.tilemap_base_addr =
            if bg_tilemap_high && !in_window || window_tilemap_high && in_window {
                0x9C00
            } else {
                0x9800
            };
    }

    pub fn get_tile_addr(
        &self,
        is_window_tile: bool,
        scx: u8,
        scy: u8,
        ly: u8,
        window_line: u8,
    ) -> u16 {
        let (x, pixel_y) = if is_window_tile {
            (self.fetcher_x & 0x1F, window_line)
        } else {
            (
                ((scx / 8).wrapping_add(self.fetcher_x)) & 0x1F,
                ly.wrapping_add(scy),
            )
        };

        let y = pixel_y / 8;

        self.tilemap_base_addr + (y as u16 * 32) + x as u16
    }
}
