use crate::engine::assets::image_asset::ImageAsset;
use sdl2::render::WindowCanvas;

use crate::arena::arena::Arena;
use crate::data::diagnostics::{Diagnostic, Diagnostics};
use crate::game_props::{ANTIQUE_WHITE, RENDER_GRID, TILE_SIZE};
use crate::inputs::input::Input;
use crate::views::diag_hud::DiagHud;
use crate::views::floor_view::FloorView;
use crate::views::grid_view::GridView;
use crate::views::text::Text;
use crate::views::walls_view::WallsView;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::error::Error;

pub struct Game<'a> {
    // renderers
    arena: &'a Arena,
    floor: FloorView<'a>,
    grid: GridView,
    walls: WallsView<'a>,
    diag_hud: DiagHud<'a>,

    // models
    diagnostics: Diagnostics,

    // cameras
    camera_platform: Point,
}

impl<'a> Game<'a> {
    pub fn new(
        arena: &'a Arena,
        floor_asset: &'a ImageAsset<'a>,
        wall_asset: &'a ImageAsset<'a>,
        stats_text: Text<'a>,
    ) -> Result<Self, Box<dyn Error>> {
        let floor = FloorView::from_arena(&arena, floor_asset, TILE_SIZE);
        let grid = GridView::new(arena.ncols, arena.nrows, TILE_SIZE);
        let walls = WallsView::new(&arena.walls, wall_asset, TILE_SIZE);
        let diag_hud = DiagHud::new(Point::new(0, 0), stats_text);

        let camera_platform = Point::new(0, 0);
        Ok(Game {
            arena,
            floor,
            grid,
            walls,
            diag_hud,
            diagnostics: Diagnostics::new(),
            camera_platform,
        })
    }

    pub fn update(&mut self, dt: u32, input: &Input, diagnostics: Diagnostic) {
        // TODO: player controller would run here, update his position and then cameras accordingly
        let len = dt as i32;
        if input.has_up() {
            self.camera_platform = self.camera_platform.offset(0, -len);
        }
        if input.has_down() {
            self.camera_platform = self.camera_platform.offset(0, len);
        }
        if input.has_left() {
            self.camera_platform = self.camera_platform.offset(-len, 0);
        }
        if input.has_right() {
            self.camera_platform = self.camera_platform.offset(len, 0);
        }
        self.diagnostics.update(diagnostics);
    }

    pub fn render(&self, canvas: &mut WindowCanvas) -> Result<(), Box<dyn Error>> {
        canvas.set_draw_color(Color::RGB.call(ANTIQUE_WHITE));
        canvas.clear();
        if RENDER_GRID {
            self.grid.render(canvas)?;
        }
        self.floor.render(canvas, &self.camera_platform)?;
        self.walls.render(canvas, &self.camera_platform)?;
        self.diag_hud.render(canvas, &self.diagnostics.current())?;
        canvas.present();
        Ok(())
    }
}
