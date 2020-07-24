use sdl2::render::{Texture, TextureCreator, WindowCanvas};

use crate::arena::arena::Arena;
use crate::engine::assets::image_asset::ImageAsset;
use crate::engine::position::TilePosition;
use crate::engine::sprite::{PositionedSprite, Sprite};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::video::WindowContext;
use std::error::Error;

pub struct FloorView<'a> {
    floor_tiles: &'a Vec<TilePosition>,
    sprites: Vec<PositionedSprite<'a>>,
    texture: Texture<'a>,
    tile_size: u32,
}

impl<'a> FloorView<'a> {
    fn new(
        canvas: &mut WindowCanvas,
        texture_creator: &'a TextureCreator<WindowContext>,
        floor_tiles: &'a Vec<TilePosition>,
        asset: &'a ImageAsset,
        tile_size: u32,
        rows: u32,
        cols: u32,
    ) -> Result<FloorView<'a>, Box<dyn Error>> {
        let sprites = init_sprites(floor_tiles, asset, tile_size);

        let texture = init_texture(canvas, texture_creator, &sprites, tile_size, rows, cols)?;
        Ok(FloorView {
            floor_tiles,
            sprites,
            tile_size,
            texture,
        })
    }

    pub fn from_arena(
        canvas: &mut WindowCanvas,
        texture_creator: &'a TextureCreator<WindowContext>,
        arena: &'a Arena,
        asset: &'a ImageAsset,
        tile_size: u32,
    ) -> Result<FloorView<'a>, Box<dyn Error>> {
        FloorView::new(
            canvas,
            texture_creator,
            &arena.floor_tiles,
            asset,
            tile_size,
            arena.nrows,
            arena.ncols,
        )
    }

    pub fn render_individually(
        &self,
        canvas: &mut WindowCanvas,
        viewport: &Rect,
    ) -> Result<(), String> {
        for sprite in self.sprites.iter() {
            sprite.render(canvas, viewport)?;
        }
        Ok(())
    }

    pub fn render(&self, canvas: &mut WindowCanvas, viewport: &Rect) -> Result<(), String> {
        let src = *viewport;
        canvas.copy(&self.texture, src, None)?;
        Ok(())
    }
}

fn init_texture<'a>(
    canvas: &mut WindowCanvas,
    texture_creator: &'a TextureCreator<WindowContext>,
    sprites: &Vec<PositionedSprite>,
    tile_size: u32,
    rows: u32,
    cols: u32,
) -> Result<Texture<'a>, Box<dyn Error>> {
    let height = rows * tile_size;
    let width = cols * tile_size;

    let viewport = Rect::new(0, 0, width, height);

    let mut tgt_texture =
        texture_creator.create_texture_target(canvas.default_pixel_format(), width, height)?;

    canvas.with_texture_canvas(&mut tgt_texture, |texture| {
        texture.set_draw_color(Color::RGBA(0, 0, 0, 0));
        texture.clear();
        for sprite in sprites.iter() {
            sprite
                .render(texture, &viewport)
                .expect("Failed to render sprite");
        }
    })?;

    Ok(tgt_texture)
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
