use crate::engine::assets::image_asset::ImageAsset;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

use crate::arena::arena::Arena;
use crate::entities::floor_render::FloorRender;
use crate::game_props::TILE_SIZE;
use std::error::Error;

pub struct Game<'a> {
    arena: &'a Arena,
    floor: FloorRender<'a>,
}

impl<'a> Game<'a> {
    pub fn new(
        arena: &'a Arena,
        texture_creator: &'a TextureCreator<WindowContext>,
        floor_asset: &'a ImageAsset<'a>,
    ) -> Result<Self, Box<dyn Error>> {
        let floor = FloorRender::from_arena(&arena, texture_creator, floor_asset, TILE_SIZE);

        Ok(Game { floor, arena })
    }
}
