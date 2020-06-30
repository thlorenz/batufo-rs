use std::fmt::Debug;
use sdl2::surface::Surface;
use sdl2::rect::{Rect};

#[derive(Debug)]
pub struct ImageAsset {
    pub width: u32,
    pub height: u32,
    pub rows: u32,
    pub cols: u32,
    pub tiles: u32,
    pub item_width: u32,
    pub item_height: u32,
    path: &'static str,
}

#[allow(dead_code)]
impl ImageAsset {
    pub fn new(
        width: u32,
        height: u32,
        rows: u32,
        cols: u32,
        path: &'static str) -> ImageAsset {
        ImageAsset {
            width,
            height,
            rows,
            cols,
            path,
            tiles: (rows * cols),
            item_width: width / cols,
            item_height: height / rows,
        }
    }

    // TODO: support loading other image types than bmp
    // TODO: once we figure out how add surface to the asset itself via mut
    pub fn load(&self) -> Result<Surface<'static>, String> {
        Surface::load_bmp(&self.path)
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
