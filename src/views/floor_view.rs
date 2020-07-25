use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Image;
use ggez::nalgebra as na;

pub struct FloorView {
    sprite_batch: SpriteBatch,
    floor_tiles: Vec<na::Point2<i32>>,
    tile_size: u32,
}

impl FloorView {
    pub fn new(texture: Image, floor_tiles: Vec<na::Point2<i32>>, tile_size: u32) -> FloorView {
        let sprite_batch = SpriteBatch::new(texture);
        FloorView {
            sprite_batch,
            floor_tiles,
            tile_size,
        }
    }
}
