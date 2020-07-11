use crate::engine::assets::image_asset::ImageAsset;
use crate::engine::position::TilePosition;
use crate::engine::sprite::Sprite;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Walls<'a> {
    wall_rects: Vec<Rect>,
    sprite: Sprite<'a>,
}

impl<'a> Walls<'a> {
    pub fn new(wall_tiles: &Vec<TilePosition>, asset: &'a ImageAsset, tile_size: u32) -> Self {
        let sprite =
            Sprite::new(&asset, tile_size, 0).expect("unable to create walls sprite for idx ");
        let wall_rects = init_rects(wall_tiles, tile_size);
        Walls { wall_rects, sprite }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        for rect in self.wall_rects.iter() {
            self.sprite.render(canvas, rect.clone())?;
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
