use sdl2::render::WindowCanvas;

use crate::arena::arena::Arena;
use crate::engine::assets::image_asset::ImageAsset;
use crate::engine::position::TilePosition;
use crate::engine::sprite::{PositionedSprite, Sprite};
use sdl2::rect::Rect;

pub struct FloorView<'a> {
    floor_tiles: &'a Vec<TilePosition>,
    sprites: Vec<PositionedSprite<'a>>,
    tile_size: u32,
}

impl<'a> FloorView<'a> {
    fn new(floor_tiles: &'a Vec<TilePosition>, asset: &'a ImageAsset, tile_size: u32) -> Self {
        let sprites = init_sprites(floor_tiles, asset, tile_size);
        FloorView {
            floor_tiles,
            sprites,
            tile_size,
        }
    }

    pub fn from_arena(arena: &'a Arena, asset: &'a ImageAsset, tile_size: u32) -> FloorView<'a> {
        FloorView::new(&arena.floor_tiles, asset, tile_size)
    }

    pub fn render(&self, canvas: &mut WindowCanvas, viewport: &Rect) -> Result<(), String> {
        for sprite in self.sprites.iter() {
            sprite.render(canvas, viewport)?;
        }
        Ok(())
    }
}

fn init_sprites<'a>(
    floor_tiles: &'a Vec<TilePosition>,
    asset: &'a ImageAsset,
    tile_size: u32,
) -> Vec<PositionedSprite<'a>> {
    let mut i = 0;
    floor_tiles
        .iter()
        .map(|tp| {
            let center = tp.to_world_point(tile_size);
            let row = i % 7;
            let col = (i / asset.rows) % 7;
            let rect_idx = row * asset.cols + col;
            i = i + 1;
            let sprite = Sprite::new(&asset, tile_size, rect_idx).expect(&format!(
                "unable to create floor sprite for idx {}",
                rect_idx
            ));
            PositionedSprite::new(sprite, center)
        })
        .collect()
}
