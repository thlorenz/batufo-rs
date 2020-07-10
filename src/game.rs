// use sdl2::render::TextureCreator;
// use sdl2::video::WindowContext;
// use crate::engine::assets::image_asset::ImageAsset;

use crate::arena::arena::Arena;
// use crate::entities::floor_render::FloorRender;
// use crate::game_props::TILE_SIZE;
use std::error::Error;

pub struct Game {
    arena: Arena,
    // floor: FloorRender<'a>,
}

impl Game {
    pub fn new(
        arena: Arena,
        //  texture_creator: &'static TextureCreator<WindowContext>,
        //  floor_asset: &ImageAsset,
    ) -> Result<Self, Box<dyn Error>> {
        // let floor = FloorRender::from_arena(&arena, texture_creator, floor_asset, TILE_SIZE);

        Ok(Game { arena })
    }
}
