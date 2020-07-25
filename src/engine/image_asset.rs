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
}

impl ImageAsset {
    pub fn new(width: u32, height: u32, rows: u32, cols: u32, texture: Image) -> ImageAsset {
        ImageAsset {
            width,
            height,
            rows,
            cols,
            tiles: (rows * cols),
            item_width: width / cols,
            item_height: height / rows,
            texture,
        }
    }

    pub fn rect(&self, row: u32, col: u32) -> Rect {
        let x: u32 = col * self.item_width;
        let y: u32 = row * self.item_height;
        Rect::new(
            x as f32,
            y as f32,
            self.item_width as f32,
            self.item_height as f32,
        )
    }

    pub fn rect_for_idx(&self, idx: u32) -> Rect {
        let row = idx / self.rows;
        let col = idx % self.rows;
        self.rect(row, col)
    }
}
