use std::collections::VecDeque;

#[derive(Clone, Copy, Default)]
pub(super) enum FetcherStep {
    #[default]
    GetTile,
    GetTileDataLow,
    GetTileDataHigh,
    Sleep,
    Push,
}

#[derive(Default)]
pub(super) struct FetcherState {
    tilemap_base_addr: u16,
    fetcher_x: u8,

    step: FetcherStep,
    step_dots: u8,

    tile_id: u8,
    tile_data_low: u8,
    tile_data_high: u8,
}

impl FetcherState {
    pub fn new() -> Self {
        FetcherState::default()
    }

    pub fn current_step(&self) -> FetcherStep {
        self.step
    }

    pub fn advance_step(&mut self) {
        self.step = match self.step {
            FetcherStep::GetTile => FetcherStep::GetTileDataLow,
            FetcherStep::GetTileDataLow => FetcherStep::GetTileDataHigh,
            FetcherStep::GetTileDataHigh => FetcherStep::Sleep,
            FetcherStep::Sleep => FetcherStep::Push,
            FetcherStep::Push => FetcherStep::GetTile,
        };
    }

    pub fn set_tile_id(&mut self, tile_id: u8) {
        self.tile_id = tile_id
    }

    pub fn set_tile_data_low(&mut self, value: u8) {
        self.tile_data_low = value
    }

    pub fn set_tile_data_high(&mut self, value: u8) {
        self.tile_data_high = value
    }

    pub fn tile_data_addr(
        &mut self,
        is_tile_data_mode1: bool,
        is_window_tile: bool,
        wly: u8,
        ly: u8,
        scy: u8,
    ) -> usize {
        let tile_addr = if is_tile_data_mode1 {
            0x8000i32 + self.tile_id as i32 * 16
        } else {
            0x9000i32 + (self.tile_id as i8 as i32) * 16
        } as usize;
        let row_in_tile = if is_window_tile {
            wly % 8
        } else {
            ly.wrapping_add(scy) % 8
        };
        tile_addr + row_in_tile as usize * 2
    }

    pub fn push_pixels(&mut self, fifo: &mut VecDeque<u8>) {
        if fifo.len() <= 8 {
            for x in 0..8 {
                let bit = 7 - x;
                let low_bit = (self.tile_data_low >> bit) & 1;
                let high_bit = (self.tile_data_high >> bit) & 1;

                fifo.push_back((high_bit << 1) | low_bit);
            }
            self.fetcher_x = self.fetcher_x.wrapping_add(1) & 0x1F;
            self.advance_step();
            self.step_dots = 0;
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

    pub fn get_tile_id_addr(
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
