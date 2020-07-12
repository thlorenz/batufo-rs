use crate::engine::assets::image_asset::ImageAsset;
use sdl2::render::WindowCanvas;

use crate::arena::arena::Arena;
use crate::entities::floor::Floor;
use crate::entities::grid::Grid;
use crate::entities::walls::Walls;
use crate::game_props::{ANTIQUE_WHITE, RENDER_GRID, TILE_SIZE};
use crate::inputs::input::Input;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::error::Error;

pub struct Game<'a> {
    arena: &'a Arena,
    floor: Floor<'a>,
    grid: Grid,
    walls: Walls<'a>,
    camera_platform: Point,
}

impl<'a> Game<'a> {
    pub fn new(
        arena: &'a Arena,
        floor_asset: &'a ImageAsset<'a>,
        wall_asset: &'a ImageAsset<'a>,
    ) -> Result<Self, Box<dyn Error>> {
        let floor = Floor::from_arena(&arena, floor_asset, TILE_SIZE);
        let grid = Grid::new(arena.ncols, arena.nrows, TILE_SIZE);
        let walls = Walls::new(&arena.walls, wall_asset, TILE_SIZE);

        let camera_platform = Point::new(0, 0);
        Ok(Game {
            floor,
            walls,
            arena,
            grid,
            camera_platform,
        })
    }

    pub fn update(&mut self, input: &Input) {
        // TODO: player controller would run here, update his position and then cameras accordingly
        if input.has_up() {
            self.camera_platform = self.camera_platform.offset(0, -1);
        }
        if input.has_down() {
            self.camera_platform = self.camera_platform.offset(0, 1);
        }
        if input.has_left() {
            self.camera_platform = self.camera_platform.offset(-1, 0);
        }
        if input.has_right() {
            self.camera_platform = self.camera_platform.offset(1, 0);
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        canvas.set_draw_color(Color::RGB.call(ANTIQUE_WHITE));
        canvas.clear();
        if RENDER_GRID {
            self.grid.render(canvas)?;
        }
        self.floor.render(canvas, &self.camera_platform)?;
        self.walls.render(canvas, &self.camera_platform)?;
        canvas.present();
        Ok(())
    }
}
