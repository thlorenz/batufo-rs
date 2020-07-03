use std::error::Error;
use std::fmt;

use sdl2::rect::Rect;
use sdl2::surface::Surface;

pub struct ImageAsset {
    pub width: u32,
    pub height: u32,
    pub rows: u32,
    pub cols: u32,
    pub tiles: u32,
    pub item_width: u32,
    pub item_height: u32,
    pub surface: Surface<'static>,
    #[allow(dead_code)]
    path: &'static str,
}

#[allow(dead_code)]
impl ImageAsset {
    pub fn new(
        width: u32,
        height: u32,
        rows: u32,
        cols: u32,
        path: &'static str,
    ) -> Result<Self, Box<dyn Error>> {
        // TODO: support loading other image types than bmp
        let surface = Surface::load_bmp(path)?;

        Ok(ImageAsset {
            width,
            height,
            rows,
            cols,
            path,
            surface,
            tiles: (rows * cols),
            item_width: width / cols,
            item_height: height / rows,
        })
    }

    pub fn rect(&self, row: u32, col: u32) -> Rect {
        let x: u32 = col * self.item_width;
        let y: u32 = row * self.item_height;
        Rect::new(x as i32, y as i32, self.item_width, self.item_height)
    }

    pub fn rect_for_idx(&self, idx: u32) -> Rect {
        let row = idx / self.rows;
        let col = idx % self.rows;
        self.rect(row, col)
    }
}

impl fmt::Debug for ImageAsset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\
ImageAsset {{
   tiles: {},
   rows: {}, cols: {},
   width: {} height: {},
   item_width: {}, item_height: {}
}}",
            self.tiles,
            self.rows,
            self.cols,
            self.width,
            self.height,
            self.item_width,
            self.item_height
        )
    }
}
