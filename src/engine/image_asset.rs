use ggez::graphics::{Image, Rect};

pub struct ImageAsset {
    pub width: u32,
    pub height: u32,
    pub rows: u32,
    pub cols: u32,
    pub tiles: u32,
    pub item_width: u32,
    pub item_height: u32,
    pub texture: Image,
    width_fraction: f32,
    height_fraction: f32,
}

impl ImageAsset {
    pub fn new(width: u32, height: u32, rows: u32, cols: u32, texture: Image) -> ImageAsset {
        let width_fraction = 1.0 / cols as f32;
        let height_fraction = 1.0 / rows as f32;
        ImageAsset {
            width,
            height,
            rows,
            cols,
            tiles: (rows * cols),
            item_width: width / cols,
            item_height: height / rows,
            texture,
            width_fraction,
            height_fraction,
        }
    }

    pub fn rect(&self, row: u32, col: u32) -> Rect {
        let fx: f32 = col as f32 * self.width_fraction;
        let fy: f32 = row as f32 * self.height_fraction;

        Rect::new(fx, fy, self.width_fraction, self.height_fraction)
    }

    pub fn rect_for_idx(&self, idx: u32) -> Rect {
        let row = idx / self.rows;
        let col = idx % self.rows;
        self.rect(row, col)
    }
}
