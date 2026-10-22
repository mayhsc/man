#[derive(Default)]
pub(super) struct Fetcher {
    current_x: u8,
}

impl Fetcher {
    pub fn new() -> Self {
        Fetcher::default()
    }

    pub fn get_tile(&self) {}
}

pub(super) struct  Tile {
  x: u8,
  y: u8,
}