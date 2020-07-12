use crate::engine::assets::image_asset::ImageAsset;
use crate::engine::position::TilePosition;
use crate::engine::sprite::Sprite;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

pub struct Walls<'a> {
    rects: Vec<Rect>,
    sprite: Sprite<'a>,
}

impl<'a> Walls<'a> {
    pub fn new(wall_tiles: &Vec<TilePosition>, asset: &'a ImageAsset, tile_size: u32) -> Self {
        let sprite =
            Sprite::new(&asset, tile_size, 0).expect("unable to create walls sprite for idx ");
        let rects = init_rects(wall_tiles, tile_size);
        Walls { rects, sprite }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, camera: &Point) -> Result<(), String> {
        for rect in self.rects.iter() {
            let mut rect = rect.clone();
            rect.offset(camera.x, camera.y);
            self.sprite.render(canvas, rect)?;
        }
        Ok(())
    }
}

fn init_rects(wall_tiles: &Vec<TilePosition>, tile_size: u32) -> Vec<Rect> {
    wall_tiles
        .iter()
        .map(|tp| tp.to_world_rect(tile_size))
        .collect()
}
