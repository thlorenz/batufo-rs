use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use crate::arena::arena::Arena;
use crate::engine::assets::image_asset::ImageAsset;
use crate::engine::position::TilePosition;
use crate::engine::sprite::{PositionedSprite, Sprite};

struct FloorRender<'a> {
    floor_tiles: &'a Vec<TilePosition>,
    sprites: Vec<PositionedSprite>,
    tile_size: u32,
}

impl FloorRender<'_> {
    fn new(
        floor_tiles: &'static Vec<TilePosition>,
        ncols: u32,
        nrows: u32,
        texture_creator: &'static TextureCreator<WindowContext>,
        asset: &ImageAsset,
        tile_size: u32,
    ) -> Self {
        let sprites: Vec<PositionedSprite> =
            init_sprites(floor_tiles, texture_creator, asset, ncols, nrows, tile_size);
        FloorRender {
            floor_tiles,
            sprites,
            tile_size,
        }
    }

    pub fn from_arena(
        arena: &'static Arena,
        texture_creator: &'static TextureCreator<WindowContext>,
        asset: &ImageAsset,
        tile_size: u32,
    ) -> FloorRender<'static> {
        FloorRender::new(
            &arena.floor_tiles,
            arena.ncols,
            arena.nrows,
            texture_creator,
            asset,
            tile_size,
        )
    }

    pub fn render(&self) {}
}

fn init_sprites(
    floor_tiles: &'static Vec<TilePosition>,
    texture_creator: &'static TextureCreator<WindowContext>,
    asset: &ImageAsset,
    ncols: u32,
    nrows: u32,
    tile_size: u32,
) -> Vec<PositionedSprite> {
    let mut i = 0;
    floor_tiles
        .iter()
        .map(|tp| {
            let center = tp.to_world_point(tile_size);
            let row = i % 7;
            let col = (i / nrows) % 7;
            let rect_idx = row * ncols + col;
            i = i + 1;
            let sprite = Sprite::new(&texture_creator, &asset, rect_idx).expect(&format!(
                "unable to create floor sprite for idx {}",
                rect_idx
            ));
            PositionedSprite::new(sprite, center)
        })
        .collect()
}
