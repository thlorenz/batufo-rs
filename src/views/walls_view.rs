use crate::engine::assets::image_asset::ImageAsset;
use crate::engine::position::TilePosition;
use crate::engine::sprite::Sprite;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct WallsView<'a> {
    rects: Vec<Rect>,
    sprite: Sprite<'a>,
}

impl<'a> WallsView<'a> {
    pub fn new(wall_tiles: &Vec<TilePosition>, asset: &'a ImageAsset, tile_size: u32) -> Self {
        let sprite =
            Sprite::new(&asset, tile_size, 0).expect("unable to create walls sprite for idx ");
        let rects = init_rects(wall_tiles, tile_size);
        WallsView { rects, sprite }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, viewport: &Rect) -> Result<(), String> {
        for rect in self.rects.iter() {
            self.sprite.render(canvas, viewport, &rect)?;
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
