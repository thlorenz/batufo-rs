#[derive(Debug, PartialEq)]
pub struct TilePosition {
    pub col: u32,
    pub row: u32,
    pub rel_x: f32,
    pub rel_y: f32,
}

impl TilePosition {
    pub fn new(col: u32, row: u32, rel_x: f32, rel_y: f32) -> TilePosition {
        TilePosition {
            col,
            row,
            rel_x,
            rel_y,
        }
    }

    pub fn centered(col: u32, row: u32, tile_size: f32) -> TilePosition {
        let rel_x = tile_size / 2.0;
        let rel_y = tile_size / 2.0;
        TilePosition::new(col, row, rel_x, rel_y)
    }
}
