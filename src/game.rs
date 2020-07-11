use crate::engine::assets::image_asset::ImageAsset;
use sdl2::render::WindowCanvas;

use crate::arena::arena::Arena;
use crate::entities::floor::Floor;
use crate::entities::grid::Grid;
use crate::game_props::{ANTIQUE_WHITE, RENDER_GRID, TILE_SIZE};
use sdl2::pixels::Color;
use std::error::Error;

pub struct Game<'a> {
    arena: &'a Arena,
    floor: Floor<'a>,
    grid: Grid,
}

impl<'a> Game<'a> {
    pub fn new(arena: &'a Arena, floor_asset: &'a ImageAsset<'a>) -> Result<Self, Box<dyn Error>> {
        let floor = Floor::from_arena(&arena, floor_asset, TILE_SIZE);
        let grid = Grid::new(arena.ncols, arena.nrows, TILE_SIZE);

        Ok(Game { floor, arena, grid })
    }

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB.call(ANTIQUE_WHITE));
        canvas.clear();
        if RENDER_GRID {
            self.grid.render(canvas)?;
        }
        // self.floor.render(canvas)?;
        canvas.present();
        Ok(())
    }
}
